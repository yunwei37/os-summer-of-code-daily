# lab2 学习报告

这部分的代码从上一章 lab1 的代码开始完成：

这一章的实验指导中:

- 实现动态内存的分配
- 了解 `QEMU` 虚拟的物理内存机制
- 通过页的方式对物理内存进行管理

lab2 的代码主要在 memory 文件夹中；算法部分在 algorithm 文件夹中

## 动态内存分配

我们的内核中也需要动态内存分配，典型的应用场景有：

- `Box<T>` ，类似 `malloc` 有；
- 引用计数 `Rc<T>`，原子引用计数 `Arc<T>`，主要用于在引用计数清零，即某对象不再被引用时，对该对象进行自动回收；
- 一些 std 中的数据结构，如 Vec 和 HashMap 等。

需要实现 `Trait GlobalAlloc` :

将这个类实例化，并使用语义项 #[global_allocator] 进行标记。

为了实现 Trait GlobalAlloc，需要支持这两个函数：

```RS
unsafe fn alloc(&self, layout: Layout) -> *mut u8;
unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout);
```

例如 memory/heap2.rs ：

```rs

/// 利用 VectorAllocator 的接口实现全局分配器的 GlobalAlloc trait
unsafe impl alloc::alloc::GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let offset = (*self.0.get())
            .as_mut()
            .unwrap()
            .alloc(layout.size(), layout.align())
            .expect("Heap overflow");
        &mut HEAP_SPACE[offset] as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let offset = ptr as usize - &HEAP_SPACE as *const _ as usize;
        (*self.0.get())
            .as_mut()
            .unwrap()
            .dealloc(offset, layout.size(), layout.align());
    }
}

```

Layout:

- size 表示要分配的字节数;
- align 则表示分配的虚拟地址的最小对齐要求，即分配的地址要求是 align 的倍数。

### 支持动态内存分配的方法

连续内存分配算法: 使用 Buddy System 来实现:

这里可以试着调用 Buddy System Allocator 的轮子：

操作系统动态分配内存所用的堆大小（8M）

- #[global_allocator]：可以为全局需要用到堆的地方分配空间。
- #[alloc_error_handler]：空间分配错误的回调

### 动态内存分配测试

例如：

```rs
    // 动态内存分配测试
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    let v = Box::new(5);
    assert_eq!(*v, 5);
    core::mem::drop(v);
```

思考：

动态分配的内存地址在哪个范围里？

（上面源代码的注释中提及了在bss段中，是一个静态的没有初始化的数组，算是内核代码的一部分。

这一部分是内核的堆分配，参考 memory/heap.rs:

```rs
#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

/// 初始化操作系统运行时堆空间
pub fn init() {
    // 告诉分配器使用这一段预留的空间作为堆
    unsafe {
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}

```

## 物理内存探测

外设和内存的访问；

操作系统怎样知道物理内存所在的那段物理地址：

在 RISC-V 中，这个一般是由 Bootloader ，即 OpenSBI 来完成的。它来完成对于包括物理内存在内的各外设的扫描，将扫描结果以 DTB（Device Tree Blob）的格式保存在物理内存中的某个地方。随后 OpenSBI 会将其地址保存在 a1 寄存器中，给我们使用。

最后一块含义为 DRAM 的地址空间，这就是 OS 将要管理的 128 MB 的内存空间：

`0x80000000 	0x88000000 	DRAM 缺省 128MB，大小可配置`

首先建立一个 PhysicalAddress 的类，然后对其实现一系列的 usize 的加、减和输出等等操作；

直接将 DRAM 物理内存结束地址硬编码到内核中：记录下操作系统用到的地址结尾（即 linker script 中的 kernel_end）。

os/src/memory/config.rs：

```rs

lazy_static! {
    /// 内核代码结束的地址，即可以用来分配的内存起始地址
    ///
    /// 因为 Rust 语言限制，我们只能将其作为一个运行时求值的 static 变量，而不能作为 const
    pub static ref KERNEL_END_ADDRESS: PhysicalAddress = PhysicalAddress(kernel_end as usize);
}

extern "C" {
    /// 由 `linker.ld` 指定的内核代码结束位置
    ///
    /// 作为变量存在 [`KERNEL_END_ADDRESS`]
    fn kernel_end();
}

```

