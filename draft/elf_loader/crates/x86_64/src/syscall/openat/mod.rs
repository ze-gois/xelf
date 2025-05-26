pub mod flags;

use crate::syscall::error::*;
use crate::syscall::syscall3;
use crate::Syscall;

#[inline(always)]
pub unsafe fn openat(dirfd: i32, pathname: *const u8, flags: i32) -> Result<i32> {
    let ret = syscall3(
        Syscall::Openat.to(),
        dirfd as usize,
        pathname as usize,
        flags as usize,
    ) as i64;

    if ret < 0 {
        if let Some(error) = Error::from_syscall(ret) {
            Err(error)
        } else {
            Err(Error::EIO)
        }
    } else {
        Ok(ret as i32)
    }
}

use crate::syscall::syscall4;

pub unsafe fn openat4(dirfd: i32, pathname: *const u8, flags: i32, mode: i32) -> Result<i32> {
    let ret = syscall4(
        Syscall::Openat.to(),
        dirfd as usize,
        pathname as usize,
        flags as usize,
        mode as usize,
    ) as i64;

    if ret < 0 {
        if let Some(error) = Error::from_syscall(ret) {
            Err(error)
        } else {
            Err(Error::EIO)
        }
    } else {
        Ok(ret as i32)
    }
}
