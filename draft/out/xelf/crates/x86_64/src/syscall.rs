pub mod error;

pub mod close;
pub mod exit;
pub mod lseek;
pub mod mmap;
pub mod mprotect;
pub mod munmap;
pub mod openat;
pub mod prctl;
pub mod read;
pub mod write;

pub use close::close;
pub use exit::exit;
pub use lseek::lseek;
pub use mmap::mmap;
pub use mprotect::mprotect;
pub use munmap::munmap;
pub use openat::openat;
pub use openat::openat4;
pub use prctl::prctl;
pub use read::read;
pub use write::write;

#[repr(usize)]
pub enum Syscall {
    Read = 0,
    Write = 1,
    Close = 3,
    Lseek = 8,
    MMap = 9,
    MProtect = 10,
    MUnmap = 11,
    Exit = 60,
    Prctl = 157,
    Openat = 257,
}

impl Syscall {
    pub fn from(n: usize) -> Option<Syscall> {
        match n {
            0 => Some(Syscall::Read),
            1 => Some(Syscall::Write),
            3 => Some(Syscall::Close),
            8 => Some(Syscall::Lseek),
            9 => Some(Syscall::MMap),
            10 => Some(Syscall::MProtect),
            11 => Some(Syscall::MUnmap),
            60 => Some(Syscall::Exit),
            157 => Some(Syscall::Prctl),
            257 => Some(Syscall::Openat),
            _ => None,
        }
    }

    pub fn to(self) -> usize {
        self as usize
    }
}

#[inline(always)]
pub unsafe fn syscall0(n: usize) -> usize {
    let ret: usize;
    core::arch::asm!(
        "syscall",
        in("rax") n,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall1(n: usize, a1: usize) -> usize {
    let ret: usize;
    core::arch::asm!(
        "syscall",
        in("rax") n,
        in("rdi") a1,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> usize {
    let ret: usize;
    core::arch::asm!(
        "syscall",
        in("rax") n,
        in("rdi") a1,
        in("rsi") a2,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> usize {
    let ret: usize;
    core::arch::asm!(
        "syscall",
        in("rax") n,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> usize {
    let ret: usize;
    core::arch::asm!(
        "syscall",
        in("rax") n,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall5(n: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> usize {
    let ret: usize;
    core::arch::asm!(
        "syscall",
        in("rax") n,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        in("r8") a5,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall6(
    n: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> usize {
    let ret: usize;
    core::arch::asm!(
        "syscall",
        in("rax") n,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        in("r8") a5,
        in("r9") a6,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
    );
    ret
}
