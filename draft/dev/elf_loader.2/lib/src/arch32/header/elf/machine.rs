pub use crate::arch32::data::Half as T;

/* Legal values for e_machine (architecture).  */
/* reserved 11-14 */
/* reserved 16 */
/* reserved 24-35 */
/* reserved 121-130 */
/* reserved 145-159 */
/* reserved 182 */
/* reserved 184 */
/* reserved 206-209 */
/* reserved 225-242 */
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
    Mipsrs4Be = 10,  /* MIPS RS4000 Big-Endian */
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
            7 => Self::M860,        /* Intel 80860 */
            8 => Self::Mips,        /* MIPS RS3000 Big-Endian */
            10 => Self::Mipsrs4Be,  /* MIPS RS4000 Big-Endian */
            6 => Self::Num,         /* Number of valid machines */
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
            Self::M860 => 7,        /* Intel 80860 */
            Self::Mips => 8,        /* MIPS RS3000 Big-Endian */
            Self::Mipsrs4Be => 10,  /* MIPS RS4000 Big-Endian */
            Self::Num => 6,         /* Number of valid machines */
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
            Self::M860 => "Intel 80860",
            Self::Mips => "MIPS RS3000 Big-Endian",
            Self::Mipsrs4Be => "MIPS RS4000 Big-Endian",
            Self::Num => "Number of defined machines",
            Self::ReservedLo => "Reserved for future use",
            Self::ReservedHi => "Reserved for future use",
            Self::Undefined => "Not defined",
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
