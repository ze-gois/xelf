#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
use x86_64 as arch;

use crate::result::*;

pub struct Arch;

pub fn write(file_descriptor: isize, byte_buffer: *const u8, byte_count: usize) -> Result<isize> {
    let returned_value = arch::syscall3(
        1usize,
        file_descriptor as usize,
        byte_buffer as usize,
        byte_count as usize,
    );

    handle_result(returned_value)
}
