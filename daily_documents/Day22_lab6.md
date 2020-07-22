# lab6 学习报告

这一章的实验指导包含：


- 单独生成 ELF 格式的用户程序，并打包进文件系统中
- 创建并运行用户进程
- 使用系统调用为用户程序提供服务

## 构建用户程序框架

接下来，我们需要为用户程序提供一个类似的没有Rust std标准运行时依赖的极简运行时环境。这里我们会快速梳理一遍我们为用户程序进行的流程。

`与 os 的旁边建立一个 ` typo

首先，我们在 os 的旁边建立一个 user crate，移除默认的 main.rs，而是在 src 目录下建立 lib 和 bin 子目录， 在 lib 中存放的是极简运行时环境，在 bin 中存放的源文件会被编译成多个单独的执行文件。

### 基础框架搭建

和操作系统一样，我们需要为用户程序移除 std 依赖，并且补充一些必要的功能：

在 lib.rs 中添加：

- 声明
- 堆栈相关
- panic 处理
- 入口函数

```rs

#![no_std]
#![feature(llvm_asm)]
#![feature(lang_items)]
#![feature(panic_info_message)]
#![feature(linkage)]


/// 大小为 [`USER_HEAP_SIZE`] 的堆空间
static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

/// 使用 `buddy_system_allocator` 中的堆
#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

/// 打印 panic 信息并退出用户程序
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "\x1b[1;31m{}:{}: '{}'\x1b[0m",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("\x1b[1;31mpanic: '{}'\x1b[0m", info.message().unwrap());
    }
    sys_exit(-1);
}

/// 程序入口
#[no_mangle]
pub extern "C" fn _start(_args: isize, _argv: *const u8) -> ! {
    unsafe {
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
    sys_exit(main())
}

/// 默认的 main 函数
///
/// 设置了弱的 linkage，会被 `bin` 中文件的 `main` 函数取代
#[linkage = "weak"]
#[no_mangle]
fn main() -> isize {
    panic!("no main() linked");
}

/// 终止程序
#[no_mangle]
pub extern "C" fn abort() {
    panic!("abort");
}

/// 内存不足时终止程序
#[lang = "oom"]
fn oom(_: Layout) -> ! {
    panic!("out of memory");
}


```

另外，在 .cargo/config 还需要设置编译目标为 RISC-V 64：

```toml
# 编译的目标平台
[build]
target = "riscv64imac-unknown-none-elf"

```

console.rs:

在 stdout stdin 基础上进行输入输出

```rs
//! 在系统调用基础上实现 `print!` `println!`
//!
//! 代码与 `os` crate 中的 `console.rs` 基本相同

use crate::syscall::*;
use alloc::string::String;
use core::fmt::{self, Write};

/// 实现 [`core::fmt::Write`] trait 来进行格式化输出
struct Stdout;

impl Write for Stdout {
    /// 打印一个字符串
    fn write_str(&mut self, s: &str) -> fmt::Result {
        sys_write(STDOUT, s.as_bytes());
        Ok(())
    }
}

/// 打印由 [`core::format_args!`] 格式化后的数据
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// 实现类似于标准库中的 `print!` 宏
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

/// 实现类似于标准库中的 `println!` 宏
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

/// 从控制台读取一个字符（阻塞）
pub fn getchar() -> u8 {
    let mut c = [0u8; 1];
    sys_read(STDIN, &mut c);
    c[0]
}

/// 从控制台读取一个或多个字符（阻塞）
pub fn getchars() -> String {
    let mut buffer = [0u8; 64];
    loop {
        let size = sys_read(STDIN, &mut buffer);
        if let Ok(string) = String::from_utf8(buffer.iter().copied().take(size as usize).collect())
        {
            return string;
        }
    }
}

```

## 打包为磁盘镜像

现在，我们只需要利用工具将编译后的用户程序打包为镜像，就可以使用了。

安装工具：

```cargo install rcore-fs-fuse --git https://github.com/rcore-os/rcore-fs```

打包:

这个工具可以将一个目录打包成 SimpleFileSystem 格式的磁盘镜像。

将elf文件单独放在一个导出目录中，即

`user/build/disk`:

