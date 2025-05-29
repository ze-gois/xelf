use crate::Arch;
use crate::result::Result;
use crate::traits::callable::Callable;

pub mod prot {
    pub const NONE: i32 = 0x0;
    pub const READ: i32 = 0x1;
    pub const WRITE: i32 = 0x2;
    pub const EXEC: i32 = 0x4;
}

pub mod flag {
    pub const SHARED: i32 = 0x01;
    pub const PRIVATE: i32 = 0x02;
    pub const FIXED: i32 = 0x10;
    pub const ANONYMOUS: i32 = 0x20;
}
// syscall numbers
const SYS_MMAP: usize = 9;
const SYS_MUNMAP: usize = 11;

#[inline(always)]
pub unsafe fn mmap(
    addr: *mut u8,
    length: usize,
    prot: i32,
    flags: i32,
    fd: i32,
    offset: i64,
) -> Result<*mut u8> {
    let ret = Arch::syscall6(
        SYS_MMAP,
        addr as usize,
        length,
        prot as usize,
        flags as usize,
        fd as usize,
        offset as usize,
    )?;

    Ok(ret as *mut u8)
}

#[inline(always)]
pub unsafe fn munmap(addr: *mut u8, length: usize) -> Result<()> {
    let _ = Arch::syscall2(SYS_MUNMAP, addr as usize, length)?;

    Ok(())
}
