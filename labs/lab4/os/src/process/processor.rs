use super::*;
use algorithm::*;
use lazy_static::*;
use hashbrown::HashSet;

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

impl Processor {
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

}