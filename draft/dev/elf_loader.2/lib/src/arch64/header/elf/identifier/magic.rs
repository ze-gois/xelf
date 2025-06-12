use crate::arch64::data::UnsignedChar as T;

#[derive(Clone, Copy)]
pub enum Magic {
    Undefined = 0,
    M0 = 0x7f,
    M1 = 0x45, // ASCII for 'E'
    M2 = 0x4C, // ASCII for 'L'
    M3 = 0x46, // ASCII for 'F'
}

type E = Magic;

impl E {
    pub fn from(i: T) -> Self {
        match i {
            0x7f => Self::M0,
            0x45 => Self::M1,
            0x4C => Self::M2,
            0x46 => Self::M3,
            _ => Self::Undefined,
        }
    }

    pub fn to(&self) -> T {
        match self {
            Self::M0 => 0x7f,
            Self::M1 => 0x45,
            Self::M2 => 0x4c,
            Self::M3 => 0x46,
            Self::Undefined => 0x33,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::M0 => "(DEL)",
            Self::M1 => "E",
            Self::M2 => "L",
            Self::M3 => "F",
            Self::Undefined => "Undefined",
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
