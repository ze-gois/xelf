use crate::arch::UnsignedChar as T;

#[repr(C)]
pub enum Endianess {
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

type E = Endianess;

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
