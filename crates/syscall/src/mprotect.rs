pub mod flags;

use super::Number;
use arch::{Arch, Callable};

static NUMBER: usize = Number::MProtect as usize;

define_syscall_error!(
    Error,
    MProtect,
    "mprotect",
    [[NotReadable, -13, "File not open for reading", EACCES]]
);

#[inline(always)]
pub fn mprotect(addr: *mut u8, len: usize, prot: i32) -> crate::Result<isize> {
    let arch_result = Arch::syscall3(NUMBER, addr as usize, len, prot as usize);
    handle_result(arch_result)
}
