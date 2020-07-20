# lab3 学习报告

lab3 和 lab2 联系紧密，是其后续部分，在 lab2 中涉及通过页的方式对物理内存进行管理：

在 lab3 中主要涉及：

- 虚拟地址和物理地址的概念和关系
- 利用页表完成虚拟地址到物理地址的映射
- 实现内核的重映射 

## 从虚拟内存到物理内存

### 原理

在现代的操作系统中，为了让其他的程序能方便的运行在操作系统上，需要完成的一个很重要的抽象是「每个程序有自己的地址空间，且地址空间范围是一样的」，这将会减少了上层程序的大量麻烦，否则程序本身要维护自己需要的物理内存，这也会导致极大程度的不安全。

这个执行上看到的地址空间，就是虚拟内存。而访问虚拟内存的地址就是`虚拟地址（Virtual Address）`，与之对应的是`物理地址（Physical Address）`。

这样的设计会导致上层的应用程序可能会访问同一个值相等的虚拟地址，所以操作系统需要做的就是替这些程序维护这个虚拟地址到物理地址的映射。甚者，为了统一和连贯，内核自己本身访问内存也将会通过虚拟地址。

### Sv39

选择 RISC-V 本身硬件支持的 Sv39 模式作为页表的实现：

- 物理地址有 56 位
- 虚拟地址有 64 位，只有低 39 位有效。 63-39 位的值必须等于第 38 位的值。
- 物理页号为 44 位，每个物理页大小为 4KB
- 虚拟页号为 27 位，每个虚拟页大小也为 4KB
- 物理地址和虚拟地址的最后 12 位都表示页内偏移

### 页表项

`页表项（PTE，Page Table Entry）`是用来描述一个虚拟页号如何映射到物理页号的：

Sv39 模式里面的一个页表项大小为 64 位（即 8 字节）。其中第 53-10 共 44 位为一个物理页号，表示这个虚拟页号映射到的物理页号。后面的第 9-0 位则描述页的相关状态信息。

- V 表示这个页表项是否合法。如果为 0 表示不合法，此时页表项其他位的值都会被忽略。
- R,W,X 分别表示是否可读（Readable）、可写（Writable）和可执行（Executable）。
- 如果 R,W,X 均为 0，文档上说这表示这个页表项指向下一级页表。
- U 为 1 表示用户态运行的程序可以通过该页表项完成地址映射。需要将 S 态的状态寄存器 sstatus 上的 SUM (permit Supervisor User Memory access) 位手动设置为 1 才可以访问通过这些 U 为 1 的页表项进行映射的用户态内存空间。

### 多级页表

在 Sv39 模式中我们采用三级页表

### 页表基址

- 页表寄存器 satp：页表的基址（起始地址）一般会保存在一个特殊的寄存器中。

### 快表（TLB）

使用`快表（TLB, Translation Lookaside Buffer）`来作为虚拟页号到物理页号的映射的缓存。

- 需要使用 sfence.vma 指令刷新 TLB

## 修改内核

首先需要把内核的运行环境从物理地址空间转移到虚拟地址空间：将内核代码放在虚拟地址空间中以 0xffffffff80200000 开头的一段高地址空间中。

这是一种临时的线性映射：

os/src/linker.ld：

需要将起始地址修改为虚拟地址，增加 4K 对齐：

```rs
/* 目标架构 */
OUTPUT_ARCH(riscv)

/* 执行入口 */
ENTRY(_start)

/* 数据存放起始地址 */
BASE_ADDRESS = 0xffffffff80200000;

SECTIONS
{   
    /* . 表示当前地址（location counter） */
    . = BASE_ADDRESS;

    /* start 符号表示全部的开始位置 */
    kernel_start = .;

    . = ALIGN(4K);
    text_start = .;

    /* .text 字段 */
    .text : {
        /* 把 entry 函数放在最前面 */
        *(.text.entry)
        /* 要链接的文件的 .text 字段集中放在这里 */
        *(.text .text.*)
    }

    . = ALIGN(4K);
    rodata_start = .;

    /* .rodata 字段 */
    .rodata : {
        /* 要链接的文件的 .rodata 字段集中放在这里 */
        *(.rodata .rodata.*)
    }

    . = ALIGN(4K);
    data_start = .;

    /* .data 字段 */
    .data : {
        /* 要链接的文件的 .data 字段集中放在这里 */
        *(.data .data.*)
    }

    . = ALIGN(4K);
    bss_start = .;

    /* .bss 字段 */
    .bss : {
        /* 要链接的文件的 .bss 字段集中放在这里 */
        *(.sbss .bss .bss.*)
    }

    /* 结束地址 */
    . = ALIGN(4K);
    kernel_end = .;
}

```

