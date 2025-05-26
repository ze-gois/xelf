use crate::arch32::data::Off as T;

pub enum Index {
    LoProc = 0xFF00, /* Processor-specific lower bound */
    // Proc = 0xFF01,      /* Processor-specific */
    HiProc = 0xFF1F, /* Processor-specific upper bound */
    LoOs = 0xFF20,   /* Environment-specific lower bound */
    // Os = 0xFF21,        /* Operating system specific */
    HiOs = 0xFF3F,   /* Environment-specific upper bound */
    Abs = 0xFFF1,    /* Absolute value */
    Common = 0xFFF2, /* Common block symbol */
    // LoReserve = 0xff00, /* Reserved lower bound */
    // Reserved = 0xFF01,  /* Reserved */
    // HiReserve = 0xFFFF, /* Reserved upper bound */
    Undefined = 0, /* Undefined/meaningless section */
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
            Self::Undefined => "Undefined/meaningless section",
        }
    }
}
