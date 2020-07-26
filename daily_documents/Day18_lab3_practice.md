# 实验三：虚实地址转换

1. 原理：在 os/src/entry.asm 中，boot_page_table 的意义是什么？当执行 jal rust_main 时，不考虑缓存，硬件通过哪些地址找到了 rust_main 的第一条指令？

- 低地址的恒等映射：保证程序替换页表后的短暂时间内，pc 仍然可以顺着低地址去执行内存中的指令。
- 高地址：运行 main 函数

对应代码：

```
    # 初始内核映射所用的页表
    .section .data
    .align 12
boot_page_table:
    .quad 0
    .quad 0
    # 第 2 项：0x8000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1
    .quad (0x80000 << 10) | 0xcf
    .zero 507 * 8
    # 第 510 项：0xffff_ffff_8000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1
    .quad (0x80000 << 10) | 0xcf
    .quad 0
```

- 执行 jal rust_main 时，硬件首先需要加载 rust_main 对应的地址，大概是 0xffff_ffff_802x_xxxx。
- 然后，硬件先从 satp 高位置读取内存映射模式，再从 satp 低位置读取根页表页号，再访问根页表；
- 对于 rust_main 而言，一级页号是其 [30:38] 位，即 510。硬件此时定位到根页表的第 510 项
- 这一项的标志为 XWR，说明它指向一个大页而不是指向下一级页表；这一项的 V 位为 1，说明目标在内存中。
- 因此，硬件寻址到页基址 + 页内偏移
- 找到 rust_main

这一部分完全由硬件完成。

2. 为什么 Mapping 中的 page_tables 和 mapped_pairs 都保存了一些 FrameTracker？二者有何不同？

代码：

```rs
/// 某个线程的内存映射关系
pub struct Mapping {
    /// 保存所有使用到的页表
    page_tables: Vec<PageTableTracker>,
    /// 根页表的物理页号
    root_ppn: PhysicalPageNumber,
    /// 所有分配的物理页面映射信息
    mapped_pairs: VecDeque<(VirtualPageNumber, FrameTracker)>,
}
```
- page_tables 存放了所有页表所用到的页面，而 mapped_pairs 则存放了进程所用到的页面。 
  

3. 分析：假设某进程需要虚拟地址 A 到物理地址 B 的映射，这需要操作系统来完成。那么操作系统在建立映射时有没有访问 B？如果有，它是怎么在还没有映射的情况下访问 B 的呢？

- 建立映射不一定需要访问 B， 但可能会在建立映射的同时向页面中加载一些数据：
- 此时通过线性偏移量来访问
- 通过代码可以很清晰地看到这一点

建立映射的代码：

