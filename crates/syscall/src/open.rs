use super::Number;
use arch::{Arch, Callable};

pub mod flags;

static NUMBER: usize = Number::OpenAt as usize;

pub fn openat(
    directory_file_descriptor: i32,
    file_pathname: *const u8,
    flags: i32,
) -> crate::result::Result<isize> {
    let syscall_result = Arch::syscall3(
        NUMBER,
        directory_file_descriptor as usize,
        file_pathname as usize,
        flags as usize,
    );

    handle_result(syscall_result)
}

pub fn openat4(
    directory_file_descriptor: i32,
    file_pathname: *const u8,
    flags: i32,
    mode: i32,
) -> crate::result::Result<isize> {
    let syscall_result = Arch::syscall4(
        NUMBER,
        directory_file_descriptor as usize,
        file_pathname as usize,
        flags as usize,
        mode as usize,
    );
    handle_result(syscall_result)
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
        Err(err) => Err(crate::result::Error::Open(match err {
            _ => Error::TODO,
        })),
    }
}

// pub type Result<T> = core::result::Result<T, Error>;
