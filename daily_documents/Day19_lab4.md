# lab4 学习报告

lab4 涉及：

- 线程和进程的概念以及运行状态的表示
- 线程的切换
- 对 CPU 进行抽象在上面完成对线程的调度 

lab4 的代码将会在 lab3 完成的代码上面继续进行改动；

## 线程和进程

- 程序（Program）：从源代码经过编译器一系列处理（编译、链接、优化等）得到的可执行文件
- 进程（Process）：正在运行并使用计算机资源的程序
- 线程 (Thread) ：借助 CPU 和栈的执行流

首先，进程得到了操作系统提供的资源：程序的代码、数据段被加载到内存中，程序所需的虚拟内存空间被真正构建出来。同时操作系统还给进程分配了程序所要求的各种其他资源，如我们上面几个章节中提到过的页表、文件的资源。一个进程可以有多个线程，也可以如传统进程一样只有一个线程。

- 资源的分配单位：进程
- 执行的调度单位：线程

### 线程的表示

在这里我们提供一种基础的实现，每个线程会包括：

- 线程 ID
- 运行栈
- 线程执行上下文 （ Context 类型 ）
- 所属进程的记号
- 内核栈

新建一个 process 文件夹：

先添加一个 mod.rs：

```rs

mod config;
mod kernel_stack;
#[allow(clippy::module_inception)]
mod process;
mod processor;
mod thread;

use crate::interrupt::*;
use crate::memory::*;
use alloc::{sync::Arc, vec, vec::Vec};
use spin::{Mutex, RwLock};

pub use config::*;
pub use kernel_stack::KERNEL_STACK;
pub use process::Process;
pub use processor::PROCESSOR;
pub use thread::Thread;

```

再添加

os/src/process/thread.rs：

包含线程的数据结构：

```rs
//! 线程 [`Thread`]

use super::*;
use crate::fs::*;
use core::hash::{Hash, Hasher};

/// 线程 ID 使用 `isize`，可以用负数表示错误
pub type ThreadID = isize;

static mut THREAD_COUNTER: ThreadID = 0;

/// 线程的信息
pub struct Thread {
    /// 线程 ID
    pub id: ThreadID,
    /// 线程的栈
    pub stack: Range<VirtualAddress>,
    /// 所属的进程
    pub process: Arc<RwLock<Process>>,
    /// 用 `Mutex` 包装一些可变的变量
    pub inner: Mutex<ThreadInner>,
}

/// 线程中需要可变的部分
pub struct ThreadInner {
    /// 线程执行上下文
    ///
    /// 当且仅当线程被暂停执行时，`context` 为 `Some`
    pub context: Option<Context>,
    /// 是否进入休眠
    pub sleeping: bool,
    /// 是否已经结束
    pub dead: bool,
    /// 打开的文件
    pub descriptors: Vec<Arc<dyn INode>>,
}

```

### 进程的表示

进程只需要维护页面映射，并且存储一点额外信息：

- 用户态标识：我们会在后面进行区分内核态线程和用户态线程。
- 访存空间 MemorySet：进程中的线程会共享同一个页表，即可以访问的虚拟内存空间（简称：访存空间）。

创建：os/src/process/process.rs

进程的数据结构：

```rs
//! 进程 [`Process`]

use super::*;
use xmas_elf::ElfFile;

/// 进程的信息
pub struct Process {
    /// 是否属于用户态
    pub is_user: bool,
    /// 进程中的线程公用页表 / 内存映射
    pub memory_set: MemorySet,
}

```

实际上这部分有一些代码和第五章的部分内容交织在了一起。

## 线程的创建

第一个目标就是创建一个线程并且让他运行起来。

一个线程要开始运行，需要这些准备工作：

- 建立页表映射，需要包括以下映射空间：
  - 线程所执行的一段指令
  - 线程执行栈
  - 操作系统的部分内存空间
- 设置起始执行的地址
- 初始化各种寄存器，比如 sp
- 可选：设置一些执行参数（例如 argc 和 argv等 ）

思考：为什么线程即便与操作系统无关，也需要在内存中映射操作系统的内存空间呢？

- 简单来说，需要使用内核来处理中断。
- 我们会为每个进程的页表映射全部操作系统的内存，这些页表都标记为内核权限（即 U 位为 0）。

