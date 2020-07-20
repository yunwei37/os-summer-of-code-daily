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