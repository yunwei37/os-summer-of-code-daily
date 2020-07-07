# lab1 学习报告

- RISC-V 中有关中断处理的寄存器和相关流程
- 如何保存上下文，使得中断处理流程前后，原本正在执行的程序感知不到发生了中断
- 处理最简单的断点中断和时钟中断

## 中断的概念：

- 异常：执行指令时产生的，通常无法预料的错误。
- 陷阱：一系列强行导致中断的指令
- 硬件中断：由 CPU 之外的硬件产生的异步中断

## RISC-V 与中断相关的寄存器和指令

Machine mode：

- 是 RISC-V 中的最高权限模式，一些底层操作的指令只能由机器态进行使用。
- 是所有标准 RISC-V 处理器都必须实现的模式。
- 默认所有中断实际上是交给机器态处理的，但是为了实现更多功能，机器态会将某些中断交由内核态处理。这些异常也正是我们编写操作系统所需要实现的。

Supervisor mode：

- 通常为操作系统使用，可以访问一些 supervisor 级别的寄存器，通过这些寄存器对中断和虚拟内存映射进行管理。
- Unix 系统中，大部分的中断都是内核态的系统调用。机器态可以通过异常委托机制（machine interrupt delegation）将一部分中断设置为不经过机器态，直接由内核态处理

关注：内核态可以使用的一些特权指令和寄存器

### 与中断相关的寄存器

自动填写：

- sepc：用来记录触发中断的指令的地址。
- scause：记录中断是否是硬件中断，以及具体的中断原因。
- stval：需要访问但是不在内存中的地址，包含信息

指导硬件处理中断：

- stvec：设置内核态中断处理流程的入口地址。分为基址 BASE 和模式 MODE：
  - MODE 为 0 表示 Direct 模式：跳转至 BASE 进行执行
  - MODE 为 1 表示 Vectored 模式：BASE + 4 * cause
- sstatus：控制全局中断使能等
- sie：控制具体类型中断的使能
- sip：记录每种中断是否被触发
- sscratch：sscratch 在用户态保存内核栈的地址

### 与中断相关的指令

- ecall：触发中断
- sret：从内核态返回用户态，同时将 pc 的值设置为 sepc
- ebreak：触发一个断点
- mret：从机器态返回内核态
- csrrw dst, csr, src：CSR Read Write
- csrr dst, csr：CSR Write
- csrc(i) csr, rs1：CSR Clear
- csrs(i) csr, rs1：CSR Set
  
## 程序运行状态

在处理中断之前，必须要保存所有可能被修改的寄存器，并且在处理完成后恢复。我们需要保存所有通用寄存器，sepc、scause 和 stval 这三个会被硬件自动写入的 CSR 寄存器，以及 sstatus。

在 os/src/interrupt/context.rs 中添加：

```rs
use riscv::register::{sstatus::Sstatus, scause::Scause};

#[repr(C)]
pub struct Context {
    pub x: [usize; 32],     // 32 个通用寄存器
    pub sstatus: Sstatus,
    pub sepc: usize
}

```

在 os/Cargo.toml 中添加依赖:

```
[dependencies]
riscv = { git = "https://github.com/rcore-os/riscv", features = ["inline-asm"] }

```

## 状态的保存与恢复

- 保存：先用栈上的一小段空间来把需要保存的全部通用寄存器和 CSR 寄存器保存在栈上，保存完之后在跳转到 Rust 编写的中断处理函数；
- 恢复：直接把备份在栈上的内容写回寄存器。由于涉及到了寄存器级别的操作，我们需要用汇编来实现。

新建：

os/src/interrupt.asm

