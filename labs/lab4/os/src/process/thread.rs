//! 线程 [`Thread`]

use super::*;
use core::hash::{Hash, Hasher};
use core::mem::size_of;

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
    pub context: Mutex<Option<Context>>,
    pub dead: bool,
}

/// 线程中需要可变的部分
pub struct ThreadInner {
    /// 线程执行上下文
    ///
    /// 当且仅当线程被暂停执行时，`context` 为 `Some`
    pub context: Option<Context>,
    /// 是否进入休眠
    pub sleeping: bool,
}

impl Thread {
    /// 准备执行一个线程
    ///
    /// 激活对应进程的页表，并返回其 Context
    pub fn prepare(&self) -> *mut Context {
        // 激活页表
        self.process.write().memory_set.activate();
        // 取出 Context
        let parked_frame = self.context.lock().take().unwrap();
        // 将 Context 放至内核栈顶
        unsafe { KERNEL_STACK.push_context(parked_frame) }
    }
    /// 发生时钟中断后暂停线程，保存状态
    pub fn park(&self, context: Context) {
        // 检查目前线程内的 context 应当为 None
        let mut slot = self.context.lock();
        assert!(slot.is_none());
        // 将 Context 保存到线程中
        slot.replace(context);
    }
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
            unsafe { KERNEL_STACK.push_context(parked_frame) }
        } else {
            // 内核线程则将 Context 放至 sp 下
            let context = (parked_frame.sp() - size_of::<Context>()) as *mut Context;
            unsafe { *context = parked_frame };
            context
        }
    }

    pub fn new(
        process: Arc<RwLock<Process>>,
        entry_point: usize,
        arguments: Option<&[usize]>,
    ) -> MemoryResult<Arc<Thread>> {
        // 让所属进程分配并映射一段空间，作为线程的栈
        let stack = process
            .write()
            .alloc_page_range(STACK_SIZE, Flags::READABLE | Flags::WRITABLE)?;

        // 构建线程的 Context
        let context = Context::new(
            stack.end.into(),
            entry_point,
            arguments,
            process.read().is_user,
        );

        // 打包成线程
        let thread = Arc::new(Thread {
            id: unsafe {
                THREAD_COUNTER += 1;
                THREAD_COUNTER
            },
            stack,
            process,

            context: Mutex::new(Some(context)),
            dead: false,
        });

        Ok(thread)
    }
}

/// 通过线程 ID 来判等
impl PartialEq for Thread {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// 通过线程 ID 来判等
///
/// 在 Rust 中，[`PartialEq`] trait 不要求任意对象 `a` 满足 `a == a`。
/// 将类型标注为 [`Eq`]，会沿用 `PartialEq` 中定义的 `eq()` 方法，
/// 同时声明对于任意对象 `a` 满足 `a == a`。
impl Eq for Thread {}

/// 通过线程 ID 来哈希
impl Hash for Thread {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_isize(self.id);
    }
}
