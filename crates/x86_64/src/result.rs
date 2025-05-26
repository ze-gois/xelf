use result::ErrorTrait;

#[repr(isize)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    TODO,
}

impl ErrorTrait for Error {
    fn from_no(errno: isize) -> Self {
        match -errno {
            _ => Self::TODO,
        }
    }

    fn describe(&self) -> &str {
        match self {
            _ => "TODO",
        }
    }

    fn advert(&self) -> Option<isize> {
        None
    }
}

impl Into<isize> for Error {
    fn into(self) -> isize {
        match self {
            _ => unsafe { *(&self as *const Self as *const isize) },
        }
    }
}

pub fn handle_result(result: usize) -> Result<isize> {
    let signed_result = result as isize;

    if signed_result < 0 {
        Err(Error::from_no(-signed_result))
    } else {
        Ok(signed_result)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