修改 os/src/memory/config.rs 中的 KERNEL_END_ADDRESS 修改为虚拟地址并加入偏移量：

```rs
lazy_static! {
    /// 内核代码结束的地址，即可以用来分配的内存起始地址
    /// 
    /// 因为 Rust 语言限制，我们只能将其作为一个运行时求值的 static 变量，而不能作为 const
    pub static ref KERNEL_END_ADDRESS: VirtualAddress = VirtualAddress(kernel_end as usize); 
}

/// 内核使用线性映射的偏移量
pub const KERNEL_MAP_OFFSET: usize = 0xffff_ffff_0000_0000;

```

需要加入两个关于位操作的辅助 crate：

os/Cargo.toml：

```rs
bitflags = "1.2.1"
bit_field = "0.10.0"

```

os/src/memory/address.rs

对虚拟地址和虚拟页号这两个类进行了封装，同时也支持了一些诸如 VirtualAddress::from(PhysicalAddress) 的转换 trait（即一些加减偏移量等操作）：

（略过）

在启动时、在进入 rust_main 之前我们要完成一个从物理地址访存模式到虚拟访存模式的转换：我们要写一个简单的页表，完成这个线性映射：

os/src/entry.asm

```rs
    ......
_start:
    # 计算 boot_page_table 的物理页号
    lui t0, %hi(boot_page_table)
    li t1, 0xffffffff00000000
    sub t0, t0, t1
    srli t0, t0, 12
    # 8 << 60 是 satp 中使用 Sv39 模式的记号
    li t1, (8 << 60)
    or t0, t0, t1
    # 写入 satp 并更新 TLB
    csrw satp, t0
    sfence.vma

    # 加载栈地址
    lui sp, %hi(boot_stack_top)
    addi sp, sp, %lo(boot_stack_top)
    # 跳转至 rust_main
    lui t0, %hi(rust_main)
    addi t0, t0, %lo(rust_main)
    jr t0
    .....

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

上面的代码完成了：

- 把 CPU 的访问模式改为 Sv39，这里需要做的就是把一个页表的物理页号和 Sv39 模式写入 satp 寄存器，然后刷新 TLB。
- 先使用一种最简单的页表构造方法：将一个三级页表项的标志位 R,W,X 不设为全 0，可以将它变为表示 1GB 的一个大页。
- 有一个从 0x80000000 到 0x80000000 的映射，在跳转到 rust_main 之前（即 jr t0）之前，PC 的值都还是 0x802xxxxx 这样的地址，即使是写入了 satp 寄存器，但是 PC 的地址不会变。为了执行这段中间的尴尬的代码，我们在页表里面也需要加入这段代码的地址的映射。（过渡使用

还需要记得修改一下 allocator.rs:

```rs
lazy_static! {
    /// 帧分配器
    pub static ref FRAME_ALLOCATOR: Mutex<FrameAllocator<AllocatorImpl>> = Mutex::new(FrameAllocator::new(Range::from(
            PhysicalPageNumber::ceil(PhysicalAddress::from(*KERNEL_END_ADDRESS))..PhysicalPageNumber::floor(MEMORY_END_ADDRESS),
        )
    ));
}

```

make clean
make run

即可运行。

## 实现页表

首先构建了通过虚拟页号获得三级 VPN 的辅助函数： 

os/src/memory/address.rs：

```rs
impl VirtualPageNumber {
    /// 得到一、二、三级页号
    pub fn levels(self) -> [usize; 3] {
        [
            self.0.get_bits(18..27),
            self.0.get_bits(9..18),
            self.0.get_bits(0..9),
        ]
    }
}
```

### 页表项

页表项，其实就是对一个 usize（8 字节）的封装，同时我们可以用刚刚加入的 bit 级别操作的 crate 对其实现一些取出特定段的方便后续实现的函数：

新建一个 mapping 文件夹：

os/src/memory/mapping/page_table_entry.rs

对于页表项的一些功能函数：

```rs
use crate::memory::address::*;
use bit_field::BitField;
use bitflags::*;

/// Sv39 结构的页表项
#[derive(Copy, Clone, Default)]
pub struct PageTableEntry(usize);

