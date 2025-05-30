use result::ErrorTrait;

use crate::open::Error as OpenError;
use crate::read::Error as ReadError;
use crate::write::Error as WriteError;

#[repr(isize)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    Open(OpenError),
    Read(ReadError),
    Write(WriteError),
    TODO,
}

impl ErrorTrait for Error {
    fn from_no(errno: isize) -> Self {
        match -errno {
            _ => Self::TODO,
        }
    }

    fn describe(&self) -> &str {
        match self {
            Error::Open(err) => err.describe(),
            Error::Read(err) => err.describe(),
            Error::Write(err) => err.describe(),
            Error::TODO => "TODO",
        }
    }

    fn advert(&self) -> Option<isize> {
        match self {
            Error::Open(err) => err.advert(),
            Error::Read(err) => err.advert(),
            Error::Write(err) => err.advert(),
            Error::TODO => None,
        }
    }
}

impl Into<isize> for Error {
    fn into(self) -> isize {
        match self {
            _ => unsafe { *(&self as *const Self as *const isize) },
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
