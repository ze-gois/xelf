#[repr(isize)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    TODO,
}

impl Error {
    fn from_no(errno: isize) -> Self {
        match -errno {
            _ => Error::TODO,
        }
    }
}

pub fn handle_result(errno: usize) -> Result<isize> {
    let errno = errno as isize;

    if (errno) < 0 {
        Err(Error::from_no(errno))
    } else {
        Ok(errno)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
