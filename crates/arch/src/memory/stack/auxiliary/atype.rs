pub enum Type {
    Null = 0,    /* End of vector */
    Ignore = 1,  /* Entry should be ignored */
    ExecFD = 2,  /* File descriptor of program */
    PHdr = 3,    /* Program headers for program */
    PHEnt = 4,   /* Size of program header entry */
    PHNum = 5,   /* Number of program headers */
    PageSz = 6,  /* System page size */
    Base = 7,    /* Base address of interpreter */
    Flags = 8,   /* Flags */
    Entry = 9,   /* Entry point of program */
    NotELF = 10, /* Program is not ELF */
    UID = 11,    /* Real uid */
    EUID = 12,   /* Effective uid */
    GID = 13,    /* Real gid */
    EGID = 14,   /* Effective gid */
    ClkTck = 17, /* Frequency of times() */
    ExecFn = 31,
    SysInfo = 32,
    SysInfoEhdr = 33,
    DontCare = 0xFFFFFFFF,
}

impl Type {
    pub fn from(at: usize) -> Self {
        match at {
            0 => Self::Null,
            1 => Self::Ignore,
            2 => Self::ExecFD,
            3 => Self::PHdr,
            4 => Self::PHEnt,
            5 => Self::PHNum,
            6 => Self::PageSz,
            7 => Self::Base,
            8 => Self::Flags,
            9 => Self::Entry,
            10 => Self::NotELF,
            11 => Self::UID,
            12 => Self::EUID,
            13 => Self::GID,
            14 => Self::EGID,
            17 => Self::ClkTck,
            31 => Self::ExecFn,
            32 => Self::SysInfo,
            33 => Self::SysInfoEhdr,
            _ => Self::DontCare,
        }
    }

    pub fn to(self) -> usize {
        self as usize
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Null => "Null",
            Self::Ignore => "Ignore",
            Self::ExecFD => "ExecFD",
            Self::PHdr => "PHdr",
            Self::PHEnt => "PHEnt",
            Self::PHNum => "PHNum",
            Self::PageSz => "PageSz",
            Self::Base => "Base",
            Self::Flags => "Flags",
            Self::Entry => "Entry",
            Self::NotELF => "NotELF",
            Self::UID => "UID",
            Self::EUID => "EUID",
            Self::GID => "GID",
            Self::EGID => "EGID",
            Self::ClkTck => "ClkTck",
            Self::ExecFn => "ExecFn",
            Self::SysInfo => "SysInfo",
            Self::SysInfoEhdr => "SysInfoEhdr",
            Self::DontCare => "DontCare",
        }
    }
}
