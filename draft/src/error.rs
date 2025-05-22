use crate::arch;
use crate::human;
use crate::syscall;

pub trait ErrorTrait {
    fn from_no(errno: isize) -> Error;
    fn describe(&self) -> &str;
    fn advert(&self) -> Option<isize>;
}

#[repr(isize)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    Arch(arch::Error),
    Syscall(syscall::Error),
    Human(human::Error),
    TODO,
}

impl ErrorTrait for Error {
    fn from_no(errno: isize) -> Error {
        match -errno {
            _ => Error::TODO,
        }
    }

    fn describe(&self) -> &str {
        match self {
            Self::Arch(e) => e.describe(),
            Self::Syscall(e) => e.describe(),
            _ => "TODO",
        }
    }

    fn advert(&self) -> Option<isize> {
        match self {
            Self::Arch(e) => e.advert(),
            Self::Syscall(e) => e.advert(),
            _ => None,
        }
    }
}

impl Into<isize> for Error {
    fn into(self) -> isize {
        match self {
            Error::Human(_e) => -4,
            Error::Arch(_e) => -3,
            Error::Syscall(_e) => -2,
            Error::TODO => -1,
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;

// impl<T> Result<T> {
//     pub fn or_eject(self) -> isize {
//         match self {
//             Ok(_) => 0,
//             Err(e) => crate::syscall::exit(6),
//         }
//     }
// }
