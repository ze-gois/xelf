pub mod flags;

use crate::syscall::error::*;
use crate::syscall::syscall3;
use crate::Syscall;

#[inline(always)]
pub unsafe fn mprotect(addr: *mut u8, len: usize, prot: i32) -> Result<()> {
    let ret = syscall3(Syscall::MProtect.to(), addr as usize, len, prot as usize) as i64;

    if ret < 0 {
        if let Some(error) = Error::from_syscall(ret) {
            Err(error)
        } else {
            Err(Error::EIO)
        }
    } else {
        Ok(())
    }
}
