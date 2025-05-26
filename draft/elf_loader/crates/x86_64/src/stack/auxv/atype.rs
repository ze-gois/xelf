pub enum Type {
    Null = 0,
    PHdr = 3,
    PHNum = 5,
    PHEnt = 4,
    Entry = 9,
    ExecFn = 31,
    Base = 7,
    DontCare = 0xFFFFFFFF,
}

impl Type {
    pub fn from(at: usize) -> Self {
        match at {
            0 => Self::Null,
            3 => Self::PHdr,
            5 => Self::PHNum,
            4 => Self::PHEnt,
            9 => Self::Entry,
            31 => Self::ExecFn,
            7 => Self::Base,
            _ => Self::DontCare,
        }
    }

    pub fn to(self) -> usize {
        self as usize
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Null => "Null",
            Self::PHdr => "PHdr",
            Self::PHNum => "PHNum",
            Self::PHEnt => "PHEnt",
            Self::Entry => "Entry",
            Self::ExecFn => "ExecFn",
            Self::Base => "Base",
            Self::DontCare => "DontCare",
        }
    }
}
