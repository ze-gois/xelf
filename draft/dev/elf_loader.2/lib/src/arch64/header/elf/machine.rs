pub use crate::arch64::data::Half as T;

#[derive(Clone, Copy)]
pub enum Machine {
    None = 0,        /* No machine */
    M32 = 1,         /* AT&T WE 32100 */
    Sparc = 2,       /* SPARC */
    M386 = 3,        /* Intel Architecture */
    M68k = 4,        /* Motorola 68000 */
    M88k = 5,        /* Motorola 88000 */
    M860 = 7,        /* Intel 80860 */
    Mips = 8,        /* MIPS RS3000 Big-Endian */
    MipsRS4Be = 10,  /* MIPS RS4000 Big-Endian */
    Num = 6,         /* Number of valid machines */
    ReservedLo = 11, /* Reserved for future use */
    ReservedHi = 16, /* Reserved for future use */
    Undefined = 17,  /* Not defined */
}

type E = Machine;

impl E {
    pub fn from(i: T) -> Self {
        match i {
            0 => Self::None,        /* No machine */
            1 => Self::M32,         /* AT&T WE 32100 */
            2 => Self::Sparc,       /* SPARC */
            3 => Self::M386,        /* Intel Architecture */
            4 => Self::M68k,        /* Motorola 68000 */
            5 => Self::M88k,        /* Motorola 88000 */
            6 => Self::M860,        /* Number of valid machines */
            7 => Self::Mips,        /* Intel 80860 */
            8 => Self::MipsRS4Be,   /* MIPS RS3000 Big-Endian */
            10 => Self::Num,        /* MIPS RS4000 Big-Endian */
            11 => Self::ReservedLo, /* Reserved for future use */
            16 => Self::ReservedHi, /* Reserved for future use */
            _ => Self::Undefined,   /* Not defined */
        }
    }

    pub fn to(&self) -> T {
        match self {
            Self::None => 0,        /* No machine */
            Self::M32 => 1,         /* AT&T WE 32100 */
            Self::Sparc => 2,       /* SPARC */
            Self::M386 => 3,        /* Intel Architecture */
            Self::M68k => 4,        /* Motorola 68000 */
            Self::M88k => 5,        /* Motorola 88000 */
            Self::M860 => 6,        /* Number of valid machines */
            Self::Mips => 7,        /* Intel 80860 */
            Self::MipsRS4Be => 8,   /* MIPS RS3000 Big-Endian */
            Self::Num => 10,        /* MIPS RS4000 Big-Endian */
            Self::ReservedLo => 11, /* Reserved for future use */
            Self::ReservedHi => 16, /* Reserved for future use */
            Self::Undefined => 17,  /* Not defined */
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "No machine",
            Self::M32 => "AT&T WE 32100",
            Self::Sparc => "SPARC",
            Self::M386 => "Intel Architecture",
            Self::M68k => "Motorola 68000",
            Self::M88k => "Motorola 88000",
            Self::M860 => "Number of defined machines",
            Self::Mips => "Intel 80860",
            Self::MipsRS4Be => "MIPS RS3000 Big-Endian",
            Self::Num => "MIPS RS4000 Big-Endian",
            Self::ReservedLo => "Reserved for future use",
            Self::ReservedHi => "Reserved for future use",
            Self::Undefined => "Not defined",
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