```rs
/// 加入一段映射，可能会相应地分配物理页面
    ///
    /// 未被分配物理页面的虚拟页号暂时不会写入页表当中，它们会在发生 PageFault 后再建立页表项。
    pub fn map(&mut self, segment: &Segment, init_data: Option<&[u8]>) -> MemoryResult<()> {
        match segment.map_type {
            // 线性映射，直接对虚拟地址进行转换
            MapType::Linear => {
                for vpn in segment.page_range().iter() {
                    self.map_one(vpn, Some(vpn.into()), segment.flags | Flags::VALID)?;
                }
                // 拷贝数据
                if let Some(data) = init_data {
                    unsafe {
                        (&mut *slice_from_raw_parts_mut(segment.range.start.deref(), data.len()))
                            .copy_from_slice(data);
                    }
                }
            }
            // 需要分配帧进行映射
            MapType::Framed => {
                for vpn in segment.page_range().iter() {
                    // 如果有初始化数据，找到相应的数据
                    let page_data = if init_data.is_none() || init_data.unwrap().is_empty() {
                        [0u8; PAGE_SIZE]
                    } else {
                        // 这里必须进行一些调整，因为传入的数据可能并非按照整页对齐

                        // 传入的初始化数据
                        let init_data = init_data.unwrap();
                        // 整理后将要返回的一整个页面的数据
                        let mut page_data = [0u8; PAGE_SIZE];

                        // 拷贝时必须考虑区间与整页不对齐的情况
                        //    start（仅第一页时非零）
                        //      |        stop（仅最后一页时非零）
                        // 0    |---data---|          4096
                        // |------------page------------|
                        let page_address = VirtualAddress::from(vpn);
                        let start = if segment.range.start > page_address {
                            segment.range.start - page_address
                        } else {
                            0
                        };
                        let stop = min(PAGE_SIZE, segment.range.end - page_address);
                        // 计算来源和目标区间并进行拷贝
                        let dst_slice = &mut page_data[start..stop];
                        let src_slice = &init_data[(page_address + start - segment.range.start)
                            ..(page_address + stop - segment.range.start)];
                        dst_slice.copy_from_slice(src_slice);

                        page_data
                    };

                    // 建立映射
                    let mut frame = FRAME_ALLOCATOR.lock().alloc()?;
                    // 更新页表
                    self.map_one(vpn, Some(frame.page_number()), segment.flags)?;
                    // 写入数据
                    (*frame).copy_from_slice(&page_data);
                    // 保存
                    self.mapped_pairs.push_back((vpn, frame));
                }
            }
        }
        Ok(())
    }

```

4. 实验：了解并实现时钟页面置换算法（或任何你感兴趣的算法），可以自行设计样例来比较性能（等等

先看看这个页面置换是怎么做的：

os/src/interrupt/handler.rs

```rs
/// 处理缺页异常
///
/// todo: 理论上这里需要判断访问类型，并与页表中的标志位进行比对
fn page_fault(context: &mut Context, scause: Scause, stval: usize) -> *mut Context {
    static mut COUNT: usize = 0;
    println!("page_fault {}", unsafe {
        COUNT += 1;
        COUNT
    });
    let current_thread = PROCESSOR.lock().current_thread();
    let memory_set = &mut current_thread.process.inner().memory_set;

    match memory_set.mapping.handle_page_fault(stval) {
        Ok(_) => {
            memory_set.activate();
            context
        }
        Err(msg) => fault(msg, scause, stval),
    }
}


```

os/src/memory/mapping/mapping.rs

```rs

/// 处理缺页异常
    pub fn handle_page_fault(&mut self, stval: usize) -> MemoryResult<()> {
        // 发生异常的访问页面
        let vpn = VirtualPageNumber::floor(stval.into());
        // 该页面如果在进程的内存中，则应该在 Swap 中，将其取出
        let swap_tracker: SwapTracker = self
            .swapped_pages
            .remove(&vpn)
            .ok_or("stval page is not mapped")?;
        // 读取该页面的数据
        let page_data = swap_tracker.read();

        if self.mapped_pairs.full() {
            // 取出一个映射
            let (popped_vpn, mut popped_frame) = self.mapped_pairs.pop().unwrap();
            // 交换数据
            swap_tracker.write(&*popped_frame);
            (*popped_frame).copy_from_slice(&page_data);
            // 修改页表映射
            self.invalidate_one(popped_vpn)?;
            let entry = self.remap_one(vpn, popped_frame.page_number())?;
            // 更新记录
            self.mapped_pairs.push(vpn, popped_frame, entry);
            self.swapped_pages.insert(popped_vpn, swap_tracker);
        } else {
            // 添加新的映射
            let mut frame = FRAME_ALLOCATOR.lock().alloc()?;
            // 复制数据
            (*frame).copy_from_slice(&page_data);
            // 更新映射
            let entry = self.remap_one(vpn, frame.page_number())?;
            // 更新记录
            self.mapped_pairs.push(vpn, frame, entry);
        }
        Ok(())
    }

```

通过查找，可以看到最多使用的物理页面数量设置是这样的：

