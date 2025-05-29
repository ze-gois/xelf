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
            Error::TODO => "TODO",
        }
    }

    fn advert(&self) -> Option<isize> {
        match self {
            Error::TODO => None,
        }
    }
}

impl Into<isize> for Error {
    fn into(self) -> isize {
        match self {
            _ => unsafe { *(&self as *const Self as *const isize) },
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;

fn handle_result(result: usize) -> Result<isize> {
    match result {
        Ok(signed_result) => Ok(signed_result),
        Err(err) => {
            let errno: isize = err.into();

            let matched_error = match errno {
                _ => Error::TODO,
            };

            Err(matched_error)
        }
    }
}
