use crate::arch::Word as T;

/// # Section indices
///
/// Section index 0, and indices whithin 0xFF00–0xFFFF
/// are reserved for special purposes.
#[repr(C)]
pub enum Index {
    /// Undefined/meaningless section
    Undefined = 0,
    /// Processor-specific lower bound
    LoProc = 0xFF00,
    /// Processor-specific
    // Proc = 0xFF01,
    /// Processor-specific upper bound
    HiProc = 0xFF1F,
    /// Environment-specific lower bound
    LoOs = 0xFF20,
    // Operating system specific
    // Os = 0xFF21,
    /// Environment-specific upper bound
    HiOs = 0xFF3F,
    /// Absolute value
    Abs = 0xFFF1,
    /// Common block symbol
    Common = 0xFFF2,
    // Reserved lower bound
    // LoReserve = 0xff00,
    // Reserved
    // Reserved = 0xFF01,
    // Reserved upper bound
    // HiReserve = 0xFFFF,
}

impl Index {
    pub fn from(i: T) -> Self {
        match i {
            0xFF00 => Self::LoProc,
            // 0xFF00..=0xFF1F => Self::Proc,
            0xFF1F => Self::HiProc,
            0xFF20 => Self::LoOs,
            // 0xFF20..=0xFF3F => Self::Os,
            0xFF3F => Self::HiOs,
            0xFFF1 => Self::Abs,
            0xFFF2 => Self::Common,
            //0xFF00..=0xFFFF => Self::Reserved,
            _ => Self::Undefined,
        }
    }

    pub fn to(&self) -> T {
        match &self {
            Self::LoProc => 0xFF00,
            // Self::Proc => 0xFF01,
            Self::HiProc => 0xFF1F,
            Self::LoOs => 0xFF20,
            // Self::Os => 0xFF21,
            Self::HiOs => 0xFF3F,
            Self::Abs => 0xFFF1,
            Self::Common => 0xFFF2,
            //Self::LoReserve => 0xff00,
            //Self::Reserved => 0xFF01,
            //Self::HiReserve => 0xFFFF
            Self::Undefined => 0,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match &self {
            Self::LoProc => "Processor-specific lower bound",
            // Self::Proc => "Processor-specific",
            Self::HiProc => "Processor-specific upper bound",
            Self::LoOs => "Environment-specific lower bound",
            // Self::Os => "Operating system specific",
            Self::HiOs => "Environment-specific upper bound",
            Self::Abs => "Absolute value",
            Self::Common => "Common block symbol",
            // Self::LoReserve => "Reserved lower bound",
            // Self::Reserved => "Reserved",
            // Self::HiReserve => "Reserved upper bound",
            Self::Undefined => "Undefined/meaningless",
        }
    }
}