在各级文件中加入模块调用，并在 os/src/main.rs 尝试输出：

```rs
    // 注意这里的 KERNEL_END_ADDRESS 为 ref 类型，需要加 *
    println!("{}", *memory::config::KERNEL_END_ADDRESS);
```

## 物理内存管理:

通常，我们在分配物理内存时并不是以字节为单位，而是以一物理页(Frame)，即连续的 4 KB 字节为单位分配。我们希望用物理页号（Physical Page Number，PPN）来代表一物理页.

>物理页号与物理页形成一一映射

参考 os/src/memory/address.rs

在 os/src/memory/config.rs 中加入相关的设置：

```rs

/// 页 / 帧大小，必须是 2^n
pub const PAGE_SIZE: usize = 4096;

/// 可以访问的内存区域起始地址
pub const MEMORY_START_ADDRESS: PhysicalAddress = PhysicalAddress(0x8000_0000);
/// 可以访问的内存区域结束地址
pub const MEMORY_END_ADDRESS: PhysicalAddress = PhysicalAddress(0x8800_0000);

```

### 分配和回收:

为了方便管理所有的物理页，我们需要实现一个分配器可以进行分配和回收的操作：把区域的真实物理地址封装到了一个 FrameTracker 里面

- 分配器分配给我们 FrameTracker 作为一个帧的标识
- 我们利用 Rust 的 drop 机制在析构的时候自动实现回收。

封装一个物理页分配器：这个分配器将不涉及任何的具体算法。

具体的算法将用一个名为 Allocator 的 Rust trait 封装起来，而我们的 FrameAllocator 会依赖于具体的 trait 实现例化。


os/src/memory/frame/allocator.rs:

```rs
lazy_static! {
    /// 帧分配器
    pub static ref FRAME_ALLOCATOR: Mutex<FrameAllocator<AllocatorImpl>> = Mutex::new(FrameAllocator::new(Range::from(
            PhysicalPageNumber::ceil(PhysicalAddress::from(*KERNEL_END_ADDRESS))..PhysicalPageNumber::floor(MEMORY_END_ADDRESS),
        )
    ));
}

/// 帧分配 / 回收（取决于你用什么算法
pub struct FrameAllocator<T: Allocator> {
    /// 可用区间的起始
    start_ppn: PhysicalPageNumber,
    /// 分配器
    allocator: T,
}

impl<T: Allocator> FrameAllocator<T> {
    /// 创建对象
    pub fn new(range: impl Into<Range<PhysicalPageNumber>> + Copy) -> Self {
        FrameAllocator {
            start_ppn: range.into().start,
            allocator: T::new(range.into().len()),
        }
    }

    /// 分配帧，如果没有剩余则返回 `Err`
    pub fn alloc(&mut self) -> MemoryResult<FrameTracker> {
        self.allocator
            .alloc()
            .ok_or("no available frame to allocate")
            .map(|offset| FrameTracker(self.start_ppn + offset))
    }

    /// 将被释放的帧添加到空闲列表的尾部
    ///
    /// 这个函数会在 [`FrameTracker`] 被 drop 时自动调用，不应在其他地方调用
    pub(super) fn dealloc(&mut self, frame: &FrameTracker) {
        self.allocator.dealloc(frame.page_number() - self.start_ppn);
    }
}
```

有关具体的算法，我们封装了一个分配器需要的 Rust trait：

```rs
/// 分配器：固定容量，每次分配 / 回收一个元素
pub trait Allocator {
    /// 给定容量，创建分配器
    fn new(capacity: usize) -> Self;
    /// 分配一个元素，无法分配则返回 `None`
    fn alloc(&mut self) -> Option<usize>;
    /// 回收一个元素
    fn dealloc(&mut self, index: usize);
}
```

这一部分可以使用线段树，也可以不用；但似乎分配单个元素的时候线段树并没有多大意义；

使用 spin::Mutex<T> 对于 static mut 类型加锁以避免冲突；

把新写的模块加载进来，并在 main 函数中进行简单的测试：

```rs
    // 物理页分配
    for _ in 0..2 {
        let frame_0 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        let frame_1 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        println!("{} and {}", frame_0.address(), frame_1.address());
    }

```

思考，和上面的代码有何不同，我们的设计是否存在一些语法上的设计缺陷？

- 这样写会导致死锁。