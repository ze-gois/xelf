pub trait ToFlags {
    fn to(self) -> i32;
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum Open {
    // Access modes
    RDONLY = 0o0,
    WRONLY = 0o1,
    RDWR = 0o2,

    // Creation and file status flags
    CREAT = 0o100,
    EXCL = 0o200,
    NOCTTY = 0o400,
    TRUNC = 0o1000,
    APPEND = 0o2000,
    NONBLOCK = 0o4000,
    DSYNC = 0o10000,
    SYNC = 0o4010000,
    DIRECTORY = 0o200000,
    NOFOLLOW = 0o400000,
    CLOEXEC = 0o2000000,
}

impl ToFlags for Open {
    fn to(self) -> i32 {
        self as i32
    }
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum OpenAt {
    // Special fd values for openat
    FDCWD = -100,            // Use current working directory
    REMOVEDIR = 0x200,       // Remove directory instead of file
    SymlinkFollow = 0x400,   // Follow symbolic links
    SymlinkNoFollow = 0x100, // Don't follow symbolic links
}

impl ToFlags for OpenAt {
    fn to(self) -> i32 {
        self as i32
    }
}

// Open | OpenAt
impl core::ops::BitOr<OpenAt> for Open {
    type Output = i32;
    fn bitor(self, rhs: OpenAt) -> i32 {
        (self as i32) | (rhs as i32)
    }
}

// OpenAt | Open
impl core::ops::BitOr<Open> for OpenAt {
    type Output = i32;
    fn bitor(self, rhs: Open) -> i32 {
        (self as i32) | (rhs as i32)
    }
}

// Open | Open
impl core::ops::BitOr for Open {
    type Output = i32;
    fn bitor(self, rhs: Self) -> i32 {
        (self as i32) | (rhs as i32)
    }
}

// OpenAt | OpenAt
impl core::ops::BitOr for OpenAt {
    type Output = i32;
    fn bitor(self, rhs: Self) -> i32 {
        (self as i32) | (rhs as i32)
    }
}
