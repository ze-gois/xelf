#![no_std]
pub mod result;
pub use result::*;

pub mod exit;
pub mod open;
pub mod read;
pub mod write;

pub use exit::exit;
pub use open::{openat, openat4};
pub use read::read;
pub use write::write;

pub enum Number {
    Exit = 60,
    Write = 1,
    OpenAt = 257,
    Read = 0,
}

impl Number {
    pub fn from(n: usize) -> Option<Number> {
        match n {
            60 => Some(Number::Exit),
            1 => Some(Number::Write),
            257 => Some(Number::OpenAt),
            _ => None,
        }
    }

    // pub fn tupled_function(n: usize) -> Option<fn((usize,)) -> usize> {
    //     match n {
    //         1 => Some(tupled_write),
    //         _ => None,
    //     }
    // }
}

impl Into<usize> for Number {
    fn into(self) -> usize {
        self as usize
    }
}
