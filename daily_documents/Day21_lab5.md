# lab5 学习报告

lab5 涉及：

- 设备树的概念和读取
- virtio 总线协议
- 块设备驱动的实现
- 将块设备托管给文件系统 

这一部分其实在 lab4 的实验中就已经可以部分接触到了（笑

## 设备树

设备树涉及这样一个问题，我们从哪里读取设备信息？

在 RISC-V 中，这个一般是由 bootloader，即 OpenSBI 固件完成的：它来完成对于包括物理内存在内的各外设的扫描，将扫描结果以设备树二进制对象（DTB，Device Tree Blob）的格式保存在物理内存中的某个地方。而这个放置的物理地址将放在 a1 寄存器中。

将会把 HART ID （HART，Hardware Thread，硬件线程，可以理解为执行的 CPU 核）放在 a0 寄存器上。

使用这两个参数，只需要把 rust_main 函数增加两个参数即可：

os/src/main.rs

```rs

#[no_mangle]
pub extern "C" fn rust_main(_hart_id: usize, dtb_pa: PhysicalAddress) -> ! {
    memory::init();
    interrupt::init();
    drivers::init(dtb_pa);
    
    println!("{} {}",_hart_id,dtb_pa);

    ...
}

```

打印输出结果：

0 PhysicalAddress(0x82200000)

### 设备树：

每个设备在物理上连接到了父设备上最后再通过总线等连接起来构成一整个设备树，在每个节点上都描述了对应设备的信息，如支持的协议是什么类型等等。而操作系统就是通过这些节点上的信息来实现对设备的识别的。

一个设备节点上会有几个标准属性：

- compatible：该属性指的是该设备的编程模型
- model：指的是设备生产商给设备的型号
- reg：用 reg 段来自定义存储一些信息

### 解析设备树

可以直接调用 rCore 中 device_tree 库，然后遍历树上节点即可：

os/src/drivers/device_tree.rs

```rs

/// 递归遍历设备树
fn walk(node: &Node) {
    // 检查设备的协议支持并初始化
    if let Ok(compatible) = node.prop_str("compatible") {
        if compatible == "virtio,mmio" {
            virtio_probe(node);
        }
    }
    // 遍历子树
    for child in node.children.iter() {
        walk(child);
    }
}

/// 整个设备树的 Headers（用于验证和读取）
struct DtbHeader {
    magic: u32,
    size: u32,
}

```

在开始的时候，有一步来验证 Magic Number：

这一步是一个保证系统可靠性的要求，是为了验证这段内存到底是不是设备树。

```rs
/// 遍历设备树并初始化设备
pub fn init(dtb_va: VirtualAddress) {
    let header = unsafe { &*(dtb_va.0 as *const DtbHeader) };
    // from_be 是大小端序的转换（from big endian）
    let magic = u32::from_be(header.magic);
    if magic == DEVICE_TREE_MAGIC {
        let size = u32::from_be(header.size);
        // 拷贝数据，加载并遍历
        let data = unsafe { slice::from_raw_parts(dtb_va.0 as *const u8, size as usize) };
        if let Ok(dt) = DeviceTree::load(data) {
            walk(&dt.root);
        }
    }
}

```

## virtio

### 挂载到 QEMU

为了让 QEMU 挂载上我们虚拟的存储设备，我们这里选了 QEMU 支持的 virtio 协议。

virtio：

VirtIO is a standardized interface which allows virtual machines access to simplified "virtual" devices, such as block devices, network adapters and consoles. Accessing devices through VirtIO on a guest VM improves performance over more traditional "emulated" devices, as VirtIO devices require only the bare minimum setup and configuration needed to send and receive data, while the host machine handles the majority of the setup and maintenance of the actual physical hardware.

virtio 起源于 virtio: Towards a De-Facto Standard For Virtual I/O Devices 这篇论文，主要针对于半虚拟化技术中对通用设备的抽象。

以 virtio 为中心的总线下又挂载了 virtio-blk（块设备）总线、virtio-net（网络设备）总线、virtio-pci（PCI 设备）总线等，本身就构成一个设备树。

需要在 QEMU 运行的时候加入选项：

os/Makefile

```makefile

IMG_FILE    := $(USER_BUILD)/disk.img


# 运行 QEMU
qemu: build
    @qemu-system-riscv64 \
            -machine virt \
            -nographic \
            -bios default \
            -device loader,file=$(BIN_FILE),addr=0x80200000 \
            -drive file=$(TEST_IMG),format=raw,id=sfs \      # 模拟存储设备
            -device virtio-blk-device,drive=sfs              # 以 virtio Block Device 的形式挂载到 virtio 总线上

```

