pub mod flags;

use crate::syscall::error::*;
use crate::syscall::syscall3;
use crate::Syscall;

#[inline(always)]
pub unsafe fn lseek(fd: i32, offset: i64, whence: i32) -> Result<i64> {
    let ret = syscall3(
        Syscall::Lseek.to(),
        fd as usize,
        offset as usize,
        whence as usize,
    ) as i64;

    if ret < 0 {
        if let Some(error) = Error::from_syscall(ret) {
            Err(error)
        } else {
            Err(Error::EIO)
        }
    } else {
        Ok(ret)
    }
}
