use super::Number;
use arch::{Arch, Callable};

static NUMBER: usize = Number::Read as usize;

define_syscall_error!(
    Error,
    Read,
    "read",
    [
        [BadFileDescriptor, -9, "Bad file descriptor", EBADF],
        [InvalidBuffer, -14, "Invalid buffer pointer", EFAULT],
        [InvalidCount, -22, "Invalid count", EINVAL],
        [Interrupted, -4, "System call was interrupted", EINTR],
        [IOError, -5, "Input/output error", EIO],
        [IsDirectory, -21, "Is a directory", EISDIR],
        [NotReadable, -13, "File not open for reading", EACCES]
    ]
);

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

// type Result<T> = core::result::Result<T, Error>;