/// Sv39 页表项中标志位的位置
const FLAG_RANGE: core::ops::Range<usize> = 0..8;
/// Sv39 页表项中物理页号的位置
const PAGE_NUMBER_RANGE: core::ops::Range<usize> = 10..54;

impl PageTableEntry {
    /// 将相应页号和标志写入一个页表项
    pub fn new(page_number: PhysicalPageNumber, flags: Flags) -> Self {
        Self(
            *0usize
                .set_bits(..8, flags.bits() as usize)
                .set_bits(10..54, page_number.into()),
        )
    }
    /// 清除
    pub fn clear(&mut self) {
        self.0 = 0;
    }
    /// 设置物理页号，同时根据 ppn 是否为 Some 来设置 Valid 位
    pub fn update_page_number(&mut self, ppn: Option<PhysicalPageNumber>) {
        if let Some(ppn) = ppn {
            self.0
                .set_bits(FLAG_RANGE, (self.flags() | Flags::VALID).bits() as usize)
                .set_bits(PAGE_NUMBER_RANGE, ppn.into());
        } else {
            self.0
                .set_bits(FLAG_RANGE, (self.flags() - Flags::VALID).bits() as usize)
                .set_bits(PAGE_NUMBER_RANGE, 0);
        }
    }
    /// 获取页号
    pub fn page_number(&self) -> PhysicalPageNumber {
        PhysicalPageNumber::from(self.0.get_bits(10..54))
    }
    /// 获取地址
    pub fn address(&self) -> PhysicalAddress {
        PhysicalAddress::from(self.page_number())
    }
    /// 获取标志位
    pub fn flags(&self) -> Flags {
        unsafe { Flags::from_bits_unchecked(self.0.get_bits(..8) as u8) }
    }
    /// 是否为空（可能非空也非 Valid）
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
    /// 是否指向下一级（RWX 全为0）
    pub fn has_next_level(&self) -> bool {
        let flags = self.flags();
        !(flags.contains(Flags::READABLE)
            || flags.contains(Flags::WRITABLE)
            || flags.contains(Flags::EXECUTABLE))
    }
}

impl core::fmt::Debug for PageTableEntry {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter
            .debug_struct("PageTableEntry")
            .field("value", &self.0)
            .field("page_number", &self.page_number())
            .field("flags", &self.flags())
            .finish()
    }
}

bitflags! {
    /// 页表项中的 8 个标志位
    #[derive(Default)]
    pub struct Flags: u8 {
        /// 有效位
        const VALID =       1 << 0;
        /// 可读位
        const READABLE =    1 << 1;
        /// 可写位
        const WRITABLE =    1 << 2;
        /// 可执行位
        const EXECUTABLE =  1 << 3;
        /// 用户位
        const USER =        1 << 4;
        /// 全局位，我们不会使用
        const GLOBAL =      1 << 5;
        /// 已使用位，用于替换算法
        const ACCESSED =    1 << 6;
        /// 已修改位，用于替换算法
        const DIRTY =       1 << 7;
    }
}

macro_rules! implement_flags {
    ($field: ident, $name: ident, $quote: literal) => {
        impl Flags {
            #[doc = "返回 `Flags::"]
            #[doc = $quote]
            #[doc = "` 或 `Flags::empty()`"]
            pub fn $name(value: bool) -> Flags {
                if value {
                    Flags::$field
                } else {
                    Flags::empty()
                }
            }
        }
    };
}

implement_flags! {USER, user, "USER"}
implement_flags! {READABLE, readable, "READABLE"}
implement_flags! {WRITABLE, writable, "WRITABLE"}
implement_flags! {EXECUTABLE, executable, "EXECUTABLE"}
```

## 页表

os/src/memory/mapping/page_table.rs

```rs
//! 单一页表页面（4K） [`PageTable`]，以及相应封装 [`FrameTracker`] 的 [`PageTableTracker`]
use super::page_table_entry::PageTableEntry;
use crate::memory::{address::*, config::PAGE_SIZE, frame::FrameTracker};
/// 存有 512 个页表项的页表
#[repr(C)]
pub struct PageTable {
    pub entries: [PageTableEntry; PAGE_SIZE / 8],
}

impl PageTable {
    /// 将页表清零
    pub fn zero_init(&mut self) {
        self.entries = [Default::default(); PAGE_SIZE / 8];
    }
}

/// 类似于 [`FrameTracker`]，用于记录某一个内存中页表
pub struct PageTableTracker(pub FrameTracker);

