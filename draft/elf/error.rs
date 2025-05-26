#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[repr(u32)]
pub enum Error {
    NotELF = 0,
}

pub type Result<T = (), E = Error> = core::result::Result<T, E>;
