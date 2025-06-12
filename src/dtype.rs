pub mod macros;
use crate::Result;
use crate::elf_define_type;

// #[derive(Copy, Clone, Eq, PartialEq)]
// pub enum Endianness {
//     Little,
//     Big,
// }

use crate::dtype::UChar as T;

#[repr(C)]
#[derive(Clone, Copy)]
pub enum Endianness {
    /// Invalid data encoding
    None = 0,
    /// 2's complement, little endian
    LSB = 1,
    /// 2's complement, big endian
    MSB = 2,
    /// Number of valid endianesses
    Number = 3,
    /// Syntax
    Undefined = 4,
}

type E = Endianness;

impl E {
    pub fn from(b: T) -> Self {
        match b {
            0 => Self::None,
            1 => Self::LSB,
            2 => Self::MSB,
            3 => Self::Number,
            _ => Self::Undefined,
        }
    }

    pub fn to(&self) -> T {
        match self {
            Self::None => 0,
            Self::LSB => 1,
            Self::MSB => 2,
            Self::Number => 3,
            Self::Undefined => 4,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "Invalid data encoding data.",
            Self::LSB => "LSB: 2's complement, little endian data.",
            Self::MSB => "MSB: 2's complement, big endian data.",
            Self::Number => "Number of endianesses",
            Self::Undefined => "Undefined data encoding",
        }
    }
}

use core::fmt::{Debug, Display, Formatter};

impl Display for E {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Debug for E {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub trait ELFType {
    type Inner;
    const SIZE_BITS: usize;
    const SIZE_BYTES: usize;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    InvalidData(isize),
    InvalidEndian,
    InvalidType,
    ShorterData(isize),
}

elf_define_type!(pub SXWord, i64); //Unsigned program address
elf_define_type!(pub UChar, u8); //Unsigned file offset
elf_define_type!(pub Half, u16); //Unsigned medium integer
elf_define_type!(pub SWord, i32); //Unsigned integer
elf_define_type!(pub XWord, u64); //Signed integer
elf_define_type!(pub Word, u32); //Unsigned long integer
elf_define_type!(pub Off, u64); //Signed long integer
elf_define_type!(pub Addr, u64); //Unsigned small integer
