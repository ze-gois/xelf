use super::Number;
use arch::{Arch, Callable};

static NUMBER: usize = Number::Write as usize;

define_syscall_error!(Error, Write, "write", [
    [BadFileDescriptor, -9, "Bad file descriptor", EBADF],
    [InvalidBuffer, -14, "Invalid buffer pointer", EFAULT],
    [BufferTooLarge, -27, "Buffer too large", EFBIG],
    [Interrupted, -4, "System call was interrupted", EINTR],
    [IOError, -5, "Input/output error", EIO],
    [NoSpaceLeft, -28, "No space left on device", ENOSPC],
    [BrokenPipe, -32, "Broken pipe", EPIPE]
]);

pub fn write(
    file_descriptor: isize,
    byte_buffer: *const u8,
    byte_count: usize,
) -> crate::result::Result<isize> {
    let syscall_result = Arch::syscall3(
        NUMBER,
        file_descriptor as usize,
        byte_buffer as usize,
        byte_count as usize,
    );

    handle_result(syscall_result)
}

// type Result<T> = core::result::Result<T, Error>;
