use super::context::Context;
use super::timer;
use crate::fs::STDIN;
use crate::kernel::syscall_handler;
use crate::memory::*;
use crate::process::PROCESSOR;
use crate::sbi::console_getchar;
use alloc::{format, string::String};
use riscv::register::{
    scause::{Exception, Interrupt, Scause, Trap},
    sie, stvec,
};

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

        // 开启外部中断使能
        sie::set_sext();

        // 在 OpenSBI 中开启外部中断
        *PhysicalAddress(0x0c00_2080).deref_kernel() = 1u32 << 10;
        // 在 OpenSBI 中开启串口
        *PhysicalAddress(0x1000_0004).deref_kernel() = 0x0bu8;
        *PhysicalAddress(0x1000_0001).deref_kernel() = 0x01u8;
        // 其他一些外部中断相关魔数
        *PhysicalAddress(0x0C00_0028).deref_kernel() = 0x07u32;
        *PhysicalAddress(0x0C20_1000).deref_kernel() = 0u32;
    }
}

/// 中断的处理入口
///
/// `interrupt.asm` 首先保存寄存器至 Context，其作为参数和 scause 以及 stval 一并传入此函数
/// 具体的中断类型需要根据 scause 来推断，然后分别处理
#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) -> *mut Context {
    // 返回的 Context 必须位于放在内核栈顶
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
        _ => Err(format!(
            "unimplemented interrupt type: {:x?}",
            scause.cause()
        )),
    }
    .unwrap_or_else(|msg| fault(msg, scause, stval))
}

/// 处理 ebreak 断点
///
/// 继续执行，其中 `sepc` 增加 2 字节，以跳过当前这条 `ebreak` 指令
fn breakpoint(context: &mut Context) -> Result<*mut Context, String> {
    println!("Breakpoint at 0x{:x}", context.sepc);
    context.sepc += 2;
    Ok(context)
}

/// 处理时钟中断
fn supervisor_timer(context: &mut Context) -> Result<*mut Context, String> {
    timer::tick();
    PROCESSOR.get().park_current_thread(context);
    //println!("change");
    Ok(PROCESSOR.get().prepare_next_thread())
}

/// 处理外部中断，只实现了键盘输入
fn supervisor_external(context: &mut Context) -> Result<*mut Context, String> {
    let mut c = console_getchar();
    if c <= 255 {
        if c == 3 {
            PROCESSOR.get().kill_current_thread();
            PROCESSOR.get().prepare_next_thread();
        }
        else if c == 'f' as usize {
            //println!("{:?}",context);
            PROCESSOR.get().fork_current_thread(context);
            //println!("{:?}",context);
        }
        else{
            if c == '\r' as usize {
                c = '\n' as usize;
            }
            STDIN.push(c as u8);
        }
    }
    Ok(context)
}

/// 出现未能解决的异常，终止当前线程
fn fault(msg: String, scause: Scause, stval: usize) -> *mut Context {
    println!(
        "{:#x?} terminated: {}",
        PROCESSOR.get().current_thread(),
        msg
    );
    println!("cause: {:?}, stval: {:x}", scause.cause(), stval);

    PROCESSOR.get().kill_current_thread();
    // 跳转到 PROCESSOR 调度的下一个线程
    PROCESSOR.get().prepare_next_thread()
}