### 执行第一个线程

需要对 interrupt.asm 稍作修改：

os/src/interrupt.asm

```rs
    
    # 取出 CSR 并保存
    csrr    t0, sstatus
    csrr    t1, sepc
    SAVE    t0, 32
    SAVE    t1, 33
    # 调用 handle_interrupt，传入参数
    # context: &mut Context
    mv      a0, sp
    # scause: Scause
    csrr    a1, scause
    # stval: usize
    csrr    a2, stval
    jal handle_interrupt

    
    .globl __restore
# 离开中断
# 此时内核栈顶被推入了一个 Context，而 a0 指向它
# 接下来从 Context 中恢复所有寄存器，并将 Context 出栈（用 sscratch 记录内核栈地址）
# 最后跳转至恢复的 sepc 的位置
__restore:
    # 从 a0 中读取 sp
    # 思考：a0 是在哪里被赋值的？（有两种情况）
    mv      sp, a0
    # 恢复 CSR
    LOAD    t0, 32
    LOAD    t1, 33
    csrw    sstatus, t0
    csrw    sepc, t1
    # 将内核栈地址写入 sscratch
    addi    t0, sp, 36*8
    csrw    sscratch, t0


```

### 设计 Context

- 保存通用寄存器
- sepc
- sepc：执行 sret 指令后会跳转到这里，所以 sepc 应当存储线程的入口地址（执行的函数地址）
- sstatus

关于 sstatus 标志位的具体意义：

- spp：中断前系统处于内核态（1）还是用户态（0）
- sie：内核态是否允许中断。对用户态而言，无论 sie 取何值都开启中断
- spie：中断前是否开中断（用户态中断时可能 sie 为 0）

设计好 Context 之后，我们只需要将它应用到所有的寄存器上（即执行 __restore），就可以切换到第一个线程了。

os/src/process/processor.rs

```rs

/// 第一次开始运行
///
/// 从 `current_thread` 中取出 [`Context`]，然后直接调用 `interrupt.asm` 中的 `__restore`
/// 来从 `Context` 中继续执行该线程。
pub fn run(&mut self) -> ! {
    // interrupt.asm 中的标签
    extern "C" {
        fn __restore(context: usize);
    }
    /* 激活线程的页表，取得 Context。具体过程会在后面讲解 */
    unsafe {
        __restore(context);
    }
    unreachable!()
}

```

### 在启动时不打开中断

在操作系统初始化的过程中是不应该有中断的。所以，我们删去之前设置「开启中断」的代码。

os/interrupt/timer.rs

```rs

/// 初始化时钟中断
///
/// 开启时钟中断使能，并且预约第一次时钟中断
pub fn init() {
    unsafe {
        // 开启 STIE，允许时钟中断
        sie::set_stimer();
        // 开启 SIE（不是 sie 寄存器），允许内核态被中断打断
        // sstatus::set_sie();
    }
    // 设置下一次时钟中断
    set_next_timeout();
}

```

## 线程的切换

当发生中断时，在 __restore 时，a0 寄存器的值是 handle_interrupt 函数的返回值。也就是说，如果我们令 handle_interrupt 函数返回另一个线程的 *mut Context，就可以在时钟中断后跳转到这个线程来执行。

从这个原理出发，我们就可以完成线程的切换工作

### 修改中断处理

在线程切换时（即时钟中断时），handle_interrupt 函数需要将上一个线程的 Context 保存起来，然后将下一个线程的 Context 恢复并返回。

os/src/interrupt/handler.rs

可以看到，当发生断点中断时，直接返回原来的上下文（修改一下 sepc）；而如果是时钟中断的时候，我们通过执行 PROCESSOR.get().tick(context) 函数得到的返回值作为上下文。

