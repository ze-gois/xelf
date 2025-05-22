use super::Number;
use super::handle_return;
use crate::arch::{Arch, Callable};

static NUMBER: usize = Number::Write as usize;

pub fn wrapped_write(
    file_descriptor: isize,
    byte_buffer: *const u8,
    byte_count: usize,
) -> crate::Result<isize> {
    Arch::wrapped_syscall3(
        NUMBER,
        file_descriptor as usize,
        byte_buffer as usize,
        byte_count as usize,
    )
}

pub fn write(
    file_descriptor: isize,
    byte_buffer: *const u8,
    byte_count: usize,
) -> crate::Result<isize> {
    let returned_value = Arch::syscall3(
        NUMBER,
        file_descriptor as usize,
        byte_buffer as usize,
        byte_count as usize,
    );

    handle_return(returned_value as isize)
}