user/Makefile

```makefile
.PHONY: build

TARGET      := riscv64imac-unknown-none-elf
MODE        := debug

# 用户程序目录
SRC_DIR		:= src/bin
# 编译后执行文件目录
TARGET_DIR	:= target/$(TARGET)/$(MODE)
# 用户程序源文件
SRC_FILES	:= $(wildcard $(SRC_DIR)/*.rs)
# 根据源文件取得编译后的执行文件
BIN_FILES	:= $(patsubst $(SRC_DIR)/%.rs, $(TARGET_DIR)/%, $(SRC_FILES))

OUT_DIR		:= build/disk
IMG_FILE	:= build/raw.img
QCOW_FILE	:= build/disk.img

# 安装 rcore-fs-fuse 工具
dependency:
ifeq ($(shell which rcore-fs-fuse),)
	@echo Installing rcore-fs-fuse
	@cargo install rcore-fs-fuse --git https://github.com/rcore-os/rcore-fs
endif

# 编译、打包、格式转换、预留空间
build: dependency
	@cargo build
	@echo Targets: $(patsubst $(SRC_DIR)/%.rs, %, $(SRC_FILES))
	@rm -rf $(OUT_DIR)
	@mkdir -p $(OUT_DIR)
	@cp $(BIN_FILES) $(OUT_DIR)
	@rcore-fs-fuse --fs sfs $(IMG_FILE) $(OUT_DIR) zip
	@qemu-img convert -f raw $(IMG_FILE) -O qcow2 $(QCOW_FILE)
	@qemu-img resize $(QCOW_FILE) +1G

clean:
	@cargo clean
	@rm -rf $(OUT_DIR) $(IMG_FILE) $(QCOW_FILE)
```

## 解析 ELF 文件并创建线程

现在，我们需要从 ELF 文件中加载用户程序的代码和数据信息，并且映射到内存中：

使用 xmas-elf 这个 crate 替我们实现了 ELF 的解析。

为 INode 添加一个将整个文件作为 [u8] 读取出来的方法：

os/src/fs/inode_ext.rs

```rs
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

### 解析各个字段

对于不同的字段进行不同的处理：

os/src/memory/mapping/memory_set.rs

```rs
/// 通过 elf 文件创建内存映射（不包括栈）
pub fn from_elf(file: &ElfFile, is_user: bool) -> MemoryResult<MemorySet> {
    // 建立带有内核映射的 MemorySet
    let mut memory_set = MemorySet::new_kernel()?;

    // 遍历 elf 文件的所有部分
    for program_header in file.program_iter() {
        if program_header.get_type() != Ok(Type::Load) {
            continue;
        }
        // 从每个字段读取「起始地址」「大小」和「数据」
        let start = VirtualAddress(program_header.virtual_addr() as usize);
        let size = program_header.mem_size() as usize;
        let data: &[u8] =
            if let SegmentData::Undefined(data) = program_header.get_data(file).unwrap() {
                data
            } else {
                return Err("unsupported elf format");
            };

        // 将每一部分作为 Segment 进行映射
        let segment = Segment {
            map_type: MapType::Framed,
            range: Range::from(start..(start + size)),
            flags: Flags::user(is_user)
                | Flags::readable(program_header.flags().is_read())
                | Flags::writable(program_header.flags().is_write())
                | Flags::executable(program_header.flags().is_execute()),
        };

        // 建立映射并复制数据
        memory_set.add_segment(segment, Some(data))?;
    }

    Ok(memory_set)
}

