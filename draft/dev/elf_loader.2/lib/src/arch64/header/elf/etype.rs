use crate::arch64::data::Half as T;

#[derive(Clone, Copy)]
pub enum ElfType {
    None = 0,        /* No file type */
    Rel = 1,         /* Relocatable object file */
    Exec = 2,        /* Executable file */
    Dyn = 3,         /* Shared object file */
    Core = 4,        /* Core file */
    LoOS = 0xFE00,   /* Environment-specific use lower boundary */
    HiOS = 0xFEFF,   /* Environment-specific use upper boundary */
    LoProc = 0xFF00, /* Processor-specific use lower boundary */
    HiProc = 0xFFFF, /* Processor-specific use upper boundary */
    Num = 5,         /* Number of defined types */
    Undefined = 6,   /* Undefined use */
}

type E = ElfType;

impl E {
    pub fn from(i: T) -> Self {
        match i {
            0 => Self::None,        /* No file type */
            1 => Self::Rel,         /* Relocatable object file */
            2 => Self::Exec,        /* Executable file */
            3 => Self::Dyn,         /* Shared object file */
            4 => Self::Core,        /* Core file */
            0xFE00 => Self::LoOS,   /* Environment-specific use lower boundary */
            0xFEFF => Self::HiOS,   /* Environment-specific use upper boundary */
            0xFF00 => Self::LoProc, /* Processor-specific use lower boundary */
            0xFFFF => Self::HiProc, /* Processor-specific use upper boundary */
            5 => Self::Num,         /* Number of defined types */
            _ => Self::Undefined,   /* Undefined use */
        }
    }

    pub fn to(&self) -> T {
        match self {
            Self::None => 0,        /* No file type */
            Self::Rel => 1,         /* Relocatable object file */
            Self::Exec => 2,        /* Executable file */
            Self::Dyn => 3,         /* Shared object file */
            Self::Core => 4,        /* Core file */
            Self::LoOS => 0xFE00,   /* Environment-specific use lower boundary */
            Self::HiOS => 0xFEFF,   /* Environment-specific use upper boundary */
            Self::LoProc => 0xFF00, /* Processor-specific use lower boundary */
            Self::HiProc => 0xFFFF, /* Processor-specific use upper boundary */
            Self::Num => 5,         /* Number of defined types */
            Self::Undefined => 6,   /* Undefined use */
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "No file type",
            Self::Rel => "Relocatable object file",
            Self::Exec => "Executable file",
            Self::Dyn => "Shared object file",
            Self::Core => "Core file",
            Self::LoOS => "Environment-specific use lower boundary",
            Self::HiOS => "Environment-specific use upper boundary",
            Self::LoProc => "Processor-specific use lower boundary",
            Self::HiProc => "Processor-specific use upper boundary",
            Self::Num => "Number of defined types",
            Self::Undefined => "Undefined use",
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
