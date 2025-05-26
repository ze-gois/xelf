#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Flag {
    None = 0,
    X = 0x1,               // Execute permission
    W = 0x2,               // Write permission
    R = 0x4,               // Read permission
    MASKOS = 0x00FF0000,   // These flag bits are reserved for environment-specific use
    MASKPROC = 0xFF000000, // These flag bits are reserved for processor-specific use
}

impl Flag {
    fn from(value: u32) -> Self {
        match value {
            0x1 => Self::X,
            0x2 => Self::W,
            0x4 => Self::R,
            0x00FF0000 => Self::MASKOS,
            0xFF000000 => Self::MASKPROC,
            _ => Self::None, // Default case
        }
    }

    pub fn to(self) -> u32 {
        self as u32
    }
}
