# 实验一：中断

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