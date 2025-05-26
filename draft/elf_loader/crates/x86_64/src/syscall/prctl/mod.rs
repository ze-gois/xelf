use super::error::*;
use super::{syscall5, Syscall};

pub const PR_SET_MM: i32 = 39;
pub const MM_MAP_MIN_ADDR: usize = 6;

pub unsafe fn prctl(
    option: i32,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> Result<i32> {
    Ok(syscall5(Syscall::Prctl.to(), option as usize, arg2, arg3, arg4, arg5) as i32)
}
