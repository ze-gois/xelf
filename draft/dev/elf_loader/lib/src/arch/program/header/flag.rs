pub use crate::arch::Word as T;

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum Flag {
    Blank = 0x0,                       /* No rights */
    Execute = 0x1,                     /* Execute */
    Write = 0x2,                       /* Write */
    Read = 0x4,                        /* Read */
    ExecuteWrite = 0x3,                /* Execute and Write */
    ExecuteRead = 0x5,                 /* Execute and Read */
    WriteRead = 0x6,                   /* Write and Read */
    ExecuteWriteRead = 0x7,            /* Execute, Write and Read */
    OsBlank = 0x00FF0000,              /* Os-specific no rights */
    OsExecute = 0x00FF0001,            /* Os-specific Execute */
    OsWrite = 0x00FF0002,              /* Os-specific Write */
    OsRead = 0x00FF0004,               /* Os-specific Read */
    OsExecuteWrite = 0x00FF0003,       /* Os-specific Execute and Write */
    OsExecuteRead = 0x00FF0005,        /* Os-specific Execute and Read */
    OsWriteRead = 0x00FF0006,          /* Os-specific Write and Read */
    OsExecuteWriteRead = 0x00FF0007,   /* Os-specific Execute, Write and Read */
    ProcBlank = 0xFF000000,            /* Proc-specific no rights */
    ProcExecute = 0xFF000001,          /* Proc-specific Execute */
    ProcWrite = 0xFF000002,            /* Proc-specific Write */
    ProcRead = 0xFF000004,             /* Proc-specific Read */
    ProcExecuteWrite = 0xFF000003,     /* Proc-specific Execute and Write */
    ProcExecuteRead = 0xFF000005,      /* Proc-specific Execute and Read */
    ProcWriteRead = 0xFF000006,        /* Proc-specific Write and Read */
    ProcExecuteWriteRead = 0xFF000007, /* Proc-specific Execute, Write and Read */
    Undefined = 0x8,                   /* Proc-specific Undefined */
}

impl Flag {
    pub fn from(i: T) -> Self {
        match i {
            0x0 => Self::Blank,                       /* No rights */
            0x1 => Self::Execute,                     /* Execute */
            0x2 => Self::Write,                       /* Write */
            0x4 => Self::Read,                        /* Read */
            0x3 => Self::ExecuteWrite,                /* Execute and Write */
            0x5 => Self::ExecuteRead,                 /* Execute and Read */
            0x6 => Self::WriteRead,                   /* Write and Read */
            0x7 => Self::ExecuteWriteRead,            /* Execute, Write and Read */
            0x00FF0000 => Self::OsBlank,              /* Os-specific no rights */
            0x00FF0001 => Self::OsExecute,            /* Os-specific Execute */
            0x00FF0002 => Self::OsWrite,              /* Os-specific Write */
            0x00FF0004 => Self::OsRead,               /* Os-specific Read */
            0x00FF0003 => Self::OsExecuteWrite,       /* Os-specific Execute and Write */
            0x00FF0005 => Self::OsExecuteRead,        /* Os-specific Execute and Read */
            0x00FF0006 => Self::OsWriteRead,          /* Os-specific Write and Read */
            0x00FF0007 => Self::OsExecuteWriteRead,   /* Os-specific Execute, Write and Read */
            0xFF000000 => Self::ProcBlank,            /* Proc-specific no rights */
            0xFF000001 => Self::ProcExecute,          /* Proc-specific Execute */
            0xFF000002 => Self::ProcWrite,            /* Proc-specific Write */
            0xFF000004 => Self::ProcRead,             /* Proc-specific Read */
            0xFF000003 => Self::ProcExecuteWrite,     /* Proc-specific Execute and Write */
            0xFF000005 => Self::ProcExecuteRead,      /* Proc-specific Execute and Read */
            0xFF000006 => Self::ProcWriteRead,        /* Proc-specific Write and Read */
            0xFF000007 => Self::ProcExecuteWriteRead, /* Proc-specific Execute, Write and Read */
            _ => Self::Undefined,                     /* Proc-specific Undefined */
        }
    }

    pub fn to(&self) -> T {
        match &self {
            Self::Blank => 0x0,                       /* No rights */
            Self::Execute => 0x1,                     /* Execute */
            Self::Write => 0x2,                       /* Write */
            Self::Read => 0x4,                        /* Read */
            Self::ExecuteWrite => 0x3,                /* Execute and Write */
            Self::ExecuteRead => 0x5,                 /* Execute and Read */
            Self::WriteRead => 0x6,                   /* Write and Read */
            Self::ExecuteWriteRead => 0x7,            /* Execute, Write and Read */
            Self::OsBlank => 0x00FF0000,              /* Os-specific no rights */
            Self::OsExecute => 0x00FF0001,            /* Os-specific Execute */
            Self::OsWrite => 0x00FF0002,              /* Os-specific Write */
            Self::OsRead => 0x00FF0004,               /* Os-specific Read */
            Self::OsExecuteWrite => 0x00FF0003,       /* Os-specific Execute and Write */
            Self::OsExecuteRead => 0x00FF0005,        /* Os-specific Execute and Read */
            Self::OsWriteRead => 0x00FF0006,          /* Os-specific Write and Read */
            Self::OsExecuteWriteRead => 0x00FF0007,   /* Os-specific Execute, Write and Read */
            Self::ProcBlank => 0xFF000000,            /* Proc-specific no rights */
            Self::ProcExecute => 0xFF000001,          /* Proc-specific Execute */
            Self::ProcWrite => 0xFF000002,            /* Proc-specific Write */
            Self::ProcRead => 0xFF000004,             /* Proc-specific Read */
            Self::ProcExecuteWrite => 0xFF000003,     /* Proc-specific Execute and Write */
            Self::ProcExecuteRead => 0xFF000005,      /* Proc-specific Execute and Read */
            Self::ProcWriteRead => 0xFF000006,        /* Proc-specific Write and Read */
            Self::ProcExecuteWriteRead => 0xFF000007, /* Proc-specific Execute, Write and Read */
            Self::Undefined => 0x8,                   /* Proc-specific Undefined */
        }
    }

