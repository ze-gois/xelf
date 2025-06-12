use crate::arch64::data::UnsignedChar as T;

#[derive(Clone, Copy)]
pub enum OsAbi {
    // NONE = 0,         /* UNIX System V ABI */
    // LINUX = 3,        /* Compatibility alias.  */
    SYSV = 0,         /* Alias.  */
    HPUX = 1,         /* HP-UX */
    NETBSD = 2,       /* NetBSD.  */
    GNU = 3,          /* Object uses GNU ELF extensions.  */
    SOLARIS = 6,      /* Sun Solaris.  */
    AIX = 7,          /* IBM AIX.  */
    IRIX = 8,         /* SGI Irix.  */
    FREEBSD = 9,      /* FreeBSD.  */
    TRU64 = 10,       /* Compaq TRU64 UNIX.  */
    MODESTO = 11,     /* Novell Modesto.  */
    OPENBSD = 12,     /* OpenBSD.  */
    ARMAEABI = 64,    /* ARM EABI */
    ARM = 97,         /* ARM */
    UNDEFINED = 98,   /* NotSpecificed */
    STANDALONE = 255, /* Standalone (embedded) application */
}

type E = OsAbi;

impl E {
    pub fn from(i: T) -> Self {
        match i {
            0 => Self::SYSV,
            1 => Self::HPUX,
            2 => Self::NETBSD,
            3 => Self::GNU,
            6 => Self::SOLARIS,
            7 => Self::AIX,
            8 => Self::IRIX,
            9 => Self::FREEBSD,
            10 => Self::TRU64,
            11 => Self::MODESTO,
            12 => Self::OPENBSD,
            64 => Self::ARMAEABI,
            97 => Self::ARM,
            255 => Self::STANDALONE,
            _ => Self::UNDEFINED,
        }
    }

    pub fn to(&self) -> T {
        match self {
            Self::SYSV => 0,
            Self::HPUX => 1,
            Self::NETBSD => 2,
            Self::GNU => 3,
            Self::SOLARIS => 6,
            Self::AIX => 7,
            Self::IRIX => 8,
            Self::FREEBSD => 9,
            Self::TRU64 => 10,
            Self::MODESTO => 11,
            Self::OPENBSD => 12,
            Self::ARMAEABI => 64,
            Self::ARM => 97,
            Self::STANDALONE => 255,
            Self::UNDEFINED => 98,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SYSV => "UNIX System V ABI",
            Self::HPUX => "HP-UX ABI",
            Self::NETBSD => "NetBSD ABI",
            Self::GNU => "Object uses GNU ELF extensions ABI",
            Self::SOLARIS => "Sun Solaris ABI",
            Self::AIX => "IBM AIX ABI",
            Self::IRIX => "SGI Irix ABI",
            Self::FREEBSD => "FreeBSD ABI",
            Self::TRU64 => "Compaq TRU64 UNIX ABI",
            Self::MODESTO => "Novell Modesto ABI",
            Self::OPENBSD => "OpenBSD ABI",
            Self::ARMAEABI => "ARM EABI ABI",
            Self::ARM => "ARM ABI",
            Self::STANDALONE => "Standalone (embedded) application ABI",
            Self::UNDEFINED => "Undefined OS ABI",
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
