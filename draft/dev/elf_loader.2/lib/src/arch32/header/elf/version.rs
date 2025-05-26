pub use crate::arch32::data::Word as T;

#[derive(Clone, Copy)]
pub enum Version {
    None = 0,    /* Invalid version */
    Current = 1, /* Current version */
    Num = 2,
    Undefined = 3,
}

type E = Version;

impl E {
    pub fn from(i: T) -> Self {
        match i {
            0 => Self::None,
            1 => Self::Current,
            2 => Self::Num,
            _ => Self::Undefined,
        }
    }

    pub fn to(&self) -> T {
        match self {
            Self::None => 0,
            Self::Current => 1,
            Self::Num => 2,
            Self::Undefined => 3,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "Invalid version",
            Self::Current => "Current version",
            Self::Num => "?  Number of version",
            Self::Undefined => "Undefined version",
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