    pub fn as_str(&self) -> &'static str {
        match &self {
            Self::Blank => "No rights",
            Self::Execute => "Execute",
            Self::Write => "Write",
            Self::Read => "Read",
            Self::ExecuteWrite => "Execute and Write",
            Self::ExecuteRead => "Execute and Read",
            Self::WriteRead => "Write and Read",
            Self::ExecuteWriteRead => "Execute, Write and Read",
            Self::OsBlank => "No rights (OS)",
            Self::OsExecute => "Execute (OS)",
            Self::OsWrite => "Write (OS)",
            Self::OsRead => "Read (OS)",
            Self::OsExecuteWrite => "Execute and Write (OS)",
            Self::OsExecuteRead => "Execute and Read (OS)",
            Self::OsWriteRead => "Write and Read (OS)",
            Self::OsExecuteWriteRead => "Execute, Write and Read (OS)",
            Self::ProcBlank => "No rights (Proc)",
            Self::ProcExecute => "Execute (Proc)",
            Self::ProcWrite => "Write (Proc)",
            Self::ProcRead => "Read (Proc)",
            Self::ProcExecuteWrite => "Execute and Write (Proc)",
            Self::ProcExecuteRead => "Execute and Read (Proc)",
            Self::ProcWriteRead => "Write and Read (Proc)",
            Self::ProcExecuteWriteRead => "Execute, Write and Read (Proc)",
            Self::Undefined => "Undefined",
        }
    }

    pub fn as_str_acronym(&self) -> &'static str {
        match &self {
            Self::Blank => "          ",
            Self::Execute => " X        ",
            Self::Write => "  W       ",
            Self::Read => "   R      ",
            Self::ExecuteWrite => " XW       ",
            Self::ExecuteRead => " X R      ",
            Self::WriteRead => "  WR      ",
            Self::ExecuteWriteRead => " EWR      ",
            Self::OsBlank => "    ( OS )",
            Self::OsExecute => " X  ( OS )",
            Self::OsWrite => "  W ( OS )",
            Self::OsRead => "   R( OS )",
            Self::OsExecuteWrite => " XW ( OS )",
            Self::OsExecuteRead => " W R( OS )",
            Self::OsWriteRead => "  AR( OS )",
            Self::OsExecuteWriteRead => " XWR( OS )",
            Self::ProcBlank => "    (Proc)",
            Self::ProcExecute => " X  (Proc)",
            Self::ProcWrite => "  W (Proc)",
            Self::ProcRead => "   R(Proc)",
            Self::ProcExecuteWrite => " XW (Proc)",
            Self::ProcExecuteRead => " X R(Proc)",
            Self::ProcWriteRead => "  WR(Proc)",
            Self::ProcExecuteWriteRead => " XWR(Proc)",
            Self::Undefined => "~Undefined",
        }
    }

    pub fn from_posix(i: T) -> Self {
        match i {
            0x0 => Self::Blank,
            0x1 => Self::Read,
            0x2 => Self::Write,
            0x4 => Self::Execute,
            _ => {
                let read = Self::from_posix(i & Self::Read.to_posix());
                let write = Self::from_posix(i & Self::Write.to_posix());
                let execute = Self::from_posix(i & Self::Execute.to_posix());

                read + write + execute
            }
        }
    }

    pub fn to_posix(&self) -> T {
        match self {
            Self::Blank => 0x0,
            Self::Read => 0x1,
            Self::Write => 0x2,
            Self::Execute => 0x4,
            Self::WriteRead => 0x3,
            Self::ExecuteRead => 0x5,
            Self::ExecuteWrite => 0x6,
            Self::ExecuteWriteRead => 0x7,
            _ => 0x0,
        }
    }
}

use core::ops::Add;

impl Add for Flag {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Flag::from(self.to() | rhs.to())
    }
}

use core::ops::Sub;

impl Sub for Flag {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Flag::from(self.to() & !rhs.to())
    }
}

use core::ops::BitAnd;

impl BitAnd<T> for Flag {
    type Output = Self;

    fn bitand(self, rhs: T) -> Self::Output {
        Flag::from(self.to() & rhs)
    }
}

impl BitAnd<Flag> for Flag {
    type Output = bool;

    fn bitand(self, rhs: Flag) -> Self::Output {
        self.to() & rhs.to() != 0
    }
}

use core::fmt::{Debug, Display, Formatter, Result};

type E = Flag;

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
