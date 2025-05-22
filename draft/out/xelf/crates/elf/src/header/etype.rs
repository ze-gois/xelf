pub enum EType {
    NONE = 0,        // No file type
    REL = 1,         // Relocatable object file
    EXEC = 2,        // Executable file
    DYN = 3,         // Shared object file
    CORE = 4,        // Core file
    LOOS = 0xFE00,   // Environment-specific use
    HIOS = 0xFEFF,   //
    LOPROC = 0xFF00, // Processor-specific use
    HIPROC = 0xFFFF, //
}

impl EType {
    pub fn from(value: u16) -> Self {
        match value {
            0 => EType::NONE,
            1 => EType::REL,
            2 => EType::EXEC,
            3 => EType::DYN,
            4 => EType::CORE,
            v if v >= 0xFE00 && v <= 0xFEFF => EType::LOOS,
            v if v >= 0xFF00 => EType::LOPROC, //v if v >= 0xFF00 && v <= 0xFFFF => EType::LOPROC,
            _ => EType::NONE,                  // Default case
        }
    }

    pub fn to(self) -> u16 {
        self as u16
    }
}