```rs
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) -> *mut Context {
    /* ... */
    // 根据中断类型来处理，返回的 Context 必须位于放在内核栈顶
    match scause.cause() {
        // 断点中断（ebreak）
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        // 系统调用
        Trap::Exception(Exception::UserEnvCall) => syscall_handler(context),
        // 时钟中断
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        // 外部中断（键盘输入）
        Trap::Interrupt(Interrupt::SupervisorExternal) => supervisor_external(context),
        // 其他情况，无法处理
        _ => fault("unimplemented interrupt type: {:x?}", scause, stval),
    }
}

/// 处理 ebreak 断点
fn breakpoint(context: &mut Context) -> *mut Context {
    println!("Breakpoint at 0x{:x}", context.sepc);
    context.sepc += 2;
    context
}

/// 处理时钟中断
fn supervisor_timer(context: &mut Context) -> *mut Context {
    timer::tick();
    PROCESSOR.get().tick(context)
}

```

### 线程切换

关于 Processor::tick 函数是如何实现的：

os/src/process/processor.rs

```rs

/// 在一个时钟中断时，替换掉 context
pub fn tick(&mut self, context: &mut Context) -> *mut Context {
    // 向调度器询问下一个线程
    if let Some(next_thread) = self.scheduler.get_next() {
        if next_thread == self.current_thread() {
            // 没有更换线程，直接返回 Context
            context
        } else {
            // 准备下一个线程
            let next_context = next_thread.run();
            let current_thread = self.current_thread.replace(next_thread).unwrap();
            // 储存当前线程 Context
            current_thread.park(*context);
            // 返回下一个线程的 Context
            next_context
        }
    } else {
        panic!("all threads terminated, shutting down");
    }
}

```

### 上下文 Context 的保存和取出

os/src/process/thread.rs

在线程切换时，我们需要保存前一个线程的 Context，为此我们实现 Thread::park 函数。

```rs

/// 发生时钟中断后暂停线程，保存状态
pub fn park(&self, context: Context) {
    // 检查目前线程内的 context 应当为 None
    let mut slot = self.context.lock();
    assert!(slot.is_none());
    // 将 Context 保存到线程中
    slot.replace(context);
}

```

我们需要取出下一个线程的 Context，为此我们实现 Thread::run。启动一个线程除了需要 Context，还需要切换页表。这个操作我们也在这个方法中完成：

```rs

/// 准备执行一个线程
///
/// 激活对应进程的页表，并返回其 Context
pub fn run(&self) -> *mut Context {
    // 激活页表
    self.process.read().memory_set.activate();
    // 取出 Context
    let parked_frame = self.context.lock().take().unwrap();

    if self.process.read().is_user {
        // 用户线程则将 Context 放至内核栈顶
        KERNEL_STACK.push_context(parked_frame)
    } else {
        // 内核线程则将 Context 放至 sp 下
        let context = (parked_frame.sp() - size_of::<Context>()) as *mut Context;
        unsafe { *context = parked_frame };
        context
    }
}

```

思考：在 run 函数中，我们在一开始就激活了页表，会不会导致后续流程无法正常执行？

此时映射的操作系统内核空间是不变的

我们需要为sp指针准备好一个专门用于在内核态执行函数的内核栈。

## 内核栈

需求：

- 内核栈只会在中断时使用，而中断结束后就不再使用
- 只需要实现一个共用的内核栈就可以了
- 每个线程都需要能够在中断时第一时间找到内核栈的地址
- 我们将内核栈的地址存放到内核态使用的特权寄存器 sscratch 中


解决方案：

- 预留一段空间作为内核栈
- 运行线程时，在 sscratch 寄存器中保存内核栈指针
- 如果线程遇到中断，则从将 Context 压入 sscratch 指向的栈中（Context 的地址为 sscratch - size_of::<Context>()），同时用新的栈地址来替换 sp（此时 sp 也会被复制到 a0 作为 handle_interrupt 的参数） ，这一部分代码可以参考之前的 asm
- 从中断中返回时（__restore 时），a0 应指向被压在内核栈中的 Context。此时出栈 Context 并且将栈顶保存到 sscratch 中

### 具体实现

为内核栈预留空间：我们直接使用一个 static mut 来指定一段空间作为栈。

在我们创建线程时，需要使用的操作就是在内核栈顶压入一个初始状态 Context：

os/src/process/kernel_stack.rs

