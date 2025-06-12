use crate::arch64::data::UnsignedChar as T;

#[derive(Clone, Copy)]
pub enum Endianess {
    NONE = 0, /* Invalid data encoding */
    LSB = 1,  /* 2's complement, little endian */
    MSB = 2,  /* 2's complement, big endian */
    NUM = 3,
    UNDEFINED = 4,
}

type E = Endianess;

impl E {
    pub fn from(b: T) -> Self {
        match b {
            0 => Self::NONE,
            1 => Self::LSB,
            2 => Self::MSB,
            3 => Self::NUM,
            _ => Self::UNDEFINED,
        }
    }

    pub fn to(&self) -> T {
        match self {
            Self::NONE => 0,
            Self::LSB => 1,
            Self::MSB => 2,
            Self::NUM => 3,
            Self::UNDEFINED => 4,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NONE => "Invalid data encoding data.",
            Self::LSB => "LSB: 2's complement, little endian data.",
            Self::MSB => "MSB: 2's complement, big endian data.",
            Self::NUM => "NUM data.",
            Self::UNDEFINED => "Undefined data encoding",
        }
    }
}

use core::fmt::{Debug, Display, Formatter, Result};

impl Display for E {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.as_str())
    }
}

impl Debug for E {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.as_str())
    }
}