```

同时在这里也需要更改一下线程相关数据结构的定义（就不详细写了）

### 加载数据到内存中

思考：我们在为用户程序建立映射时，虚拟地址是 ELF 文件中写明的，那物理地址是程序在磁盘中存储的地址吗？这样做有什么问题吗？

这部分是把文件信息一次性全部加载到内存里面了，因此就是像正常那样映射执行。

我们将修改 Mapping::map 函数，为其增加一个参数表示用于初始化的数据。

```rs

    /// 加入一段映射，可能会相应地分配物理页面
    ///
    /// 未被分配物理页面的虚拟页号暂时不会写入页表当中，它们会在发生 PageFault 后再建立页表项。
    pub fn map(
        &mut self,
        segment: &Segment,
        init_data: Option<&[u8]>,
    ) -> MemoryResult<Vec<(VirtualPageNumber, FrameTracker)>> {
        match segment.map_type {
            // 线性映射，直接对虚拟地址进行转换
            MapType::Linear => {
                for vpn in segment.page_range().iter() {
                    self.map_one(vpn, vpn.into(), segment.flags | Flags::VALID)?;
                }
                // 拷贝数据
                if let Some(data) = init_data {
                    unsafe {
                        (&mut *slice_from_raw_parts_mut(segment.range.start.deref(), data.len()))
                            .copy_from_slice(data);
                    }
                }
                Ok(Vec::new())
            }
            // 需要分配帧进行映射
            MapType::Framed => {
                // 记录所有成功分配的页面映射
                let mut allocated_pairs = Vec::new();
                for vpn in segment.page_range().iter() {
                    // 分配物理页面
                    let mut frame = FRAME_ALLOCATOR.lock().alloc()?;
                    // 映射，填充 0，记录
                    self.map_one(vpn, frame.page_number(), segment.flags | Flags::VALID)?;
                    frame.fill(0);
                    allocated_pairs.push((vpn, frame));
                }

                // 拷贝数据，注意页表尚未应用，无法直接从刚刚映射的虚拟地址访问，因此必须用物理地址 + 偏移来访问。
                if let Some(data) = init_data {
                    // 对于 bss，参数会传入 data，但其长度为 0。我们已经在前面用 0 填充过页面了，因此跳过
                    if !data.is_empty() {
                        for (vpn, frame) in allocated_pairs.iter_mut() {
                            // 拷贝时必须考虑区间与整页不对齐的情况
                            //    start（仅第一页时非零）
                            //      |        stop（仅最后一页时非零）
                            // 0    |---data---|          4096
                            // |------------page------------|
                            let page_address = VirtualAddress::from(*vpn);
                            let start = if segment.range.start > page_address {
                                segment.range.start - page_address
                            } else {
                                0
                            };
                            let stop = min(PAGE_SIZE, segment.range.end - page_address);
                            // 计算来源和目标区间并进行拷贝
                            let dst_slice = &mut frame[start..stop];
                            let src_slice = &data[(page_address + start - segment.range.start)
                                ..(page_address + stop - segment.range.start)];
                            dst_slice.copy_from_slice(src_slice);
                        }
                    }
                }

                Ok(allocated_pairs)
            }
        }
    }

```

### 运行 Hello World

os/src/main.rs

```rs

// 从文件系统中找到程序
let app = fs::ROOT_INODE.find("hello_world").unwrap();
// 读取数据
let data = app.readall().unwrap();
// 解析 ELF 文件
let elf = ElfFile::new(data.as_slice()).unwrap();
// 利用 ELF 文件创建线程，映射空间并加载数据
let process = Process::from_elf(&elf, true).unwrap();
// 再从 ELF 中读出程序入口地址
let thread = Thread::new(process, elf.header.pt2.entry_point() as usize, None).unwrap();
// 添加线程
PROCESSOR.lock().add_thread(thread);

```

## 实现系统调用

为练习做准备：
- sys_read
- sys_write 
- sys_exit 

### 用户程序中调用系统调用

实验指导提供了这样一种系统调用格式

```rs
llvm_asm!("ecall" :
    "={x10}" (/* 返回读取长度 */) :
    "{x10}" (/* 文件描述符 */),
    "{x11}" (/* 读取缓冲区 */),
    "{x12}" (/* 缓冲区长度 */),
    "{x17}" (/* sys_read 编号 63 */) ::
);

```
### 避免忙等待

这里的读写系统调用都是阻塞的。

### 实现系统调用的思路

把系统调用的处理结果分为三类：


- 返回一个数值，程序继续执行
- 程序进入等待
- 程序将被终止

系统调用的处理流程：

- 首先，从相应的寄存器中取出调用代号和参数
- 根据调用代号，进入不同的处理流程，得到处理结果：
  - 返回数值并继续执行：返回值存放在 x10 寄存器，sepc += 4，继续此 context 的执行
  - 程序进入等待：同样需要更新 x10 和 sepc，但是需要将当前线程标记为等待，切换其他线程来执行
  - 程序终止：不需要考虑系统调用的返回，直接终止线程

os/src/kernel/syscall.rs

系统调用的总入口：

```rs
//! 实现各种系统调用

