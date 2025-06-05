use crate::dtype::Half as T;

#[derive(Clone, Copy)]
#[repr(C)]
/// # Object file type.
///
/// |Variant|Descriminator|Meaning|
/// |:--|:-:|:--|
/// |Type::None|0|No file type|
/// |Type::Rel|1|Relocatable object file|
/// |Type::Exec|2|Executable file|
/// |Type::Dyn|3|Shared object file|
/// |Type::Core|4|Core file|
/// |Type::LoOS|0xFE00|Environment-specific use lower boundary|
/// |Type::HiOS|0xFEFF|Environment-specific use upper boundary|
/// |Type::LoProc|0xFF00|Processor-specific use lower boundary|
/// |Type::HiProc|0xFFFF|Processor-specific use upper boundary|
pub enum Type {
    /// No file type
    None = 0,
    /// Relocatable object file
    Rel = 1,
    /// Executable file
    Exec = 2,
    /// Shared object file
    Dyn = 3,
    /// Core file
    Core = 4,
    /// Environment-specific use lower boundary
    LoOS = 0xFE00,
    /// Environment-specific use upper boundary
    HiOS = 0xFEFF,
    /// Processor-specific use lower boundary
    LoProc = 0xFF00,
    /// Processor-specific use upper boundary
    HiProc = 0xFFFF,
    /// Number of defined types
    Num = 5,
    /// Undefined use
    Undefined = 6,
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
            0xFE00 => Self::LoOS,
            0xFEFF => Self::HiOS,
            0xFF00 => Self::LoProc,
            0xFFFF => Self::HiProc,
            5 => Self::Num,
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
            Self::LoOS => 0xFE00,
            Self::HiOS => 0xFEFF,
            Self::LoProc => 0xFF00,
            Self::HiProc => 0xFFFF,
            Self::Num => 5,
            Self::Undefined => 6,
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
