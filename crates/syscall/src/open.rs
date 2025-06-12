use super::Number;
use arch::{Arch, Callable};

pub mod flags;
pub use flags::{AtFlag, Flag};

static NUMBER: usize = Number::OpenAt as usize;

define_syscall_error!(
    Error,
    Open,
    "openat",
    [
        [FileNotFound, -2, "File not found", ENOENT],
        [PermissionDenied, -13, "Permission denied", EACCES],
        [InvalidPath, -22, "Invalid path", EINVAL],
        [DirectoryNotFound, -20, "Directory not found", ENOTDIR],
        [
            TooManySymlinks,
            -40,
            "Too many levels of symbolic links",
            ELOOP
        ],
        [PathnameTooLong, -36, "Pathname too long", ENAMETOOLONG],
        [FileExists, -17, "File exists", EEXIST],
        [TooManyOpenFiles, -24, "Too many open files", EMFILE],
        [NoSpace, -28, "No space left on device", ENOSPC]
    ]
);

pub fn openat(
    directory_file_descriptor: isize,
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
    directory_file_descriptor: isize,
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

// pub type Result<T> = core::result::Result<T, Error>;