###  virtio 节点探测

进一步来区分上面提到的那些 virtio 设备：

os/src/drivers/bus/virtio_mmio.rs

由 walk 调用：

```rs

/// 从设备树的某个节点探测 virtio 协议具体类型
pub fn virtio_probe(node: &Node) {
    // reg 属性中包含了描述设备的 Header 的位置
    let reg = match node.prop_raw("reg") {
        Some(reg) => reg,
        _ => return,
    };
    let pa = PhysicalAddress(reg.as_slice().read_be_u64(0).unwrap() as usize);
    let va = VirtualAddress::from(pa);
    let header = unsafe { &mut *(va.0 as *mut VirtIOHeader) };
    // 目前只支持某个特定版本的 virtio 协议
    if !header.verify() {
        return;
    }
    // 判断设备类型
    match header.device_type() {
        DeviceType::Block => virtio_blk::add_driver(header),
        device => println!("unrecognized virtio device: {:?}", device),
    }
}

```

- 从设备树节点的 reg 信息中可以读出设备更详细信息的放置位置；即内存映射读写 MMIO（Memory Mapped I/O）
- 为了访问这段地址，我们需要把它加到页表里面

### virtio_drivers 库

在 cargo.toml 中添加：

```toml
device_tree = { git = "https://github.com/rcore-os/device_tree-rs" }
virtio-drivers = { git = "https://github.com/rcore-os/virtio-drivers" }
```

我们使用 rCore 中的 virtio_drivers 库，这个库会帮我们通过 MMIO 的方式对设备进行交互，同时我们也需要给这个库提供一些诸如申请物理内存、物理地址虚拟转换等接口。

os/src/drivers/bus/virtio_mmio.rs

```rs

lazy_static! {
    /// 用于放置给设备 DMA 所用的物理页（[`FrameTracker`]）
    pub static ref TRACKERS: RwLock<BTreeMap<PhysicalAddress, FrameTracker>> =
        RwLock::new(BTreeMap::new());
}

/// 为 DMA 操作申请连续 pages 个物理页（为 [`virtio_drivers`] 库
#[no_mangle]
extern "C" fn virtio_dma_alloc(pages: usize) -> PhysicalAddress {
    let mut pa: PhysicalAddress = Default::default();
    let mut last: PhysicalAddress = Default::default();
    for i in 0..pages {
        let tracker: FrameTracker = FRAME_ALLOCATOR.lock().alloc().unwrap();
        if i == 0 {
            pa = tracker.address();
        } else {
            assert_eq!(last + PAGE_SIZE, tracker.address());
        }
        last = tracker.address();
        TRACKERS.write().insert(last, tracker);
    }
    return pa;
}

/// 为 DMA 操作释放对应的之前申请的连续的物理页（为 [`virtio_drivers`] 库提供）
#[no_mangle]
extern "C" fn virtio_dma_dealloc(pa: PhysicalAddress, pages: usize) -> i32 {
    for i in 0..pages {
        TRACKERS.write().remove(&(pa + i * PAGE_SIZE));
    }
    0
}

/// 将物理地址转为虚拟地址（为 [`virtio_drivers`] 库提供）
#[no_mangle]
extern "C" fn virtio_phys_to_virt(pa: PhysicalAddress) -> VirtualAddress {
    VirtualAddress::from(pa)
}

/// 将虚拟地址转为物理地址（为 [`virtio_drivers`] 库提供）
#[no_mangle]
extern "C" fn virtio_virt_to_phys(va: VirtualAddress) -> PhysicalAddress {
    Mapping::lookup(va).unwrap()
}

```

由于本身设备是通过直接内存访问DMA（Direct Memory Access）技术来实现数据传输的，CPU 只需要给出要传输哪些内容，放在哪段物理内存上面，把请求告诉设备，设备后面的操作就会利用 DMA 而不经过 CPU 直接传输，在传输结束之后，会通过中断请求 IRQ（Interrupt ReQuest）技术沿着设备树把"我做完了"这个信息告诉 CPU，CPU 会作为一个中断进一步处理。

- 之所以不是虚拟地址是因为 DMA 不会经过 CPU 的 MMU 技术。
- 现在，我们的 FRAME_ALLOCATOR 还只能分配一个帧出来，我们连续调用，暂时先假设他是连续的。

在 0x80000000 到 0x88000000 的区间的物理页有可能对应着两个虚拟页：启动时的线性映射；内核栈是以 Frame 为单位分配的。

## 驱动和块设备驱动

irtio-blk 设备，这种设备提供了以整块为粒度的读和写操作，一般对应到真实的物理设备是硬盘。

