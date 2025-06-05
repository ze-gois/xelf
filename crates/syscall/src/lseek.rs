pub mod flags;

pub use flags::Flag;

use super::Number;
use arch::{Arch, Callable};

static NUMBER: usize = Number::LSeek as usize;

define_syscall_error!(
    Error,
    LSeek,
    "lseek",
    [[BadFileDescriptor, -9, "Bad file descriptor", EBADF],]
);

#[inline(always)]
pub fn lseek(fd: i32, offset: i64, whence: i32) -> crate::result::Result<isize> {
    let arch_result = Arch::syscall3(NUMBER, fd as usize, offset as usize, whence as usize);

    handle_result(arch_result)
}
