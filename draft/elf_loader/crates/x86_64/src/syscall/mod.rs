pub mod error;

pub mod syscall0;
pub mod syscall1;
pub mod syscall2;
pub mod syscall3;
pub mod syscall4;
pub mod syscall5;
pub mod syscall6;

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

pub use syscall0::syscall0;
pub use syscall1::syscall1;
pub use syscall2::syscall2;
pub use syscall3::syscall3;
pub use syscall4::syscall4;
pub use syscall5::syscall5;
pub use syscall6::syscall6;

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
