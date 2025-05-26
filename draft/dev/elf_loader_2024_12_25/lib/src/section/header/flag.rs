pub use crate::arch::XWord as T;

pub const MASK_PROC: T = 0xF0000000; /* Processor-specific mask */

#[repr(C)]
pub enum Flag {
    /// No rights
    Blank = 0x0,
    /// Write
    Write = 0x1,
    /// Allocate
    Allocate = 0x2,
    /// Execute
    Execute = 0x4,
    /// Write and Allocate
    WriteAlloccation = 0x3,
    /// Write and Execute
    WriteExecute = 0x5,
    /// Allocate and Execute
    AllocateExecute = 0x6,
    /// Write, Allocate and Execute
    WriteAllocateExecute = 0x7,
    /// Processor-specific no rights
    ProcBlank = 0xF0000000,
    /// Processor-specific Write
    ProcWrite = 0xF0000001,
    /// Processor-specific Allocate
    ProcAllocate = 0xF0000002,
    /// Processor-specific Execute
    ProcExecute = 0xF0000004,
    /// Processor-specific Write and Allocate
    ProcWriteAlloccation = 0xF0000003,
    /// Processor-specific Write and Execute
    ProcWriteExecute = 0xF0000005,
    /// Processor-specific Allocate and Execute
    ProcAllocateExecute = 0xF0000006,
    /// Processor-specific Write, Allocate and Execute
    ProcWriteAllocateExecute = 0xF0000007,
    /// Processor-specific Undefined
    Undefined = 0x8,
}

impl Flag {
    pub fn from(i: T) -> Self {
        match i {
            0x0 => Self::Blank,
            0x1 => Self::Write,
            0x2 => Self::Allocate,
            0x4 => Self::Execute,
            0x3 => Self::WriteAlloccation,
            0x5 => Self::WriteExecute,
            0x6 => Self::AllocateExecute,
            0x7 => Self::WriteAllocateExecute,
            0xF0000000 => Self::ProcBlank,
            0xF0000001 => Self::ProcWrite,
            0xF0000002 => Self::ProcAllocate,
            0xF0000004 => Self::ProcExecute,
            0xF0000003 => Self::ProcWriteAlloccation,
            0xF0000005 => Self::ProcWriteExecute,
            0xF0000006 => Self::ProcAllocateExecute,
            0xF0000007 => Self::ProcWriteAllocateExecute,
            _ => Self::Undefined,
        }
    }

    pub fn to(&self) -> T {
        match &self {
            Self::Blank => 0x0,
            Self::Write => 0x1,
            Self::Allocate => 0x2,
            Self::Execute => 0x4,
            Self::WriteAlloccation => 0x3,
            Self::WriteExecute => 0x5,
            Self::AllocateExecute => 0x6,
            Self::WriteAllocateExecute => 0x7,
            Self::ProcBlank => 0xF0000000,
            Self::ProcWrite => 0xF0000001,
            Self::ProcAllocate => 0xF0000002,
            Self::ProcExecute => 0xF0000004,
            Self::ProcWriteAlloccation => 0xF0000003,
            Self::ProcWriteExecute => 0xF0000005,
            Self::ProcAllocateExecute => 0xF0000006,
            Self::ProcWriteAllocateExecute => 0xF0000007,
            Self::Undefined => 0x8,
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

    pub fn as_str_acronym(&self) -> &'static str {
        match &self {
            Self::Blank => "          ",
            Self::Write => " W        ",
            Self::Allocate => "  A       ",
            Self::Execute => "   E      ",
            Self::WriteAlloccation => " WA       ",
            Self::WriteExecute => " W E      ",
            Self::AllocateExecute => "  AE      ",
            Self::WriteAllocateExecute => " WAE      ",
            Self::ProcBlank => "N   (Proc)",
            Self::ProcWrite => " W  (Proc)",
            Self::ProcAllocate => "  A (Proc)",
            Self::ProcExecute => "   E(Proc)",
            Self::ProcWriteAlloccation => " WA (Proc)",
            Self::ProcWriteExecute => " W E(Proc)",
            Self::ProcAllocateExecute => "  AE(Proc)",
            Self::ProcWriteAllocateExecute => " WAE(Proc)",
            Self::Undefined => "~Undefined",
        }
    }
}

use core::ops::Add;

impl Add for Flag {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Flag::from(self.to() | rhs.to())
    }
}

use core::ops::Sub;

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
