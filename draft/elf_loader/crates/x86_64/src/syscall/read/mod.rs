use crate::syscall::error::*;
use crate::syscall::syscall3;
use crate::Syscall;

#[inline(always)]
pub unsafe fn read(fd: i32, buf: *mut u8, count: usize) -> Result<usize> {
    let ret = syscall3(Syscall::Read.to(), fd as usize, buf as usize, count) as i64;

    if ret < 0 {
        if let Some(error) = Error::from_syscall(ret) {
            Err(error)
        } else {
            Err(Error::EIO)
        }
    } else {
        Ok(ret as usize)
    }
}