use super::*;
use alloc::{format, string::String};

pub const SYS_READ: usize = 63;
pub const SYS_WRITE: usize = 64;
pub const SYS_EXIT: usize = 93;

/// 系统调用在内核之内的返回值
pub(super) enum SyscallResult {
    /// 继续执行，带返回值
    Proceed(isize),
    /// 记录返回值，但暂存当前线程
    Park(isize),
    /// 丢弃当前 context，调度下一个线程继续执行
    Kill,
}

/// 系统调用的总入口
pub fn syscall_handler(context: &mut Context) -> Result<*mut Context, String> {
    // 无论如何处理，一定会跳过当前的 ecall 指令
    context.sepc += 4;

    let syscall_id = context.x[17];
    let args = [context.x[10], context.x[11], context.x[12]];

    let result = match syscall_id {
        SYS_READ => sys_read(args[0], args[1] as *mut u8, args[2]),
        SYS_WRITE => sys_write(args[0], args[1] as *mut u8, args[2]),
        SYS_EXIT => sys_exit(args[0]),
        _ => return Err(format!("unimplemented syscall: {}", syscall_id)),
    };

    Ok(match result {
        SyscallResult::Proceed(ret) => {
            // 将返回值放入 context 中
            context.x[10] = ret as usize;
            context
        }
        SyscallResult::Park(ret) => {
            // 将返回值放入 context 中
            context.x[10] = ret as usize;
            // 保存 context，准备下一个线程
            PROCESSOR.get().park_current_thread(context);
            PROCESSOR.get().prepare_next_thread()
        }
        SyscallResult::Kill => {
            // 终止，跳转到 PROCESSOR 调度的下一个线程
            PROCESSOR.get().kill_current_thread();
            PROCESSOR.get().prepare_next_thread()
        }
    })
}


```

## 处理文件描述符

这里我们只为 stdin 和 stdout 实现最简单的读写接口。

- 首先，操作系统需要为进程维护一个进程打开的文件清单；
- stdin 和 stdout，它们的文件描述符数值分别为 0 和 1；
- 输出流最为简单：每当遇到系统调用时，直接将缓冲区中的字符通过 SBI 调用打印出去。
- 输入流较为复杂：每当遇到系统调用时，通过中断或轮询方式获取字符：如果有，就进一步获取；如果没有就等待。直到收到约定长度的字符串才返回。

因此，我们来看看外部中断的实现：

每一个键盘按键对于操作系统而言都是一次短暂的中断：

OpenSBI 默认会关闭各种外部中断，需要将其打开，来接受按键信息。

这里需要调整一下中断开启部分：

os/src/interrupt/handler.rs

```rs

/// 初始化中断处理
///
/// 把中断入口 `__interrupt` 写入 `stvec` 中，并且开启中断使能
pub fn init() {
    unsafe {
        extern "C" {
            /// `interrupt.asm` 中的中断入口
            fn __interrupt();
        }
        // 使用 Direct 模式，将中断入口设置为 `__interrupt`
        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);

        // 开启外部中断使能
        sie::set_sext();

        // 在 OpenSBI 中开启外部中断
        *PhysicalAddress(0x0c00_2080).deref_kernel() = 1 << 10;
        // 在 OpenSBI 中开启串口
        *PhysicalAddress(0x1000_0004).deref_kernel() = 0x0bu8;
        *PhysicalAddress(0x1000_0001).deref_kernel() = 0x01u8;
    }
}

```

### 实现输入输出流

这里，缓冲区使用 alloc::collections::VecDeque 来实现。

在遇到键盘中断时，调用 sbi_call 来获取字符并加入到缓冲区中。当遇到系统调用 sys_read 时，再相应从缓冲区中取出一定数量的字符。

os/src/kernel/fs.rs

```rs

//! 文件相关的内核功能

use super::*;
use crate::fs::*;
use core::slice::from_raw_parts_mut;

