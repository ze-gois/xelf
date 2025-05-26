use crate::arch::UnsignedChar as T;

#[repr(C)]
#[derive(Clone, Copy)]
pub enum Index {
    /// 1st magic byte
    Mag0 = 0,
    /// 2nd magic byte
    Mag1 = 1,
    /// 3rd magic byte
    Mag2 = 2,
    /// 4th magic byte
    Mag3 = 3,
    /// File class
    Class = 4,
    /// Data encoding
    Data = 5,
    /// File version1
    Version = 6,
    /// OS/ABI identification
    OsAbi = 7,
    /// ABI version
    AbiVersion = 8,
    /// Start of padding bytes
    Pad = 9,
    /// Unassigned
    Unassigned1 = 10,
    /// Unassigned
    Unassigned2 = 11,
    /// Unassigned
    Unassigned3 = 12,
    /// Unassigned
    Unassigned4 = 13,
    /// Unassigned
    Unassigned5 = 14,
    /// Size of ELF identifier array
    NIdent = 15,
    /// Used for syntax dependence
    Undefined = 16,
}

type E = Index;

impl E {
    pub fn from(i: T) -> Self {
        match i {
            0 => Self::Mag0,
            1 => Self::Mag1,
            2 => Self::Mag2,
            3 => Self::Mag3,
            4 => Self::Class,
            5 => Self::Data,
            6 => Self::Version,
            7 => Self::OsAbi,
            8 => Self::AbiVersion,
            9 => Self::Pad,
            10 => Self::Unassigned1,
            11 => Self::Unassigned2,
            12 => Self::Unassigned3,
            13 => Self::Unassigned4,
            14 => Self::Unassigned5,
            15 => Self::NIdent,
            _ => Self::Undefined,
        }
    }

    pub fn to(&self) -> T {
        match self {
            Self::Mag0 => 0,
            Self::Mag1 => 1,
            Self::Mag2 => 2,
            Self::Mag3 => 3,
            Self::Class => 4,
            Self::Data => 5,
            Self::Version => 6,
            Self::OsAbi => 7,
            Self::AbiVersion => 8,
            Self::Pad => 9,
            Self::Unassigned1 => 10,
            Self::Unassigned2 => 11,
            Self::Unassigned3 => 12,
            Self::Unassigned4 => 13,
            Self::Unassigned5 => 14,
            Self::NIdent => 15,
            Self::Undefined => 16,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Mag0 => "File identification 1st magic byte'",
            Self::Mag1 => "File identification 2nd magic byte'",
            Self::Mag2 => "File identification 3rd magic byte'",
            Self::Mag3 => "File identification 4th magic byte'",
            Self::Class => "File class byte'",
            Self::Data => "Data encoding byte'",
            Self::Version => "File version byte'",
            Self::OsAbi => "OS/ABI identification byte'",
            Self::AbiVersion => "ABI version byte'",
            Self::Pad => "Start of padding bytes byte'",
            Self::Unassigned1 => "Unassigned 10th byte'",
            Self::Unassigned2 => "Unassigned 11th byte'",
            Self::Unassigned3 => "Unassigned 12th byte'",
            Self::Unassigned4 => "Unassigned 13th byte'",
            Self::Unassigned5 => "Unassigned 14th byte'",
            Self::NIdent => "Size of ELF identifier array byte'",
            Self::Undefined => "Out of bounds byte",
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
