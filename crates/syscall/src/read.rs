use super::Number;
use arch::{Arch, Callable};

static NUMBER: usize = Number::Write as usize;

pub fn read(
    file_descriptor: isize,
    byte_buffer: *const u8,
    byte_length: usize,
) -> crate::result::Result<isize> {
    let arch_result = Arch::syscall3(
        NUMBER,
        file_descriptor as usize,
        byte_buffer as usize,
        byte_length as usize,
    );

    handle_result(arch_result)
}

use result::ErrorTrait;

#[repr(isize)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
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
            _ => "TODO",
        }
    }

    fn advert(&self) -> Option<isize> {
        None
    }
}

impl Into<isize> for Error {
    fn into(self) -> isize {
        match self {
            _ => unsafe { *(&self as *const Self as *const isize) },
        }
    }
}

fn handle_result(result: arch::result::Result<isize>) -> crate::result::Result<isize> {
    match result {
        Ok(signed_result) => Ok(signed_result),
        Err(err) => Err(crate::result::Error::Read(match err {
            _ => Error::TODO,
        })),
    }
}

// type Result<T> = core::result::Result<T, Error>;
