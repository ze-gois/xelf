use x86_64::syscall::error::Error as SyscallError;

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum Error {
    TODO,
    NotExecutable,
    ShorterData,
    InvalidData,
    SystemError(SyscallError), // Add this variant
}

impl Error {
    pub fn from_number(errno: i32) -> Option<Error> {
        match -errno {
            1 => Some(Error::ShorterData),
            2 => Some(Error::InvalidData),
            _ => {
                // panic!("Unknown error code: {}", errno)
                None
            }
        }
    }
    pub fn to_str(&self) -> &'static str {
        match self {
            Error::TODO => "TODO",
            Error::NotExecutable => "Not executable",
            Error::ShorterData => "Shorter data",
            Error::InvalidData => "Invalid data",
            Error::SystemError(_) => "System error",
        }
    }
}

impl From<SyscallError> for Error {
    fn from(error: SyscallError) -> Self {
        Error::SystemError(error)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
