use crate::arch64::data::UnsignedChar as T;

#[derive(Clone, Copy)]
pub enum Class {
    NONE = 0, /* Invalid class */
    C32 = 1,  /* 32-bit objects */
    C64 = 2,  /* 64-bit objects */
    NUM = 3,
    UNDEFINED = 255,
}

type E = Class;

impl E {
    pub fn from(i: T) -> Self {
        match i {
            0 => Self::NONE,
            1 => Self::C32,
            2 => Self::C64,
            3 => Self::NUM,
            _ => Self::UNDEFINED,
        }
    }

    pub fn to(&self) -> T {
        match self {
            Self::NONE => 0,
            Self::C32 => 1,
            Self::C64 => 2,
            Self::NUM => 3,
            Self::UNDEFINED => 255,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NONE => "Invalid class",
            Self::C32 => "32-bit objects class",
            Self::C64 => "64-bit objects class",
            Self::NUM => "NUM class",
            Self::UNDEFINED => "Undefined class",
        }
    }
}

use std::fmt::{Debug, Display, Formatter, Result};

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
