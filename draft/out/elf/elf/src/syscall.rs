pub mod error;

// pub mod at;

// pub mod close;
pub mod exit;
// pub mod lseek;
// pub mod mmap;
// pub mod mprotect;
// pub mod munmap;
// pub mod open;
// pub mod openat;
// pub mod read;
pub mod write;

pub use error::*;

// pub use close::close;
pub use exit::exit;
// pub use lseek::lseek;
// pub use mmap::mmap;
// pub use mprotect::mprotect;
// pub use munmap::munmap;
// pub use open::open;
// pub use openat::openat;
// pub use read::read;
pub use write::write;

use crate::ErrorTrait;

pub fn wrap(result: isize) -> crate::Result<isize> {
    if result < 0 {
        Err(Error::from_no(-result))
    } else {
        Ok(result)
    }
}

pub enum Number {
    Close = 0x003,
    Exit = 0x03c,
    LSeek = 0x008,
    MMap = 0x009,
    MProtect = 0x00a,
    MUnMap = 0x00b,
    Open = 0x002,
    OpenAt = 0x101,
    Read = 0x000,
    Write = 0x001,
}