```asm
# 宏：将寄存器存到栈上
.macro SAVE reg, offset
    sd  \reg, \offset*8(sp)
.endm

# 宏：将寄存器从栈中取出
.macro LOAD reg, offset
    ld  \reg, \offset*8(sp)
.endm

    .section .text
    .globl __interrupt
# 进入中断
# 保存 Context 并且进入 rust 中的中断处理函数 interrupt::handler::handle_interrupt()
__interrupt:
    # 在栈上开辟 Context 所需的空间
    addi    sp, sp, -34*8
    
    # 保存通用寄存器，除了 x0（固定为 0）
    SAVE    x1, 1
    # 将原来的 sp（sp 又名 x2）写入 2 位置
    addi    x1, sp, 34*8
    SAVE    x1, 2
    # 其他通用寄存器
    SAVE    x3, 3
    SAVE    x4, 4
    SAVE    x5, 5
    SAVE    x6, 6
    SAVE    x7, 7
    SAVE    x8, 8
    SAVE    x9, 9
    SAVE    x10, 10
    SAVE    x11, 11
    SAVE    x12, 12
    SAVE    x13, 13
    SAVE    x14, 14
    SAVE    x15, 15
    SAVE    x16, 16
    SAVE    x17, 17
    SAVE    x18, 18
    SAVE    x19, 19
    SAVE    x20, 20
    SAVE    x21, 21
    SAVE    x22, 22
    SAVE    x23, 23
    SAVE    x24, 24
    SAVE    x25, 25
    SAVE    x26, 26
    SAVE    x27, 27
    SAVE    x28, 28
    SAVE    x29, 29
    SAVE    x30, 30
    SAVE    x31, 31

    # 取出 CSR 并保存
    csrr    s1, sstatus
    csrr    s2, sepc
    SAVE    s1, 32
    SAVE    s2, 33

    # 调用 handle_interrupt，传入参数
    # context: &mut Context
    mv      a0, sp
    # scause: Scause
    csrr    a1, scause
    # stval: usize
    csrr    a2, stval
    jal  handle_interrupt

    .globl __restore
# 离开中断
# 从 Context 中恢复所有寄存器，并跳转至 Context 中 sepc 的位置
__restore:
    # 恢复 CSR
    LOAD    s1, 32
    LOAD    s2, 33
    csrw    sstatus, s1
    csrw    sepc, s2

    # 恢复通用寄存器
    LOAD    x1, 1
    LOAD    x3, 3
    LOAD    x4, 4
    LOAD    x5, 5
    LOAD    x6, 6
    LOAD    x7, 7
    LOAD    x8, 8
    LOAD    x9, 9
    LOAD    x10, 10
    LOAD    x11, 11
    LOAD    x12, 12
    LOAD    x13, 13
    LOAD    x14, 14
    LOAD    x15, 15
    LOAD    x16, 16
    LOAD    x17, 17
    LOAD    x18, 18
    LOAD    x19, 19
    LOAD    x20, 20
    LOAD    x21, 21
    LOAD    x22, 22
    LOAD    x23, 23
    LOAD    x24, 24
    LOAD    x25, 25
    LOAD    x26, 26
    LOAD    x27, 27
    LOAD    x28, 28
    LOAD    x29, 29
    LOAD    x30, 30
    LOAD    x31, 31

    # 恢复 sp（又名 x2）这里最后恢复是为了上面可以正常使用 LOAD 宏
    LOAD    x2, 2
    sret

```

进入中断处理流程:

新建 os/src/interrupt/handler.rs

```rs
use super::context::Context;
use riscv::register::stvec;

global_asm!(include_str!("./interrupt.asm"));

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
    }
}

/// 中断的处理入口
/// 
/// `interrupt.asm` 首先保存寄存器至 Context，其作为参数和 scause 以及 stval 一并传入此函数
/// 具体的中断类型需要根据 scause 来推断，然后分别处理
#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
    panic!("Interrupted: {:?}", scause.cause());
}
```

os/src/interrupt/mod.rs：

```rs
//! 中断模块
//! 
//! 

mod handler;
mod context;

/// 初始化中断相关的子模块
/// 
/// - [`handler::init`]
/// - [`timer::init`]
pub fn init() {
    handler::init();
    println!("mod interrupt initialized");
}
```

编译：`cargo build`

报错:
`error[E0412]: cannot find type `Scause` in this scope`

在 handler.rs 中添加：

`use riscv::register::scause::Scause;`

运行：

```
PMP0: 0x0000000080000000-0x000000008001ffff (A)
PMP1: 0x0000000000000000-0xffffffffffffffff (A,R,W,X)
Hello rCore-Tutorial!
mod interrupt initialized
panic: 'Interrupted: Exception(Breakpoint)'

```

## 时钟中断

RISC-V 中将中断分为三种：

- 软件中断（Software Interrupt）
- 时钟中断（Timer Interrupt）
- 外部中断（External Interrupt）

新建：os/src/interrupt/timer.rs

```rs
//! 预约和处理时钟中断

use crate::sbi::set_timer;
use riscv::register::{time, sie, sstatus};

/// 初始化时钟中断
/// 
/// 开启时钟中断使能，并且预约第一次时钟中断
pub fn init() {
    unsafe {
        // 开启 STIE，允许时钟中断
        sie::set_stimer(); 
        // 开启 SIE（不是 sie 寄存器），允许内核态被中断打断
        sstatus::set_sie();
    }
    // 设置下一次时钟中断
    set_next_timeout();
}

```

