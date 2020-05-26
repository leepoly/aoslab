//! 进程相关的内核功能

use super::*;

pub(super) fn sys_exit(code: usize) -> SyscallResult {
    println!("Thread {} exit with code {}", PROCESSOR.get().current_thread().id, code);
    SyscallResult::Kill
}

pub(super) fn sys_exec(_path: *const u8) -> SyscallResult {
    let thread = PROCESSOR.get().current_thread();
    // start_user_thread(from_cstr(path)); // todo: use start_user_thread to enable new threads
    PROCESSOR.get().park_current_thread(&thread.inner().context.unwrap());
    PROCESSOR.get().prepare_next_thread();
    SyscallResult::Proceed(0)
}