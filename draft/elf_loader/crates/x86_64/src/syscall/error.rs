#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum Error {
    EPERM = 1,         // Operation not permitted
    ENOENT = 2,        // No such file or directory
    ESRCH = 3,         // No such process
    EINTR = 4,         // Interrupted system call
    EIO = 5,           // I/O error
    ENXIO = 6,         // No such device or address
    E2BIG = 7,         // Argument list too long
    ENOEXEC = 8,       // Exec format error
    EBADF = 9,         // Bad file descriptor
    ECHILD = 10,       // No child processes
    EAGAIN = 11,       // Try again (EWOULDBLOCK)
    ENOMEM = 12,       // Out of memory
    EACCES = 13,       // Permission denied
    EFAULT = 14,       // Bad address
    ENOTBLK = 15,      // Block device required
    EBUSY = 16,        // Device or resource busy
    EEXIST = 17,       // File exists
    EXDEV = 18,        // Cross-device link
    ENODEV = 19,       // No such device
    ENOTDIR = 20,      // Not a directory
    EISDIR = 21,       // Is a directory
    EINVAL = 22,       // Invalid argument
    ENFILE = 23,       // File table overflow
    EMFILE = 24,       // Too many open files
    ENOTTY = 25,       // Not a typewriter
    ETXTBSY = 26,      // Text file busy
    EFBIG = 27,        // File too large
    ENOSPC = 28,       // No space left on device
    ESPIPE = 29,       // Illegal seek
    EROFS = 30,        // Read-only file system
    EMLINK = 31,       // Too many links
    EPIPE = 32,        // Broken pipe
    EDOM = 33,         // Math argument out of domain
    ERANGE = 34,       // Math result not representable
    EDEADLK = 35,      // Resource deadlock would occur
    ENAMETOOLONG = 36, // File name too long
    ENOLCK = 37,       // No record locks available
    ENOSYS = 38,       // Invalid system call number
    ENOTEMPTY = 39,    // Directory not empty
    ELOOP = 40,        // Too many symbolic links encountered
    ENOMSG = 42,       // No message of desired type
    EIDRM = 43,        // Identifier removed
    ECHRNG = 44,       // Channel number out of range
    EL2NSYNC = 45,     // Level 2 not synchronized
}

impl Error {
    pub fn from_syscall(ret: i64) -> Option<Error> {
        match -ret as i32 {
            1 => Some(Error::EPERM),
            2 => Some(Error::ENOENT),
            3 => Some(Error::ESRCH),
            4 => Some(Error::EINTR),
            5 => Some(Error::EIO),
            6 => Some(Error::ENXIO),
            7 => Some(Error::E2BIG),
            8 => Some(Error::ENOEXEC),
            9 => Some(Error::EBADF),
            10 => Some(Error::ECHILD),
            11 => Some(Error::EAGAIN),
            12 => Some(Error::ENOMEM),
            13 => Some(Error::EACCES),
            14 => Some(Error::EFAULT),
            15 => Some(Error::ENOTBLK),
            16 => Some(Error::EBUSY),
            17 => Some(Error::EEXIST),
            18 => Some(Error::EXDEV),
            19 => Some(Error::ENODEV),
            20 => Some(Error::ENOTDIR),
            21 => Some(Error::EISDIR),
            22 => Some(Error::EINVAL),
            23 => Some(Error::ENFILE),
            24 => Some(Error::EMFILE),
            25 => Some(Error::ENOTTY),
            26 => Some(Error::ETXTBSY),
            27 => Some(Error::EFBIG),
            28 => Some(Error::ENOSPC),
            29 => Some(Error::ESPIPE),
            30 => Some(Error::EROFS),
            31 => Some(Error::EMLINK),
            32 => Some(Error::EPIPE),
            33 => Some(Error::EDOM),
            34 => Some(Error::ERANGE),
            35 => Some(Error::EDEADLK),
            36 => Some(Error::ENAMETOOLONG),
            37 => Some(Error::ENOLCK),
            38 => Some(Error::ENOSYS),
            39 => Some(Error::ENOTEMPTY),
            40 => Some(Error::ELOOP),
            42 => Some(Error::ENOMSG),
            43 => Some(Error::EIDRM),
            44 => Some(Error::ECHRNG),
            45 => Some(Error::EL2NSYNC),
            _ => None,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Error::EPERM => "Operation not permitted",
            Error::ENOENT => "No such file or directory",
            Error::ESRCH => "No such process",
            Error::EINTR => "Interrupted system call",
            Error::EIO => "Input/output error",
            Error::ENXIO => "No such device or address",
            Error::E2BIG => "Argument list too long",
            Error::ENOEXEC => "Exec format error",
            Error::EBADF => "Bad file descriptor",
            Error::ECHILD => "No child processes",
            Error::EAGAIN => "Resource temporarily unavailable",
            Error::ENOMEM => "Cannot allocate memory",
            Error::EACCES => "Permission denied",
            Error::EFAULT => "Bad address",
            Error::ENOTBLK => "Block device required",
            Error::EBUSY => "Device or resource busy",
            Error::EEXIST => "File exists",
            Error::EXDEV => "Invalid cross-device link",
            Error::ENODEV => "No such device",
            Error::ENOTDIR => "Not a directory",
            Error::EISDIR => "Is a directory",
            Error::EINVAL => "Invalid argument",
            Error::ENFILE => "Too many open files in system",
            Error::EMFILE => "Too many open files",
            Error::ENOTTY => "Inappropriate ioctl for device",
            Error::ETXTBSY => "Text file busy",
            Error::EFBIG => "File too large",
            Error::ENOSPC => "No space left on device",
            Error::ESPIPE => "Illegal seek",
            Error::EROFS => "Read-only file system",
            Error::EMLINK => "Too many links",
            Error::EPIPE => "Broken pipe",
            Error::EDOM => "Numerical argument out of domain",
            Error::ERANGE => "Numerical result out of range",
            Error::EDEADLK => "Resource deadlock avoided",
            Error::ENAMETOOLONG => "File name too long",
            Error::ENOLCK => "No locks available",
            Error::ENOSYS => "Function not implemented",
            Error::ENOTEMPTY => "Directory not empty",
            Error::ELOOP => "Too many levels of symbolic links",
            Error::ENOMSG => "No message of desired type",
            Error::EIDRM => "Identifier removed",
            Error::ECHRNG => "Channel number out of range",
            Error::EL2NSYNC => "Level 2 not synchronized",
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
