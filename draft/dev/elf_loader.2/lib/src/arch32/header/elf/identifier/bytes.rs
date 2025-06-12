use crate::arch32::data::UnsignedChar as T;

#[derive(Clone, Copy)]
pub enum IdentifierByte {
    M0 = 0,         /* File identification */
    M1 = 1,         /* File identification */
    M2 = 2,         /* File identification */
    M3 = 3,         /* File identification */
    CLASS = 4,      /* File class */
    DATA = 5,       /* Data encoding */
    VERSION = 6,    /* File version  */
    OSABI = 7,      /* OS/ABI identification */
    ABIVERSION = 8, /* ABI version */
    PAD = 9,        /* Start of padding bytes */
    EMPTY1 = 10,    /* Unassigned */
    EMPTY2 = 11,    /* Unassigned */
    EMPTY3 = 12,    /* Unassigned */
    EMPTY4 = 13,    /* Unassigned */
    EMPTY5 = 14,    /* Unassigned */
    EMPTY6 = 15,    /* Unassigned */
    NIDENT = 16,    /* Size of ELF identifier array */
    UNDEFINED = 17,
}

type E = IdentifierByte;

impl E {
    pub fn from(i: T) -> Self {
        match i {
            0 => Self::M0,
            1 => Self::M1,
            2 => Self::M2,
            3 => Self::M3,
            4 => Self::CLASS,
            5 => Self::DATA,
            6 => Self::VERSION,
            7 => Self::OSABI,
            8 => Self::ABIVERSION,
            9 => Self::PAD,
            10 => Self::EMPTY1,
            11 => Self::EMPTY2,
            12 => Self::EMPTY3,
            13 => Self::EMPTY4,
            14 => Self::EMPTY5,
            15 => Self::EMPTY6,
            16 => Self::NIDENT,
            _ => Self::UNDEFINED,
        }
    }

    pub fn to(&self) -> T {
        match self {
            Self::M0 => 0,
            Self::M1 => 1,
            Self::M2 => 2,
            Self::M3 => 3,
            Self::CLASS => 4,
            Self::DATA => 5,
            Self::VERSION => 6,
            Self::OSABI => 7,
            Self::ABIVERSION => 8,
            Self::PAD => 9,
            Self::EMPTY1 => 10,
            Self::EMPTY2 => 11,
            Self::EMPTY3 => 12,
            Self::EMPTY4 => 13,
            Self::EMPTY5 => 14,
            Self::EMPTY6 => 15,
            Self::NIDENT => 16,
            Self::UNDEFINED => 17,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::M0 => "File identification 1st magic byte'",
            Self::M1 => "File identification 2nd magic byte'",
            Self::M2 => "File identification 3rd magic byte'",
            Self::M3 => "File identification 4th magic byte'",
            Self::CLASS => "File class byte'",
            Self::DATA => "Data encoding byte'",
            Self::VERSION => "File version byte'",
            Self::OSABI => "OS/ABI identification byte'",
            Self::ABIVERSION => "ABI version byte'",
            Self::PAD => "Start of padding bytes byte'",
            Self::EMPTY1 => "Empty 10th byte'",
            Self::EMPTY2 => "Empty 11th byte'",
            Self::EMPTY3 => "Empty 12th byte'",
            Self::EMPTY4 => "Empty 13th byte'",
            Self::EMPTY5 => "Empty 14th byte'",
            Self::EMPTY6 => "Empty 15th byte'",
            Self::NIDENT => "Size of ELF identifier array byte'",
            Self::UNDEFINED => "Out of bounds byte",
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
