use crate::arch64::data::XWord as T;

pub enum Flag {
    Write = 0x1,           /* Section contains writable data */
    Alloc = 0x2,           /* Section is allocated in memory image of program */
    ExecInstr = 0x4,       /* Section contains executable instructions */
    MaskOs = 0x0F000000,   /* Environment-specific use */
    MaskProc = 0xF0000000, /* Processor-specific use */
    Undefined = 0x5,       /* Undefined */
}

impl Flag {
    pub fn from(i: T) -> Self {
        match i {
            0x1 => Self::Write,
            0x2 => Self::Alloc,
            0x4 => Self::ExecInstr,
            0x0F000000 => Self::MaskOs,
            0xF0000000 => Self::MaskProc,
            _ => Self::Undefined,
        }
    }

    pub fn to(&self) -> T {
        match &self {
            Self::Write => 0x1,           /* Section contains writable data */
            Self::Alloc => 0x2,           /* Section is allocated in memory image of program */
            Self::ExecInstr => 0x4,       /* Section contains executable instructions */
            Self::MaskOs => 0x0F000000,   /* Environment-specific use */
            Self::MaskProc => 0xF0000000, /* Processor-specific use */
            Self::Undefined => 0x5,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match &self {
            Self::Write => "Section contains writable data flag",
            Self::Alloc => "Section is allocated in memory image of program flag",
            Self::ExecInstr => "Section contains executable instructions flag",
            Self::MaskOs => "Environment-specific use flag",
            Self::MaskProc => "Processor-specific use flag",
            Self::Undefined => "Undefined flag",
        }
    }
}

use std::fmt::{Debug, Display, Formatter, Result};

type E = Flag;

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