impl PageTableTracker {
    /// 将一个分配的帧清零，形成空的页表
    pub fn new(frame: FrameTracker) -> Self {
        let mut page_table = Self(frame);
        page_table.zero_init();
        page_table
    }
    /// 获取物理页号
    pub fn page_number(&self) -> PhysicalPageNumber {
        self.0.page_number()
    }
}

// PageTableEntry 和 PageTableTracker 都可以 deref 到对应的 PageTable

impl core::ops::Deref for PageTableTracker {
    type Target = PageTable;
    fn deref(&self) -> &Self::Target {
        self.0.address().deref_kernel()
    }
}

impl core::ops::DerefMut for PageTableTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.address().deref_kernel()
    }
}

impl PageTableEntry {
    pub fn get_next_table(&self) -> &'static mut PageTable {
        self.address().deref_kernel()
    }
}

```

## 实现内核重映射

由于各个段之间的访问权限是不同的，因此，我们考虑对这些段分别进行重映射，使得他们的访问权限被正确设置。

这个需求可以抽象为一段内存（可能是很多个虚拟页）通过一个方式映射到很多个物理页上，同时这个内存段将会有一个统一的属性和进一步高层次的管理。

### 内存段 Segment

内存段是一篇连续的虚拟页范围，其中的每一页通过线性映射（直接偏移到一个物理页）或者分配（其中的每个虚拟页调用物理页分配器分配一个物理页）。

- 内核使用线性映射
- 用户空间使用按帧分配映射

os/src/memory/mapping/segment.rs

用 enum 和 struct 来封装内存段映射的类型和内存段本身：

后面，上层需要做的是把一个 Segment 中没有建立物理页映射关系的全部虚拟页，都申请到物理页并建立映射关系（或者说线性映射没有这样的虚拟页，而分配映射需要把每个虚拟页都申请一个对应的物理页）；因此可以实现这样一个需要具体分配的迭代器。

```rs
//! 映射类型 [`MapType`] 和映射片段 [`Segment`]

use crate::memory::{address::*, mapping::Flags, range::Range};

/// 映射的类型
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MapType {
    /// 线性映射，操作系统使用
    Linear,
    /// 按帧分配映射
    Framed,
}

/// 一个映射片段（对应旧 tutorial 的 `MemoryArea`）
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Segment {
    /// 映射类型
    pub map_type: MapType,
    /// 所映射的虚拟地址
    pub range: Range<VirtualAddress>,
    /// 权限标志
    pub flags: Flags,
}

impl Segment {
    /// 遍历对应的物理地址（如果可能）
    pub fn iter_mapped(&self) -> Option<impl Iterator<Item = PhysicalPageNumber>> {
        match self.map_type {
            // 线性映射可以直接将虚拟地址转换
            MapType::Linear => Some(self.page_range().into().iter()),
            // 按帧映射无法直接获得物理地址，需要分配
            MapType::Framed => None,
        }
    }

    /// 将地址相应地上下取整，获得虚拟页号区间
    pub fn page_range(&self) -> Range<VirtualPageNumber> {
        Range::from(
            VirtualPageNumber::floor(self.range.start)..VirtualPageNumber::ceil(self.range.end),
        )
    }
}

```

### Mapping

对页表、内存段进行组合和封装，借助其中对页表的操作实现对内存段的映射：

```rs
#[derive(Default)]
/// 某个线程的内存映射关系
pub struct Mapping {
    /// 保存所有使用到的页表
    page_tables: Vec<PageTableTracker>,
    /// 根页表的物理页号
    root_ppn: PhysicalPageNumber,
    /// 所有分配的物理页面映射信息
    mapped_pairs: VecDeque<(VirtualPageNumber, FrameTracker)>,
}

impl Mapping {
    /// 创建一个有根节点的映射
    pub fn new() -> MemoryResult<Mapping> {
        let root_table = PageTableTracker::new(FRAME_ALLOCATOR.lock().alloc()?);
        let root_ppn = root_table.page_number();
        Ok(Mapping {
            page_tables: vec![root_table],
            root_ppn,
            mapped_pairs: VecDeque::new(),
        })
    }
}
```

实现对页表的查找，并利用该函数实现：
- 对虚拟页号到物理页号的映射：
- 对一个连续的 Segment 的映射：
- 页表的激活



```rs