/// 从指定的文件中读取字符
///
/// 如果缓冲区暂无数据，返回 0；出现错误返回 -1
// todo: inode 放到 process 中去
pub(super) fn sys_read(fd: usize, buffer: *mut u8, size: usize) -> SyscallResult {
    // 从线程中获取 inode，注意避免锁
    let inode: Arc<dyn INode> =
        if let Some(inode) = PROCESSOR.get().current_thread().inner().descriptors.get(fd) {
            inode.clone()
        } else {
            return SyscallResult::Proceed(-1);
        };
    let buffer = unsafe { from_raw_parts_mut(buffer, size) };
    if let Ok(ret) = inode.read_at(0, buffer) {
        let ret = ret as isize;
        if ret > 0 {
            return SyscallResult::Proceed(ret);
        }
        if ret == 0 {
            return SyscallResult::Park(ret);
        }
    }
    SyscallResult::Proceed(-1)
}

/// 将字符写入指定的文件
pub(super) fn sys_write(fd: usize, buffer: *mut u8, size: usize) -> SyscallResult {
    if let Some(inode) = PROCESSOR.get().current_thread().inner().descriptors.get(fd) {
        let buffer = unsafe { from_raw_parts_mut(buffer, size) };
        if let Ok(ret) = inode.write_at(0, buffer) {
            let ret = ret as isize;
            if ret >= 0 {
                return SyscallResult::Proceed(ret);
            }
        }
    }
    SyscallResult::Proceed(-1)
}

```

在 os/src/fs/stdout.rs 中，实现stdout：

```rs

//! 控制台输出 [`Stdout`]

use super::*;

lazy_static! {
    pub static ref STDOUT: Arc<Stdout> = Default::default();
}

/// 控制台输出
#[derive(Default)]
pub struct Stdout;

impl INode for Stdout {
    fn write_at(&self, offset: usize, buf: &[u8]) -> Result<usize> {
        if offset != 0 {
            Err(FsError::NotSupported)
        } else if let Ok(string) = core::str::from_utf8(buf) {
            print!("{}", string);
            Ok(buf.len())
        } else {
            Err(FsError::InvalidParam)
        }
    }

    /// Read bytes at `offset` into `buf`, return the number of bytes read.
    fn read_at(&self, _offset: usize, _buf: &mut [u8]) -> Result<usize> {
        Err(FsError::NotSupported)
    }

    fn poll(&self) -> Result<PollStatus> {
        Err(FsError::NotSupported)
    }

    /// This is used to implement dynamics cast.
    /// Simply return self in the implement of the function.
    fn as_any_ref(&self) -> &dyn Any {
        self
    }
}


```

## 条件变量

条件变量用来解决这样一个问题：如果遇到了 sys_read 系统调用，而缓冲区并没有数据可以读取，应该如何让线程进行等待，而又不浪费 CPU 资源呢？

条件变量（conditional variable）的常见接口是这样的：

- wait：当前线程开始等待这个条件变量
- notify_one：让某一个等待此条件变量的线程继续运行
- notify_all：让所有等待此变量的线程继续运行

条件变量和互斥锁的区别在于：
- 互斥锁解铃还须系铃人，但条件变量可以由任何来源发出 notify 信号。
- 互斥锁的一次 lock 一定对应一次 unlock，但条件变量多次 notify 只能保证 wait 的线程执行次数不超过 notify 次数。

为输入流加入条件变量后，就可以使得调用 sys_read 的线程在等待期间保持休眠，不被调度器选中，消耗 CPU 资源。

### 调整调度器

首先，我们需要为线程池单独设立一个「休眠区」，其中保存的线程与调度器互斥。：

os/src/process/processor.rs

```rs

pub struct Processor {
    /// 当前正在执行的线程
    current_thread: Option<Arc<Thread>>,
    /// 线程调度器，记录活跃线程
    scheduler: SchedulerImpl<Arc<Thread>>,
    /// 保存休眠线程
    sleeping_threads: HashSet<Arc<Thread>>,
}

```

### 实现条件变量

条件变量会被包含在输入流等涉及等待和唤起的结构中，而一个条件变量保存的就是所有等待它的线程。

os/src/kernel/condvar.rs

```rs