### 驱动抽象

在写块设备驱动之前，我们先抽象驱动的概念:

os/src/drivers/driver.rs:

```rs
/// 驱动类型
///
/// 目前只有块设备，可能还有网络、GPU 设备等
#[derive(Debug, Eq, PartialEq)]
pub enum DeviceType {
    Block,
}

/// 驱动的接口
pub trait Driver: Send + Sync {
    /// 设备类型
    fn device_type(&self) -> DeviceType;

    /// 读取某个块到 buf 中（块设备接口）
    fn read_block(&self, _block_id: usize, _buf: &mut [u8]) -> bool {
        unimplemented!("not a block driver")
    }

    /// 将 buf 中的数据写入块中（块设备接口）
    fn write_block(&self, _block_id: usize, _buf: &[u8]) -> bool {
        unimplemented!("not a block driver")
    }
}

lazy_static! {
    /// 所有驱动
    pub static ref DRIVERS: RwLock<Vec<Arc<dyn Driver>>> = RwLock::new(Vec::new());
}

```

不过这样写还是为了方便未来的扩展x

### 抽象块设备

有了驱动的概念，我们就可以进一步抽象块设备：

这里漏了文件夹标签

os/src/drivers/block/mod.rs

这里所谓的 BlockDevice 其实就是一个 Driver 的引用。而且利用 rcore-fs 中提供的 BlockDevice trait 实现了为文件系统的接口，实际上是对上传文件系统的连接。

```rs

use super::driver::Driver;
use alloc::sync::Arc;
use rcore_fs::dev;

pub mod virtio_blk;

/// 块设备抽象（驱动的引用）
pub struct BlockDevice(pub Arc<dyn Driver>);

/// 为 [`BlockDevice`] 实现 [`rcore-fs`] 中 [`BlockDevice`] trait
///
/// 使得文件系统可以通过调用块设备的该接口来读写
impl dev::BlockDevice for BlockDevice {
    /// 每个块的大小（取 2 的对数）
    ///
    /// 这里取 512B 是因为 virtio 驱动对设备的操作粒度为 512B
    const BLOCK_SIZE_LOG2: u8 = 9;

    /// 读取某个块到 buf 中
    fn read_at(&self, block_id: usize, buf: &mut [u8]) -> dev::Result<()> {
        match self.0.read_block(block_id, buf) {
            true => Ok(()),
            false => Err(dev::DevError),
        }
    }

    /// 将 buf 中的数据写入块中
    fn write_at(&self, block_id: usize, buf: &[u8]) -> dev::Result<()> {
        match self.0.write_block(block_id, buf) {
            true => Ok(()),
            false => Err(dev::DevError),
        }
    }

    /// 执行和设备的同步
    ///
    /// 因为我们这里全部为阻塞 I/O 所以不存在同步的问题
    fn sync(&self) -> dev::Result<()> {
        Ok(())
    }
}

```

### virtio-blk 块设备驱动

通过调用现成的库完成 virtio-blk 的驱动:

os/src/drivers/block/virtio_blk.rs

```rs
use super::super::driver::{DeviceType, Driver, DRIVERS};
use alloc::sync::Arc;
use spin::Mutex;
use virtio_drivers::{VirtIOBlk, VirtIOHeader};

/// virtio 协议的块设备驱动
struct VirtIOBlkDriver(Mutex<VirtIOBlk<'static>>);

/// 为 [`VirtIOBlkDriver`] 实现 [`Driver`] trait
///
/// 调用了 [`virtio_drivers`] 库，其中规定的块大小为 512B
impl Driver for VirtIOBlkDriver {
    /// 设备类型
    fn device_type(&self) -> DeviceType {
        DeviceType::Block
    }

    /// 读取某个块到 buf 中
    fn read_block(&self, block_id: usize, buf: &mut [u8]) -> bool {
        self.0.lock().read_block(block_id, buf).is_ok()
    }

    /// 将 buf 中的数据写入块中
    fn write_block(&self, block_id: usize, buf: &[u8]) -> bool {
        self.0.lock().write_block(block_id, buf).is_ok()
    }
}

/// 将从设备树中读取出的设备信息放到 [`static@DRIVERS`] 中
pub fn add_driver(header: &'static mut VirtIOHeader) {
    let virtio_blk = VirtIOBlk::new(header).expect("failed to init blk driver");
    let driver = Arc::new(VirtIOBlkDriver(Mutex::new(virtio_blk)));
    DRIVERS.write().push(driver);
}

```

目前 virtio-drivers 库中的代码虽然调用了 DMA，但是返回时还是阻塞的逻辑，我们这里为了简化也没有设计 IRQ 的响应机制。

