use super::Number;
use crate::ErrorTrait;
use crate::arch::Error as ArchError;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    EPERM = 1, // Operation not permitted
    TODO,
}

impl Error {
    pub fn from_arch(n: Number, e: ArchError) -> Self {
        match (n, e) {
            (Number::Exit, ArchError::TODO) => Self::TODO,
            _ => Self::TODO,
        }
    }
}

impl ErrorTrait for Error {
    fn from_no(errno: isize) -> crate::Error {
        crate::Error::Syscall(match -errno {
            1 => Self::EPERM,
            _ => Self::TODO,
        })
    }

    fn describe(&self) -> &str {
        match self {
            Self::EPERM => "Operation not permitted",
            _ => "Unknown error",
        }
    }

    fn advert(&self) -> Option<isize> {
        None
    }
}

impl Into<isize> for Error {
    fn into(self) -> isize {
        self as isize
    }
}

impl From<isize> for Error {
    fn from(n: isize) -> Self {
        match n {
            1 => Self::EPERM,
            _ => Self::TODO,
        }
    }
}

pub fn handle_return(ret: isize) -> crate::Result<isize> {
    if ret < 0 {
        Err(Error::from_no(-ret))
    } else {
        Ok(ret)
    }
}
