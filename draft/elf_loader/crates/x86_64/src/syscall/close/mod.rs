use crate::syscall::error::*;
use crate::syscall::syscall1;
use crate::Syscall;

#[inline(always)]
pub unsafe fn close(fd: i32) -> Result<()> {
    let ret = syscall1(Syscall::Close.to(), fd as usize) as i64;

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