关于驱动部分的总结：

设计模式（从上往下）：

- trait BlockDevice
- lockDevice(pub Arc Driver)
- trait Driver
- struct VirtioBlkDriver

其中 Driver 作为一个核心 trait 为上提供实现，上层也就是 Driver 的使用侧（设备的抽象），而下层则是 Driver 的实现侧（设备的实现）。

## 文件系统

之前我们在加载 QEMU 的时候引入了一个磁盘镜像文件，这个文件的打包是由 rcore-fs-fuse 工具 来完成的.

接下来我们需要让操作系统理解块设备里面的文件系统。

### Simple File System

因为文件系统本身比较庞大，我们这里还是采用了 rCore 中的文件系统模块 rcore-fs：

[https://github.com/rcore-os/rcore-fs](https://github.com/rcore-os/rcore-fs)

我们这里选择最简单的 Simple File System

具体实现：

os/src/fs/mod.rs

根目录将会在我们第一次使用 ROOT_INODE 时进行初始化，而初始化的方式是找到全部设备驱动中的第一个存储设备作为根目录。

```rs

lazy_static! {
    /// 根文件系统的根目录的 INode
    pub static ref ROOT_INODE: Arc<dyn INode> = {
        // 选择第一个块设备
        for driver in DRIVERS.read().iter() {
            if driver.device_type() == DeviceType::Block {
                let device = BlockDevice(driver.clone());
                // 动态分配一段内存空间作为设备 Cache
                let device_with_cache = Arc::new(BlockCache::new(device, BLOCK_CACHE_CAPACITY));
                return SimpleFileSystem::open(device_with_cache)
                    .expect("failed to open SFS")
                    .root_inode();
            }
        }
        panic!("failed to load fs")
    };
}

```

BlockCache，该模块也是 rcore-fs 提供的，提供了一个存储设备在内存 Cache 的抽象，通过调用 BlockCache::new(device, BLOCK_CACHE_CAPACITY) 就可以把 device 自动变为一个有 Cache 的设备。

### 测试

os/src/fs/mod.rs

```rs

/// 触发 [`static@ROOT_INODE`] 的初始化并打印根目录内容
pub fn init() {
    ROOT_INODE.ls();
    println!("mod fs initialized");
}

```

os/src/fs/inode_ext.rs

为 [`INode`] 实现 trait [`INodeExt`] 以扩展功能

```rs

use super::*;

/// 为 [`INode`] 类型添加的扩展功能
pub trait INodeExt {
    /// 打印当前目录的文件
    fn ls(&self);

    /// 读取文件内容
    fn readall(&self) -> Result<Vec<u8>>;
}

impl INodeExt for dyn INode {
    fn ls(&self) {
        let mut id = 0;
        while let Ok(name) = self.get_entry(id) {
            println!("{}", name);
            id += 1;
        }
    }

    fn readall(&self) -> Result<Vec<u8>> {
        // 从文件头读取长度
        let size = self.metadata()?.size;
        // 构建 Vec 并读取
        let mut buffer = Vec::with_capacity(size);
        unsafe { buffer.set_len(size) };
        self.read_at(0, buffer.as_mut_slice())?;
        Ok(buffer)
    }
}

```

最后在主函数中测试初始化，然后测试在另一个内核线程中创建个文件夹：

os/src/main.rs

```rs
/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
#[no_mangle]
pub extern "C" fn rust_main(_hart_id: usize, dtb_pa: PhysicalAddress) -> ! {
    memory::init();
    interrupt::init();
    drivers::init(dtb_pa);
    fs::init();

    let process = Process::new_kernel().unwrap();

    PROCESSOR
        .get()
        .add_thread(Thread::new(process.clone(), simple as usize, Some(&[0])).unwrap());

    // 把多余的 process 引用丢弃掉
    drop(process);

    PROCESSOR.lock().run()
}

/// 测试任何内核线程都可以操作文件系统和驱动
fn simple(id: usize) {
    println!("hello from thread id {}", id);
    // 新建一个目录
    fs::ROOT_INODE
        .create("tmp", rcore_fs::vfs::FileType::Dir, 0o666)
        .expect("failed to mkdir /tmp");
    // 输出根文件目录内容
    fs::ls("/");

    loop {}
}

```

make run：

```

```

## 小结

本章我们的工作有：

- 在 QEMU 上挂载了存储设备
- 通过读取设备树找到了挂载的设备
- 实现了 virtio 驱动，把物理设备抽象为了驱动
- 进一步把驱动抽象给上层文件系统使用
- 调用 rcore-fs 的文件系统实现对文件的管理
