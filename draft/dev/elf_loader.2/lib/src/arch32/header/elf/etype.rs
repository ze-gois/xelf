pub use crate::arch32::data::Half as T;

#[derive(Clone, Copy)]
pub enum Type {
    None = 0,        /* No file type  */
    Rel = 1,         /* Relocatable file  */
    Exec = 2,        /* Executable file  */
    Dyn = 3,         /* Shared object file  */
    Core = 4,        /* Core file  */
    Num = 5,         /*Number of defined types */
    LoProc = 0xff00, /* Processor-specific  */
    HiProc = 0xffff, /* Processor-specific  */
    Undefined = 6,   /* Undefined use */
}

type E = Type;

impl E {
    pub fn from(i: T) -> Self {
        match i {
            0 => Self::None,
            1 => Self::Rel,
            2 => Self::Exec,
            3 => Self::Dyn,
            4 => Self::Core,
            5 => Self::Num,
            0xFF00 => Self::LoProc,
            0xFFFF => Self::HiProc,
            _ => Self::Undefined,
        }
    }

    pub fn to(&self) -> T {
        match self {
            Self::None => 0,
            Self::Rel => 1,
            Self::Exec => 2,
            Self::Dyn => 3,
            Self::Core => 4,
            Self::Num => 5,
            Self::LoProc => 0xFF00,
            Self::HiProc => 0xFFFF,
            Self::Undefined => 6,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "NONE (No file type)",
            Self::Rel => "REL (Relocatable object file type)",
            Self::Exec => "EXEC (Executable file type)",
            Self::Dyn => "DYN (Shared object file type)",
            Self::Core => "CORE (Core file type)",
            Self::Num => "NUM (Number of defined types)",
            Self::LoProc => "LOPROC (Processor-specific type)",
            Self::HiProc => "HIPROC (Processor-specific type)",
            Self::Undefined => "UNDEFINED (Undefined type)",
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