impl Mapping {
    /// 将当前的映射加载到 `satp` 寄存器并记录
    pub fn activate(&self) {
        // satp 低 27 位为页号，高 4 位为模式，8 表示 Sv39
        let new_satp = self.root_ppn.0 | (8 << 60);
        unsafe {
            // 将 new_satp 的值写到 satp 寄存器
            llvm_asm!("csrw satp, $0" :: "r"(new_satp) :: "volatile");
            // 刷新 TLB
            llvm_asm!("sfence.vma" :::: "volatile");
        }
    }

    /// 创建一个有根节点的映射
    pub fn new() -> MemoryResult<Mapping> {
        let root_table = PageTableTracker::new(FRAME_ALLOCATOR.lock().alloc()?);
        let root_ppn = root_table.page_number();
        Ok(Mapping {
            page_tables: vec![root_table],
            root_ppn,
            mapped_pairs: VecDeque::new(),
        })
    }

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

    /// 移除一段映射
    pub fn unmap(&mut self, segment: &Segment) {
        for vpn in segment.page_range().iter() {
            let entry = self.find_entry(vpn).unwrap();
            assert!(!entry.is_empty());
            // 从页表中清除项
            entry.clear();
        }
    }

    /// 找到给定虚拟页号的三级页表项
    ///
    /// 如果找不到对应的页表项，则会相应创建页表
    pub fn find_entry(&mut self, vpn: VirtualPageNumber) -> MemoryResult<&mut PageTableEntry> {
        // 从根页表开始向下查询
        // 这里不用 self.page_tables[0] 避免后面产生 borrow-check 冲突（我太菜了）
        let root_table: &mut PageTable = PhysicalAddress::from(self.root_ppn).deref_kernel();
        let mut entry = &mut root_table.entries[vpn.levels()[0]];
        for vpn_slice in &vpn.levels()[1..] {
            if entry.is_empty() {
                // 如果页表不存在，则需要分配一个新的页表
                let new_table = PageTableTracker::new(FRAME_ALLOCATOR.lock().alloc()?);
                let new_ppn = new_table.page_number();
                // 将新页表的页号写入当前的页表项
                *entry = PageTableEntry::new(Some(new_ppn), Flags::VALID);
                // 保存页表
                self.page_tables.push(new_table);
            }
            // 进入下一级页表（使用偏移量来访问物理地址）
            entry = &mut entry.get_next_table().entries[*vpn_slice];
        }
        // 此时 entry 位于第三级页表
        Ok(entry)
    }

    /// 查找虚拟地址对应的物理地址
    pub fn lookup(va: VirtualAddress) -> Option<PhysicalAddress> {
        let mut current_ppn;
        unsafe {
            llvm_asm!("csrr $0, satp" : "=r"(current_ppn) ::: "volatile");
            current_ppn ^= 8 << 60;
        }

        let root_table: &PageTable =
            PhysicalAddress::from(PhysicalPageNumber(current_ppn)).deref_kernel();
        let vpn = VirtualPageNumber::floor(va);
        let mut entry = &root_table.entries[vpn.levels()[0]];
        // 为了支持大页的查找，我们用 length 表示查找到的物理页需要加多少位的偏移
        let mut length = 12 + 2 * 9;
        for vpn_slice in &vpn.levels()[1..] {
            if entry.is_empty() {
                return None;
            }
            if entry.has_next_level() {
                length -= 9;
                entry = &mut entry.get_next_table().entries[*vpn_slice];
            } else {
                break;
            }
        }
        let base = PhysicalAddress::from(entry.page_number()).0;
        let offset = va.0 & ((1 << length) - 1);
        Some(PhysicalAddress(base + offset))
    }

    /// 为给定的虚拟 / 物理页号建立映射关系
    fn map_one(
        &mut self,
        vpn: VirtualPageNumber,
        ppn: Option<PhysicalPageNumber>,
        flags: Flags,
    ) -> MemoryResult<()> {
        // 定位到页表项
        let entry = self.find_entry(vpn)?;
        assert!(entry.is_empty(), "virtual address is already mapped");
        // 页表项为空，则写入内容
        *entry = PageTableEntry::new(ppn, flags);
        Ok(())
    }
}
```

### MemorySet

我们需要把内核的每个段根据不同的属性写入上面的封装的 Mapping 中，并把它作为一个新的结构 MemorySet 给后面的线程的概念使用：

os/src/memory/mapping/memory_set.rs

```rs
//! 一个线程中关于内存空间的所有信息 [`MemorySet`]
//!

use crate::memory::{
    address::*,
    config::*,
    mapping::{Flags, MapType, Mapping, Segment},
    range::Range,
    MemoryResult,
};
use alloc::{vec, vec::Vec};

