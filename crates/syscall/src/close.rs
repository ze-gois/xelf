use super::Number;
use arch::{Arch, Callable};

static NUMBER: usize = Number::Close as usize;

define_syscall_error!(
    Error,
    Close,
    "close",
    [[NotReadable, -13, "File not open for reading", EACCES]]
);

#[inline(always)]
pub fn close(fd: i32) -> crate::Result<isize> {
    let arch_result = Arch::syscall1(NUMBER, fd as usize);
    handle_result(arch_result)
}
