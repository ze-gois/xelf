use super::Number;
use crate::arch::handle_return;
use crate::arch::{Arch, Callable};

pub mod flags;

static NUMBER: usize = Number::OpenAt as usize;

pub fn wrapped_openat(
    directory_file_descriptor: i32,
    file_pathname: *const u8,
    flags: i32,
) -> crate::Result<isize> {
    Arch::wrapped_syscall3(
        NUMBER,
        directory_file_descriptor as usize,
        file_pathname as usize,
        flags as usize,
    )
}

pub fn openat(
    directory_file_descriptor: i32,
    file_pathname: *const u8,
    flags: i32,
) -> crate::Result<isize> {
    let returned_value = Arch::syscall3(
        NUMBER,
        directory_file_descriptor as usize,
        file_pathname as usize,
        flags as usize,
    );

    handle_return(returned_value)
}

pub fn wrapped_openat4(
    directory_file_descriptor: i32,
    file_pathname: *const u8,
    flags: i32,
    mode: i32,
) -> crate::Result<isize> {
    Arch::wrapped_syscall4(
        NUMBER,
        directory_file_descriptor as usize,
        file_pathname as usize,
        flags as usize,
        mode as usize,
    )
}

pub fn openat4(
    directory_file_descriptor: i32,
    file_pathname: *const u8,
    flags: i32,
    mode: i32,
) -> crate::Result<isize> {
    handle_return(Arch::syscall4(
        NUMBER,
        directory_file_descriptor as usize,
        file_pathname as usize,
        flags as usize,
        mode as usize,
    ))
}
