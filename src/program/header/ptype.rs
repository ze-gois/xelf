use crate::arch::Word as T;

#[derive(PartialEq)]
pub enum Type {
    Null = 0,            // Unused entry
    Load = 1,            // Loadable segment
    Dynamic = 2,         // Dynamic linking tables
    Interp = 3,          // Program interpreter path name
    Note = 4,            // Note sections
    ShLib = 5,           // Reserved
    PHdr = 6,            // Program header table
    LoOs = 0x60000000,   // OS-specific lower bound
    HiOs = 0x6FFFFFFF,   // OS-specific upper bound
    LoProc = 0x70000000, // Processor-specific lower bound
    HiProc = 0x7FFFFFFF, // Processor-specific upper bound
    Undefined = 7,       // Undefined
}

impl Type {
    pub fn from(i: T) -> Self {
        match i {
            0 => Self::Null,
            1 => Self::Load,
            2 => Self::Dynamic,
            3 => Self::Interp,
            4 => Self::Note,
            5 => Self::ShLib,
            6 => Self::PHdr,
            0x60000000 => Self::LoOs,
            0x6FFFFFFF => Self::HiOs,
            0x70000000 => Self::LoProc,
            0x7FFFFFFF => Self::HiProc,
            _ => Self::Undefined,
        }
    }

    pub fn to(&self) -> T {
        match &self {
            Self::Null => 0,
            Self::Load => 1,
            Self::Dynamic => 2,
            Self::Interp => 3,
            Self::Note => 4,
            Self::ShLib => 5,
            Self::PHdr => 6,
            Self::LoOs => 0x60000000,
            Self::HiOs => 0x6FFFFFFF,
            Self::LoProc => 0x70000000,
            Self::HiProc => 0x7FFFFFFF,
            Self::Undefined => 7,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Null => "Unused entry",
            Self::Load => "Loadable segment",
            Self::Dynamic => "Dynamic linking tables",
            Self::Interp => "Program interpreter path name",
            Self::Note => "Note sections",
            Self::ShLib => "Reserved",
            Self::PHdr => "Program header table",
            Self::LoOs => "OS-specific lower bound",
            Self::HiOs => "OS-specific upper bound",
            Self::LoProc => "Processor-specific lower bound",
            Self::HiProc => "Processor-specific upper bound",
            Self::Undefined => "Undefined",
        }
    }

    pub fn as_acro(&self) -> &'static str {
        match self {
            Self::Null => "Null",
            Self::Load => "Load",
            Self::Dynamic => "Dynamic",
            Self::Interp => "Interp",
            Self::Note => "Note",
            Self::ShLib => "ShLib",
            Self::PHdr => "PHdr",
            Self::LoOs => "LoOs",
            Self::HiOs => "HiOs",
            Self::LoProc => "LoProc",
            Self::HiProc => "HiProc",
            Self::Undefined => "Undefined",
        }
    }
}
