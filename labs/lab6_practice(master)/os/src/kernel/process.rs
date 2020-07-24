//! 进程相关的内核功能

use super::*;

pub(super) fn sys_exit(code: usize) -> SyscallResult {
    println!(
        "thread {} exit with code {}",
        PROCESSOR.lock().current_thread().id,
        code
    );
    SyscallResult::Kill
}

pub(super) fn sys_get_tid() -> SyscallResult {
    SyscallResult::Proceed(PROCESSOR.lock().current_thread().id.clone())
}

pub(super) fn sys_clone(context: &Context) -> SyscallResult {
    let id = PROCESSOR.lock().current_thread().id.clone();
    PROCESSOR.lock().fork_current_thread(context);
    if PROCESSOR.lock().current_thread().id.clone() == id {
        SyscallResult::Proceed(id)
    } else {
        SyscallResult::Proceed(0)
    }
}
