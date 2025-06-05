use super::Number;
use arch::{Arch, Callable};

static NUMBER: usize = Number::MUnMap as usize;

define_syscall_error!(
    Error,
    MUnMap,
    "munmap",
    [[NotReadable, -13, "File not open for reading", EACCES]]
);

#[inline(always)]
pub fn munmap(addr: *mut u8, length: usize) -> crate::Result<isize> {
    let arch_result = Arch::syscall2(NUMBER, addr as usize, length);
    handle_result(arch_result)
}