//! 条件变量

use super::*;
use alloc::collections::VecDeque;

#[derive(Default)]
pub struct Condvar {
    /// 所有等待此条件变量的线程
    watchers: Mutex<VecDeque<Arc<Thread>>>,
}

impl Condvar {
    /// 令当前线程休眠，等待此条件变量
    pub fn wait(&self) {
        self.watchers
            .lock()
            .push_back(PROCESSOR.get().current_thread());
        PROCESSOR.get().sleep_current_thread();
    }

    /// 唤起一个等待此条件变量的线程
    pub fn notify_one(&self) {
        if let Some(thread) = self.watchers.lock().pop_front() {
            PROCESSOR.get().wake_thread(thread);
        }
    }
}

```

当一个线程调用 sys_read 而缓冲区为空时，就会将其加入条件变量的 watcher 中，同时在 Processor 中移出活跃线程。而当键盘中断到来，读取到字符时，就会将线程重新放回调度器中，准备下一次调用。

os/src/fs/stdin.rs

```rs
//! 键盘输入 [`Stdin`]

use super::*;
use alloc::collections::VecDeque;

lazy_static! {
    pub static ref STDIN: Arc<Stdin> = Default::default();
}

/// 控制台键盘输入，实现 [`INode`] 接口
#[derive(Default)]
pub struct Stdin {
    /// 从后插入，前段弹出
    buffer: Mutex<VecDeque<u8>>,
    /// 条件变量用于使等待输入的线程休眠
    condvar: Condvar,
}

impl INode for Stdin {
    /// Read bytes at `offset` into `buf`, return the number of bytes read.
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> Result<usize> {
        if offset != 0 {
            // 不支持 offset
            Err(FsError::NotSupported)
        } else if self.buffer.lock().len() == 0 {
            // 缓冲区没有数据，将当前线程休眠
            self.condvar.wait();
            Ok(0)
        } else {
            let mut stdin_buffer = self.buffer.lock();
            for (i, byte) in buf.iter_mut().enumerate() {
                if let Some(b) = stdin_buffer.pop_front() {
                    *byte = b;
                } else {
                    return Ok(i);
                }
            }
            Ok(buf.len())
        }
    }

    /// Write bytes at `offset` from `buf`, return the number of bytes written.
    fn write_at(&self, _offset: usize, _buf: &[u8]) -> Result<usize> {
        Err(FsError::NotSupported)
    }

    fn poll(&self) -> Result<PollStatus> {
        Err(FsError::NotSupported)
    }

    /// This is used to implement dynamics cast.
    /// Simply return self in the implement of the function.
    fn as_any_ref(&self) -> &dyn Any {
        self
    }
}

impl Stdin {
    /// 向缓冲区插入一个字符，然后唤起一个线程
    pub fn push(&self, c: u8) {
        self.buffer.lock().push_back(c);
        self.condvar.notify_one();
    }
}


```

开放思考：如果多个线程同时等待输入流会怎么样？有什么解决方案吗？

会导致只有一个线程获取输入，别的就一直被阻塞。

比对源代码，这边还发现一个问题：

```rs
    .global boot_page_table
boot_page_table:
    # .8byte表示长度为8个字节的整数
    .8byte 0
    .8byte 0
    # 第 2 项：0x8000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1
    .8byte (0x80000 << 10) | 0xcf
    .zero 505 * 8
    # 第 508 项：0xffff_ffff_0000_0000 -> 0x0000_0000，0xcf 表示 VRWXAD 均为 1
    .8byte (0x00000 << 10) | 0xcf
    .8byte 0
    # 第 510 项：0xffff_ffff_8000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1
    .8byte (0x80000 << 10) | 0xcf
    .8byte 0

```

和之前相比，这个需要改成 8byte（好像是前几天的修改

## 总结

其实这一章的内容比较多（但很多代码没有写出来...

- 我们成功单独生成 ELF 格式的用户程序，并打包进文件系统中
- 从文件中读取，创建并运行用户进程
- 而为了可以让用户程序享受到操作系统的功能，我们使用系统调用为用户程序提供服务。 