/// 一个进程所有关于内存空间管理的信息
pub struct MemorySet {
    /// 维护页表和映射关系
    pub mapping: Mapping,
    /// 每个字段
    pub segments: Vec<Segment>,
}

impl MemorySet {
    /// 创建内核重映射
    pub fn new_kernel() -> MemoryResult<MemorySet> {
        // 在 linker.ld 里面标记的各个字段的起始点，均为 4K 对齐
        extern "C" {
            fn text_start();
            fn rodata_start();
            fn data_start();
            fn bss_start();
        }

        // 建立字段
        let segments = vec![
            // .text 段，r-x
            Segment {
                map_type: MapType::Linear,
                range: Range::from((text_start as usize)..(rodata_start as usize)),
                flags: Flags::READABLE | Flags::EXECUTABLE,
            },
            // .rodata 段，r--
            Segment {
                map_type: MapType::Linear,
                range: Range::from((rodata_start as usize)..(data_start as usize)),
                flags: Flags::READABLE,
            },
            // .data 段，rw-
            Segment {
                map_type: MapType::Linear,
                range: Range::from((data_start as usize)..(bss_start as usize)),
                flags: Flags::READABLE | Flags::WRITABLE,
            },
            // .bss 段，rw-
            Segment {
                map_type: MapType::Linear,
                range: Range::from(VirtualAddress::from(bss_start as usize)..*KERNEL_END_ADDRESS),
                flags: Flags::READABLE | Flags::WRITABLE,
            },
            // 剩余内存空间，rw-
            Segment {
                map_type: MapType::Linear,
                range: Range::from(*KERNEL_END_ADDRESS..VirtualAddress::from(MEMORY_END_ADDRESS)),
                flags: Flags::READABLE | Flags::WRITABLE,
            },
        ];
        let mut mapping = Mapping::new()?;

        // 每个字段在页表中进行映射
        for segment in segments.iter() {
            mapping.map(segment, None)?;
        }
        Ok(MemorySet {
            mapping,
            segments,
        })
    }

    /// 替换 `satp` 以激活页表
    ///
    /// 如果当前页表就是自身，则不会替换，但仍然会刷新 TLB。
    pub fn activate(&self) {
        self.mapping.activate();
    }

    /// 添加一个 [`Segment`] 的内存映射
    pub fn add_segment(&mut self, segment: Segment, init_data: Option<&[u8]>) -> MemoryResult<()> {
        // 检测 segment 没有重合
        assert!(!self.overlap_with(segment.page_range()));
        // 映射
        self.mapping.map(&segment, init_data)?;
        self.segments.push(segment);
        Ok(())
    }

    /// 移除一个 [`Segment`] 的内存映射
    ///
    /// `segment` 必须已经映射
    pub fn remove_segment(&mut self, segment: &Segment) -> MemoryResult<()> {
        // 找到对应的 segment
        let segment_index = self
            .segments
            .iter()
            .position(|s| s == segment)
            .expect("segment to remove cannot be found");
        self.segments.remove(segment_index);
        // 移除映射
        self.mapping.unmap(segment);
        Ok(())
    }

    /// 检测一段内存区域和已有的是否存在重叠区域
    pub fn overlap_with(&self, range: Range<VirtualPageNumber>) -> bool {
        for seg in self.segments.iter() {
            if range.overlap_with(&seg.page_range()) {
                return true;
            }
        }
        false
    }
}

```

另外，还需要在 os/src/memory/frame/frame_tracker.rs 中添加：

```rs

/// `FrameTracker` 可以 deref 得到对应的 `[u8; PAGE_SIZE]`
impl core::ops::Deref for FrameTracker {
    type Target = [u8; PAGE_SIZE];
    fn deref(&self) -> &Self::Target {
        self.page_number().deref_kernel()
    }
}

/// `FrameTracker` 可以 deref 得到对应的 `[u8; PAGE_SIZE]`
impl core::ops::DerefMut for FrameTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.page_number().deref_kernel()
    }
}

```

测试：

os/src/main.rs：

```rs

    interrupt::init();
    memory::init();

    let remap = memory::mapping::MemorySet::new_kernel().unwrap();
    remap.activate();

    println!("kernel remapped");

    panic!()

```

输出：

```rs
PMP0: 0x0000000080000000-0x000000008001ffff (A)
PMP1: 0x0000000000000000-0xffffffffffffffff (A,R,W,X)
mod interrupt initialized
mod memory initialized
kernel remapped
src/main.rs:59: 'explicit panic'

```