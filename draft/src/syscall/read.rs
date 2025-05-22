use super::Number;
use crate::arch::handle_return;
use crate::arch::{Arch, Callable};

static NUMBER: usize = Number::Write as usize;

pub fn wrapped_read(
    file_descriptor: isize,
    byte_buffer: *const u8,
    byte_length: usize,
) -> crate::Result<isize> {
    Arch::wrapped_syscall3(
        NUMBER,
        file_descriptor as usize,
        byte_buffer as usize,
        byte_length as usize,
    )
}

pub fn read(
    file_descriptor: isize,
    byte_buffer: *const u8,
    byte_length: usize,
) -> crate::Result<isize> {
    handle_return(Arch::syscall3(
        NUMBER,
        file_descriptor as usize,
        byte_buffer as usize,
        byte_length as usize,
    ))
}
