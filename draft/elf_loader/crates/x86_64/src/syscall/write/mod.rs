use crate::syscall::error::*;
use crate::syscall::syscall3;
use crate::Syscall;

#[inline(always)]
pub unsafe fn write(fd: i32, buf: *const u8, count: usize) -> Result<usize> {
    let ret = syscall3(Syscall::Write.to(), fd as usize, buf as usize, count) as i64;

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
