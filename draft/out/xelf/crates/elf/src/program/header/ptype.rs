#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PType {
    Null = 0,            // Unused entry
    Load = 1,            // Loadable segment
    Dynamic = 2,         // Dynamic linking tables
    Interp = 3,          // Program interpreter path name
    Note = 4,            // Note sections
    Shlib = 5,           // Reserved
    PHdr = 6,            // Program header table
    LoOs = 0x60000000,   // Environment-specific use
    HiOs = 0x6FFFFFFF,   // Environment-specific use
    LoProc = 0x70000000, // Processor-specific use
    HiProc = 0x7FFFFFF,  // Processor-specific use
}

impl PType {
    fn from(value: u32) -> Self {
        match value {
            0 => PType::Null,
            1 => PType::Load,
            2 => PType::Dynamic,
            3 => PType::Interp,
            4 => PType::Note,
            5 => PType::Shlib,
            6 => PType::PHdr,
            v if v >= 0x60000000 && v <= 0x6FFFFFFF => PType::LoOs,
            v if v >= 0x70000000 && v <= 0x7FFFFFF => PType::LoProc,
            _ => PType::Null, // Default case
        }
    }

    pub fn to(self) -> u32 {
        self as u32
    }
}
