use crate::dtype2::Error as DTypeError;
use human::result::Error as HumanError;
use syscall::result::Error as SyscallError;

#[repr(isize)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    Syscall(SyscallError),
    Human(HumanError),
    DType(DTypeError),
    TODO,
}

impl result::ErrorTrait for Error {
    fn from_no(errno: isize) -> Error {
        match -errno {
            _ => Error::TODO,
        }
    }

    fn describe(&self) -> &str {
        match self {
            _ => "TODO",
        }
    }

    fn advert(&self) -> Option<isize> {
        match self {
            _ => None,
        }
    }
}

impl Into<isize> for Error {
    fn into(self) -> isize {
        match self {
            Error::Human(_e) => -4,
            Error::Syscall(_e) => -2,
            Error::DType(_e) => -3,
            Error::TODO => -1,
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
