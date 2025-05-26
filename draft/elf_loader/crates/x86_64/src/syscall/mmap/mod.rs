pub mod flags;

use core::arch::x86_64;

use crate::syscall::error::*;
use crate::syscall::syscall6;
use crate::Syscall;

#[inline(always)]
pub unsafe fn mmap(
    addr: *mut u8,
    length: usize,
    prot: i32,
    flags: i32,
    fd: i32,
    offset: i64,
) -> Result<*mut u8> {
    let ret = syscall6(
        Syscall::MMap.to(),
        addr as usize,
        length,
        prot as usize,
        flags as usize,
        fd as usize,
        offset as usize,
    ) as i64;

    if ret < 0 {
        crate::print_str("Failed.");
        if let Some(error) = Error::from_syscall(ret) {
            Err(error)
        } else {
            Err(Error::EIO)
        }
    } else {
        Ok(ret as *mut u8)
    }
}
