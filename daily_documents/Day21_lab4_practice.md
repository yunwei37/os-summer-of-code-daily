# 实验四（上）：线程

1. 原理：线程切换之中，页表是何时切换的？页表的切换会不会影响程序 / 操作系统的运行？为什么？

    这里实际上和实验指导中是不同的；

    在实验指导中：

    os/src/process/thread.rs: impl Thread

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

    在 lab4 的分支代码中，应当是在 Process::prepare_next_thread() 中调用 Thread::prepare():

    ```rs
    /// 准备执行一个线程
    ///
    /// 激活对应进程的页表，并返回其 Context
    pub fn prepare(&self) -> *mut Context {
        // 激活页表
        self.process.write().memory_set.activate();
        // 取出 Context
        let parked_frame = self.inner().context.take().unwrap();
        // 将 Context 放至内核栈顶
        unsafe { KERNEL_STACK.push_context(parked_frame) }
    }
    ```

    它不会影响执行，因为在中断期间是操作系统正在执行，而操作系统所用到的内核线性映射是存在于每个页表中的。（这我记得重复了
    
    上下文 Context 的保存和取出：

    >思考：在 run 函数中，我们在一开始就激活了页表，会不会导致后续流程无法正常执行？
    >
    >不会，因为每一个进程的 MemorySet 都会映射操作系统的空间，否则在遇到中断的时候，将无法执行异常处理。

2. 设计：如果不使用 sscratch 提供内核栈，而是像原来一样，遇到中断就直接将上下文压栈，请举出（思路即可，无需代码）：

- 一种情况不会出现问题
- 一种情况导致异常无法处理（指无法进入 handle_interrupt）
- 一种情况导致产生嵌套异常（指第二个异常能够进行到调用 handle_interrupt，不考虑后续执行情况）
- 一种情况导致一个用户进程（先不考虑是怎么来的）可以将自己变为内核进程，或以内核态执行自己的代码

解答：

由于中断可能是多种原因导致的，也包含了不少错误等内容，因此可以从这些方向考虑：

- 只运行一个非常善意的线程，比如 loop {}：这种情况下和内核进程类似
- 线程把自己的 sp 搞丢了，比如 mv sp, x0。此时无法保存寄存器，也没有能够支持操作系统正常运行的栈：这种情况下出现错误之后就没有办法处理异常和恢复操作系统的状态，操作系统也会崩溃
- 运行两个线程。在两个线程切换的时候，会需要切换页表。但是此时操作系统运行在前一个线程的栈上，一旦切换，再访问栈就会导致缺页，因为每个线程的栈只在自己的页表中；
- 用户进程巧妙地设计 sp，使得它恰好落在内核的某些变量附近，于是在保存寄存器时就修改了变量的值。这相当于任意修改操作系统的控制信息；（这个回答我觉得和题目有点小关联不紧，不过可以提供思路）



1. 实验：当键盘按下 Ctrl + C 时，操作系统应该能够捕捉到中断。实现操作系统捕获该信号并结束当前运行的线程（你可能需要阅读一点在实验指导中没有提到的代码）

    这边有一个qemu特有的属性，需要按下 Ctrl + Alt + C 来在虚拟机中导入 Ctrl + C ，这个要注意一下；

    具体实现比较简单：

    在 `interrupt/handler.rs` 中，修改 `supervisor_external` 函数：

    ```rs
    /// 处理外部中断，只实现了键盘输入
    fn supervisor_external(context: &mut Context) -> Result<*mut Context, String> {
        let mut c = console_getchar();
        if c <= 255 {
            if c == 3 {
                PROCESSOR.get().kill_current_thread();
                PROCESSOR.get().prepare_next_thread();
            }else{
                if c == '\r' as usize {
                    c = '\n' as usize;
                }
                STDIN.push(c as u8);
            }
        }
        Ok(context)
    }


    ```

2. 实验：实现线程的 fork()。目前的内核线程不能进行系统调用，所以我们先简化地实现为“按 F 进入 fork”。fork 后应当为目前的线程复制一份几乎一样的拷贝，新线程与旧线程同属一个进程，公用页表和大部分内存空间，而新线程的栈是一份拷贝。

代码：

handler.rs

```rs

/// 处理外部中断，只实现了键盘输入
fn supervisor_external(context: &mut Context) -> Result<*mut Context, String> {
    let mut c = console_getchar();
    if c <= 255 {
        if c == 3 {
            PROCESSOR.get().kill_current_thread();
            PROCESSOR.get().prepare_next_thread();
        }
        else if c == 'f' as usize {
            PROCESSOR.get().fork_current_thread(context);
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

```

processsor.rs

```rs

    /// 线程的 fork()
    /// fork 后应当为目前的线程复制一份几乎一样的拷贝，新线程与旧线程同属一个进程，公用页表和大部分内存空间，而新线程的栈是一份拷贝。
    pub fn fork_current_thread(&mut self, context: &Context){
        let thread = self.current_thread().fork(*context).unwrap();
        self.add_thread(thread);
    }

```

thread.rs

```rs

pub fn fork(&self, current_context: Context) -> MemoryResult<Arc<Thread>> {

        println!("enter fork");

        // 让所属进程分配并映射一段空间，作为线程的栈
        let stack = self.process
            .write()
            .alloc_page_range(STACK_SIZE, Flags::READABLE | Flags::WRITABLE)?;

        for i in 0..STACK_SIZE {
            *VirtualAddress(stack.start.0 + i).deref::<u8>() = *VirtualAddress(self.stack.start.0 + i).deref::<u8>()
        }

        let mut context = current_context.clone();

        context.set_sp( usize::from(stack.start) -  usize::from(self.stack.start) + current_context.sp()  );

        // 打包成线程
        let thread = Arc::new(Thread {
            id: unsafe {
                THREAD_COUNTER += 1;
                THREAD_COUNTER
            },
            stack,
            process: Arc::clone(&self.process),
            inner: Mutex::new(ThreadInner {
                context: Some(context),
                sleeping: false,
                descriptors: vec![STDIN.clone(), STDOUT.clone()],
            }),
        });

        Ok(thread)
    }

```

测试用户程序：

```rs

pub fn main() -> usize {
    println!("Hello world from user mode program!");
    for i in 0..10000000{
        if i%1000000 == 0 {
            println!("Hello world from user mode program!{}",i);
        }
    }
    0
}

```

输出：

```sh

Hello world from user mode program!6000000
Hello world from user mode program!7000000
Hello world from user mode program!6000000
Hello world from user mode program!8000000
Hello world from user mode program!7000000
Hello world from user mode program!9000000
Thread 3 exit with code 0
Hello world from user mode program!8000000
Hello world from user mode program!9000000
Thread 2 exit with code 0
src/process/processor.rs:101: 'all threads terminated, shutting down'
make[1]: Leaving directory '/home/yunwei/rCore-Tutorial-lab-4/os'

```