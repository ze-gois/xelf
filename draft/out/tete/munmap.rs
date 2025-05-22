use crate::syscall::error::*;
use crate::syscall::syscall2;
use crate::Syscall;

#[inline(always)]
pub unsafe fn munmap(addr: *mut u8, length: usize) -> Result<()> {
    let ret = syscall2(Syscall::MUnmap.to(), addr as usize, length) as i64;

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