```rs
/// 内核进程最多使用的物理页面数量
pub const KERNEL_PROCESS_FRAME_QUOTA: usize = 16;
/// 用户进程最多使用的物理页面数量
pub const USER_PROCESS_FRAME_QUOTA: usize = 16;
```

这里实现的时钟页面置换算法：

```rs

pub struct ClockSwapper {
    /// 记录映射和添加的顺序
    queue: Vec<(VirtualPageNumber, FrameTracker, usize)>,
    /// 映射数量上限
    quota: usize,
}


impl Swapper for ClockSwapper {

    fn new(quota: usize) -> Self {
        Self {
            queue: Vec::new(),
            quota,
        }
    }
    fn full(&self) -> bool {
        self.queue.len() == self.quota
    }
    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)> {
        let len = self.queue.len()-1;
        let mut index:usize = 0;
        let mut isfound:bool = false;
        unsafe{
            loop{
                for i in 0..(len+1){
                    let pe = self.queue[i].2.clone() as *mut PageTableEntry;
                    //println!("{:?}",(*pe));
                    let mut flags = (*pe).flags().clone();
                    if flags.contains(Flags::ACCESSED) {
                        flags.set(Flags::ACCESSED,false);
                        //println!("{:?}",flags);
                        (*pe).set_flags(flags);
                    } else {
                        //println!("{:?}",flags);
                        index = i;
                        isfound = true;
                        break;
                    }
                }
                if isfound {
                    break;
                }
            }
        }
        println!("{:?}",index);
        let f = self.queue.remove(index);
        Some((f.0,f.1))
    }
    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, entry: *mut PageTableEntry) {
        self.queue.push((vpn, frame, entry as usize));
    }
    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool) {
        self.queue.retain(|(vpn, _, _)| predicate(vpn));
    }
}



```

测试：

中间打印的部分log:

```rs
0,PageTableEntry { value: 539831495, page_number: PhysicalPageNumber(527179), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
1,PageTableEntry { value: 539844743, page_number: PhysicalPageNumber(527192), flags: VALID | READABLE | WRITABLE | DIRTY }
page_fault 200
0,PageTableEntry { value: 539831495, page_number: PhysicalPageNumber(527179), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
1,PageTableEntry { value: 539830471, page_number: PhysicalPageNumber(527178), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
2,PageTableEntry { value: 539827399, page_number: PhysicalPageNumber(527175), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
3,PageTableEntry { value: 539832519, page_number: PhysicalPageNumber(527180), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
4,PageTableEntry { value: 539833543, page_number: PhysicalPageNumber(527181), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
5,PageTableEntry { value: 539834567, page_number: PhysicalPageNumber(527182), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
6,PageTableEntry { value: 539835591, page_number: PhysicalPageNumber(527183), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
7,PageTableEntry { value: 539836615, page_number: PhysicalPageNumber(527184), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
8,PageTableEntry { value: 539837639, page_number: PhysicalPageNumber(527185), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
9,PageTableEntry { value: 539838663, page_number: PhysicalPageNumber(527186), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
10,PageTableEntry { value: 539839687, page_number: PhysicalPageNumber(527187), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
11,PageTableEntry { value: 539840711, page_number: PhysicalPageNumber(527188), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
12,PageTableEntry { value: 539841735, page_number: PhysicalPageNumber(527189), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
13,PageTableEntry { value: 539842759, page_number: PhysicalPageNumber(527190), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
14,PageTableEntry { value: 539843783, page_number: PhysicalPageNumber(527191), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
15,PageTableEntry { value: 539844807, page_number: PhysicalPageNumber(527192), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
0,PageTableEntry { value: 539831431, page_number: PhysicalPageNumber(527179), flags: VALID | READABLE | WRITABLE | DIRTY }
page_fault 201
0,PageTableEntry { value: 539830407, page_number: PhysicalPageNumber(527178), flags: VALID | READABLE | WRITABLE | DIRTY }
test passed

```