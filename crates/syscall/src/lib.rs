#![no_std]
pub mod result;
pub use result::*;

#[macro_use]
pub mod macros;

pub mod close;
pub mod exit;
pub mod lseek;
pub mod mmap;
pub mod mprotect;
pub mod munmap;
pub mod open;
pub mod read;
pub mod write;

pub use close::close;
pub use exit::exit;
pub use lseek::lseek;
pub use mmap::mmap;
pub use mprotect::mprotect;
pub use munmap::munmap;
pub use open::{openat, openat4};
pub use read::read;
pub use write::write;

pub enum Number {
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
    Stat = 4,
    FStat = 5,
    LStat = 6,
    Poll = 7,
    LSeek = 8,
    MMap = 9,
    MProtect = 10,
    MUnMap = 11,
    Brk = 12,
    Exit = 60,
    OpenAt = 257,
}

impl Number {
    pub fn from(n: usize) -> Option<Number> {
        match n {
            60 => Some(Number::Exit),
            1 => Some(Number::Write),
            257 => Some(Number::OpenAt),
            _ => None,
        }
    }
}

impl Into<usize> for Number {
    fn into(self) -> usize {
        self as usize
    }
}