设置时钟中断:

os/src/sbi.rs

```rs
/// 时钟中断的间隔，单位是 CPU 指令
static INTERVAL: usize = 100000;

/// 设置下一次时钟中断
/// 
/// 获取当前时间，加上中断间隔，通过 SBI 调用预约下一次中断
fn set_next_timeout() {
    set_timer(time::read() + INTERVAL);
}

/// 触发时钟中断计数
pub static mut TICKS: usize = 0;

/// 每一次时钟中断时调用
/// 
/// 设置下一次时钟中断，同时计数 +1
pub fn tick() {
    set_next_timeout();
    unsafe {
        TICKS += 1;
        if TICKS % 100 == 0 {
            println!("{} tick", TICKS);
        }
    }
}

```

### 实现时钟中断的处理流程

os/src/interrupt/handler.rs:

```rs
/// 中断的处理入口
/// 
/// `interrupt.asm` 首先保存寄存器至 Context，其作为参数和 scause 以及 stval 一并传入此函数
/// 具体的中断类型需要根据 scause 来推断，然后分别处理
#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
    match scause.cause() {
        // 断点中断（ebreak）
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        // 时钟中断
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        // 其他情况，终止当前线程
        _ => fault(context, scause, stval),
    }
}

/// 处理 ebreak 断点
/// 
/// 继续执行，其中 `sepc` 增加 2 字节，以跳过当前这条 `ebreak` 指令
fn breakpoint(context: &mut Context) {
    println!("Breakpoint at 0x{:x}", context.sepc);
    context.sepc += 2;
}

/// 处理时钟中断
/// 
/// 目前只会在 [`timer`] 模块中进行计数
fn supervisor_timer(_: &Context) {
    timer::tick();
}

/// 出现未能解决的异常
fn fault(context: &mut Context, scause: Scause, stval: usize) {
    panic!(
        "Unresolved interrupt: {:?}\n{:x?}\nstval: {:x}",
        scause.cause(),
        context,
        stval
    );
}

```

还需要在头部加上：

```rs
use super::timer;
use riscv::register::{
    scause::{Exception, Interrupt, Scause, Trap},
    sie, stvec,
};

```

最后：

- 给 Context 结构体加上 Debug trait；
- 在 os/interrupt/mod.rs 中引入 mod timer 并在 初始化 handler::init() 语句的后面加入 timer::init()；
- 在 main.rs 中间引入 loop{}; 循环

就能跑起来啦！

## practice（看起来是新加的

关于实验题：

1. 简述：在 rust_main 函数中，执行 ebreak 命令后至函数结束前，sp 寄存器的值是怎样变化的？
2. 回答：lab1 分支中的 rust_main 函数存在什么问题？它会导致什么结果？
3. 实验 
   1. 如果程序访问不存在的地址，会得到 Exception::LoadFault。模仿捕获 ebreak 和时钟中断的方法，捕获 LoadFault（之后 panic 即可）。
   2. 在处理异常的过程中，如果程序想要非法访问的地址是 `0x0`，则打印 `SUCCESS!`。
   3. 添加或修改少量代码，使得运行时触发这个异常，并且打印出 `SUCCESS!`。

answer：

（由于practice里面已经有答案了，这里比较简略）

1. sp -34*8; 在中断处理完成后恢复；中间不变
2. 不结束
3. 实验：

handler.rs

```rs
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
    context.sepc = 0;
    match scause.cause() {
        // 断点中断（ebreak）
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        Trap::Exception(Exception::LoadFault) => loadfault(context,stval),
        // 时钟中断
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        // 其他情况，终止当前线程
        _ => fault(context, scause, stval),
    }
}

...

/// load
/// 
fn loadfault(context: &mut Context, stval: usize){
    if stval == 0 {
        println!("SUCCESS!");
    }
    panic!(
        "loadfault interrupt: {:x?}\nstval: {:x}",
        context,
        stval
    );
}

```

解法1，解法2. 已经有说明；

这里说一个解法三：

在 handle_interrupt 函数开头或结尾结尾加上：

`context.sepc = 0`

估计由于qemu底下是按四字节对齐访问触发异常的，因此breakpoint 那个 +2 可以忽略不计，只有 4 的倍数才会起作用；只要在错误流程处理的任何一个地方加上这句话或者类似的都能起到效果。