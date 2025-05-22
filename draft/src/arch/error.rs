use crate::ErrorTrait;

#[repr(isize)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    Numbered(isize),
    TODO = 1,
}

impl ErrorTrait for Error {
    fn from_no(errno: isize) -> crate::Error {
        crate::Error::Arch(match -errno {
            _ => Self::Numbered(errno),
        })
    }

    fn describe(&self) -> &str {
        match self {
            _ => "Unknown error",
        }
    }

    fn advert(&self) -> Option<isize> {
        None
    }
}

impl Into<isize> for Error {
    fn into(self) -> isize {
        match self {
            Self::Numbered(n) => n,
            _ => unsafe { *(&self as *const Self as *const isize) },
        }
    }
}

pub fn handle_return(result: usize) -> crate::Result<isize> {
    let signed_result = result as isize;

    if signed_result < 0 {
        Err(Error::from_no(-signed_result))
    } else {
        Ok(signed_result)
    }
}
