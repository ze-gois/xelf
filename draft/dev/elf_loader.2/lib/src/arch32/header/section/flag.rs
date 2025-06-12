pub use crate::arch32::data::Word as T;

pub const MASK_PROC: T = 0xF0000000; /* Processor-specific mask */

pub enum Flag {
    Blank = 0x0,                           /* No rights */
    Write = 0x1,                           /* Write */
    Allocate = 0x2,                        /* Allocate */
    Execute = 0x4,                         /* Execute */
    WriteAlloccation = 0x3,                /* Write and Allocate */
    WriteExecute = 0x5,                    /* Write and Execute */
    AllocateExecute = 0x6,                 /* Allocate and Execute */
    WriteAllocateExecute = 0x7,            /* Write, Allocate and Execute */
    ProcBlank = 0xF0000000,                /* Processor-specific no rights */
    ProcWrite = 0xF0000001,                /* Processor-specific Write */
    ProcAllocate = 0xF0000002,             /* Processor-specific Allocate */
    ProcExecute = 0xF0000004,              /* Processor-specific Execute */
    ProcWriteAlloccation = 0xF0000003,     /* Processor-specific Write and Allocate */
    ProcWriteExecute = 0xF0000005,         /* Processor-specific Write and Execute */
    ProcAllocateExecute = 0xF0000006,      /* Processor-specific Allocate and Execute */
    ProcWriteAllocateExecute = 0xF0000007, /* Processor-specific Write, Allocate and Execute */
    Undefined = 0x8,                       /* Processor-specific Undefined */
}

impl Flag {
    pub fn from(i: T) -> Self {
        match i {
            0x0 => Self::Blank,                           /* No rights */
            0x1 => Self::Write,                           /* Write */
            0x2 => Self::Allocate,                        /* Allocate */
            0x4 => Self::Execute,                         /* Execute */
            0x3 => Self::WriteAlloccation,                /* Write and Allocate */
            0x5 => Self::WriteExecute,                    /* Write and Execute */
            0x6 => Self::AllocateExecute,                 /* Allocate and Execute */
            0x7 => Self::WriteAllocateExecute,            /* Write, Allocate and Execute */
            0xF0000000 => Self::ProcBlank,                /* Processor-specific no rights */
            0xF0000001 => Self::ProcWrite,                /* Processor-specific Write */
            0xF0000002 => Self::ProcAllocate,             /* Processor-specific Allocate */
            0xF0000004 => Self::ProcExecute,              /* Processor-specific Execute */
            0xF0000003 => Self::ProcWriteAlloccation, /* Processor-specific Write and Allocate */
            0xF0000005 => Self::ProcWriteExecute,     /* Processor-specific Write and Execute */
            0xF0000006 => Self::ProcAllocateExecute,  /* Processor-specific Allocate and Execute */
            0xF0000007 => Self::ProcWriteAllocateExecute, /* Processor-specific Write, Allocate and Execute */
            _ => Self::Undefined,                         /* Processor-specific Undefined */
        }
    }

    pub fn to(&self) -> T {
        match &self {
            Self::Blank => 0x0,                           /* No rights */
            Self::Write => 0x1,                           /* Write */
            Self::Allocate => 0x2,                        /* Allocate */
            Self::Execute => 0x4,                         /* Execute */
            Self::WriteAlloccation => 0x3,                /* Write and Allocate */
            Self::WriteExecute => 0x5,                    /* Write and Execute */
            Self::AllocateExecute => 0x6,                 /* Allocate and Execute */
            Self::WriteAllocateExecute => 0x7,            /* Write, Allocate and Execute */
            Self::ProcBlank => 0xF0000000,                /* Processor-specific no rights */
            Self::ProcWrite => 0xF0000001,                /* Processor-specific Write */
            Self::ProcAllocate => 0xF0000002,             /* Processor-specific Allocate */
            Self::ProcExecute => 0xF0000004,              /* Processor-specific Execute */
            Self::ProcWriteAlloccation => 0xF0000003, /* Processor-specific Write and Allocate */
            Self::ProcWriteExecute => 0xF0000005,     /* Processor-specific Write and Execute */
            Self::ProcAllocateExecute => 0xF0000006,  /* Processor-specific Allocate and Execute */
            Self::ProcWriteAllocateExecute => 0xF0000007, /* Processor-specific Write, Allocate and Execute */
            Self::Undefined => 0x8,                       /* Processor-specific Undefined */
        }
    }

    pub fn as_str(&self) -> &'static str {
        match &self {
            Self::Blank => "No rights",
            Self::Write => "Write",
            Self::Allocate => "Allocate",
            Self::Execute => "Execute",
            Self::WriteAlloccation => "Write and Allocate",
            Self::WriteExecute => "Write and Execute",
            Self::AllocateExecute => "Allocate and Execute",
            Self::WriteAllocateExecute => "Write, Allocate and Execute",
            Self::ProcBlank => "Processor-specific no rights",
            Self::ProcWrite => "Processor-specific Write",
            Self::ProcAllocate => "Processor-specific Allocate",
            Self::ProcExecute => "Processor-specific Execute",
            Self::ProcWriteAlloccation => "Processor-specific Write and Allocate",
            Self::ProcWriteExecute => "Processor-specific Write and Execute",
            Self::ProcAllocateExecute => "Processor-specific Allocate and Execute",
            Self::ProcWriteAllocateExecute => "Processor-specific Write, Allocate and Execute",
            Self::Undefined => "Processor-specific Undefined",
        }
    }
}

use std::ops::Add;

impl Add for Flag {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Flag::from(self.to() | rhs.to())
    }
}

use std::ops::Sub;

impl Sub for Flag {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Flag::from(self.to() & !rhs.to())
    }
}

use core::fmt::{Debug, Display, Formatter, Result};

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