```rs

//! 内核栈 [`KernelStack`]
//!
//! 用户态的线程出现中断时，因为用户栈无法保证可用性，中断处理流程必须在内核栈上进行。
//! 所以我们创建一个公用的内核栈，即当发生中断时，会将 Context 写到内核栈顶。
//!
//! ### 线程 [`Context`] 的存放
//! > 1. 线程初始化时，一个 `Context` 放置在内核栈顶，`sp` 指向 `Context` 的位置
//! >   （即栈顶 - `size_of::<Context>()`）
//! > 2. 切换到线程，执行 `__restore` 时，将 `Context` 的数据恢复到寄存器中后，
//! >   会将 `Context` 出栈（即 `sp += size_of::<Context>()`），
//! >   然后保存 `sp` 至 `sscratch`（此时 `sscratch` 即为内核栈顶）
//! > 3. 发生中断时，将 `sscratch` 和 `sp` 互换，入栈一个 `Context` 并保存数据
//!
//! 容易发现，线程的 `Context` 一定保存在内核栈顶。因此，当线程需要运行时，
//! 从 [`Thread`] 中取出 `Context` 然后置于内核栈顶即可

use super::*;
use core::mem::size_of;

/// 内核栈
#[repr(align(16))]
#[repr(C)]
pub struct KernelStack([u8; KERNEL_STACK_SIZE]);

/// 公用的内核栈
pub static mut KERNEL_STACK: KernelStack = KernelStack([0; KERNEL_STACK_SIZE]);

impl KernelStack {
    /// 在栈顶加入 Context 并且返回新的栈顶指针
    pub fn push_context(&mut self, context: Context) -> *mut Context {
        // 栈顶
        let stack_top = &self.0 as *const _ as usize + size_of::<Self>();
        // Context 的位置
        let push_address = (stack_top - size_of::<Context>()) as *mut Context;
        unsafe {
            *push_address = context;
        }
        push_address
    }
}

```

os/src/interrupt.asm：

在这个汇编代码中，我们需要加入对 sscratch 的判断和使用,以及事后的恢复：

```rs

    ...
    # 因为线程当前的栈不一定可用，必须切换到内核栈来保存 Context 并进行中断流程
    # 因此，我们使用 sscratch 寄存器保存内核栈地址
    # 思考：sscratch 的值最初是在什么地方写入的？

    # 交换 sp 和 sscratch（切换到内核栈）
    csrrw   sp, sscratch, sp
    # 在内核栈开辟 Context 的空间
    addi    sp, sp, -36*8
    ...

    # 保存通用寄存器，除了 x0（固定为 0）
    SAVE    x1, 1


    ...

# 离开中断
# 此时内核栈顶被推入了一个 Context，而 a0 指向它
# 接下来从 Context 中恢复所有寄存器，并将 Context 出栈（用 sscratch 记录内核栈地址）
# 最后跳转至恢复的 sepc 的位置
__restore:
    # 从 a0 中读取 sp
    # 思考：a0 是在哪里被赋值的？（有两种情况）
    mv      sp, a0
    # 恢复 CSR
    LOAD    t0, 32
    LOAD    t1, 33
    csrw    sstatus, t0
    csrw    sepc, t1
    # 将内核栈地址写入 sscratch
    addi    t0, sp, 36*8
    csrw    sscratch, t0

```

思考：在栈的切换过程中，会不会导致一些栈空间没有被释放，或者被错误释放的情况？

应当是不会的；每个中断在压栈后都会出栈（除了最开始的线程

## 调度器

### 处理器抽象

我们已经可以创建和保存线程了，现在，我们再抽象出「处理器」来存放和管理线程池。同时，也需要存放和管理目前正在执行的线程（即中断前执行的线程，因为操作系统在工作时是处于中断、异常或系统调用服务之中）。

os/src/process/processor.rs

```rs
lazy_static! {
    /// 全局的 [`Processor`]
    pub static ref PROCESSOR: Lock<Processor> = Lock::new(Processor::default());
}

/// 线程调度和管理
#[derive(Default)]
pub struct Processor {
    /// 当前正在执行的线程
    current_thread: Option<Arc<Thread>>,
    /// 线程调度器，记录活跃线程
    scheduler: SchedulerImpl<Arc<Thread>>,
    /// 保存休眠线程
    sleeping_threads: HashSet<Arc<Thread>>,
}

```

注意到这里我们用了一个 Lock，它封装了 spin::Mutex，而在其基础上进一步关闭了中断。这是因为我们在内核线程中也有可能访问 PROCESSOR，但是此时我们不希望它被时钟打断，这样在中断处理中就无法访问 PROCESSOR 了，因为它已经被锁住。

感觉这里最好能提及一下 lock.rs:

```rs
//! 一个关闭中断的互斥锁 [`Lock`]

use spin::{Mutex, MutexGuard};

/// 关闭中断的互斥锁
#[derive(Default)]
pub struct Lock<T>(pub(self) Mutex<T>);

/// 封装 [`MutexGuard`] 来实现 drop 时恢复 sstatus
pub struct LockGuard<'a, T> {
    /// 在 drop 时需要先 drop 掉 [`MutexGuard`] 再恢复 sstatus
    guard: Option<MutexGuard<'a, T>>,
    /// 保存的关中断前 sstatus
    sstatus: usize,
}

```

### 调度器

调度器的算法有许多种，我们将它提取出一个 trait 作为接口

```rs

/// 线程调度器
///
/// 这里 `ThreadType` 就是 `Arc<Thread>`
pub trait Scheduler<ThreadType: Clone + Eq>: Default {
    /// 优先级的类型
    type Priority;
    /// 向线程池中添加一个线程
    fn add_thread(&mut self, thread: ThreadType);
    /// 获取下一个时间段应当执行的线程
    fn get_next(&mut self) -> Option<ThreadType>;
    /// 移除一个线程
    fn remove_thread(&mut self, thread: &ThreadType);
    /// 设置线程的优先级
    fn set_priority(&mut self, thread: ThreadType, priority: Self::Priority);
}

```

### 运行！

补充 Processor::run 的实现，让我们运行起第一个线程:

os/src/process/processor.rs: impl Processor

```rs

/// 第一次开始运行
pub fn run(&mut self) -> ! {
    // interrupt.asm 中的标签
    extern "C" {
        fn __restore(context: usize);
    }
    // 从 current_thread 中取出 Context
    if self.current_thread.is_none() {
        panic!("no thread to run, shutting down");
    }
    let context = self.current_thread().prepare();
    // 从此将没有回头
    unsafe {
        __restore(context as usize);
    }
    unreachable!()
}


```

os/src/main.rs

```rs
/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    memory::init();
    interrupt::init();

    {
        // 新建一个带有内核映射的进程。需要执行的代码就在内核中
        let process = Process::new_kernel().unwrap();

        for message in 0..8 {
            let thread = Thread::new(
                process.clone(),            // 使用同一个进程
                sample_process as usize,    // 入口函数
                Some(&[message]),           // 参数
            ).unwrap();
            PROCESSOR.get().add_thread(thread);
        }
    }

    PROCESSOR.unsafe_get().run();
    // 按照1讨论，应该这样
}

fn sample_process(message: usize) {
    for i in 0..1000000 {
        if i % 200000 == 0 {
            println!("thread {}", message);
        }
    }
}


```

尝试运行：

## 线程的结束

如果我们按照实验指导中的实现，应该会观察到：内核线程在运行完成后触发了 Exception::InstructionPageFault 而终止，其中访问的的地址 stval = 0。

### 解决办法

我们设计一个折衷的解决办法：内核线程将自己标记为“已结束”，同时触发一个普通的异常 ebreak。此时操作系统观察到线程的标记，便将其终止。

```rs

/// 内核线程需要调用这个函数来退出
fn kernel_thread_exit() {
    // 当前线程标记为结束
    PROCESSOR.get().current_thread().as_ref().inner().dead = true;
    // 制造一个中断来交给操作系统处理
    unsafe { llvm_asm!("ebreak" :::: "volatile") };
}

```

同时，在 os/src/main.rs 中：

```rs
// 设置线程的返回地址为 kernel_thread_exit
thread.as_ref().inner().context.as_mut().unwrap().set_ra(kernel_thread_exit as usize);

```

## 总结

本章做的事情：

- 理清线程和进程的概念
- 通过设置 Context，可以构造一个线程的初始状态
- 通过 __restore 标签，直接进入第一个线程之中
- 用 Context 来保存进程的状态，从而实现在时钟中断时切换线程
- 实现内核栈，提供安全的中断处理空间
- 实现调度器，完成线程的调度

相关数据结构：

- Processor
- Thread
- KernelStack