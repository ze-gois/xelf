use crate::arch32::data::Word as T;

#[derive(Clone, Copy, PartialEq)]
pub enum Type {
    Null = 0,            /* Marks an unused section header */
    ProgBits = 1,        /* Program-defined information  */
    SymTab = 2,          /* Linker symbol table */
    StrTab = 3,          /* String table */
    Rela = 4,            /* Relocation entries */
    Hash = 5,            /* Symbol hash table */
    Dynamic = 6,         /* Dynamic linking tables */
    Note = 7,            /* Note information */
    NoBits = 8,          /* Uninitialized space */
    Rel = 9,             /* Relocation without addends */
    Shlib = 10,          /* Unespecific semantics */
    DynSym = 11,         /* Dynamic loader symbol table */
    LoProc = 0x70000000, /* Environment-specific use lower bound */
    Proc = 0x70000001,   /* Environment-specific */
    HiProc = 0x7FFFFFFF, /* Environment-specific use upper bound */
    LoUser = 0x80000000, /* Processor-specific use lower bound */
    User = 0x80000001,   /* Processor-specific */
    HiUser = 0xFFFFFFFF, /* Processor-specific use upper bound */
    Undefined = 12,      /* Undefined */
}

impl Type {
    pub fn from(i: T) -> Self {
        match i {
            0 => Self::Null,
            1 => Self::ProgBits,
            2 => Self::SymTab,
            3 => Self::StrTab,
            4 => Self::Rela,
            5 => Self::Hash,
            6 => Self::Dynamic,
            7 => Self::Note,
            8 => Self::NoBits,
            9 => Self::Rel,
            10 => Self::Shlib,
            11 => Self::DynSym,
            0x70000000 => Self::LoProc,
            0x70000001..0x7FFFFFFF => Self::Proc,
            0x7FFFFFFF => Self::HiProc,
            0x80000000 => Self::LoUser,
            0x80000001..0xFFFFFFFF => Self::User,
            0xFFFFFFFF => Self::HiUser,
            _ => Self::Undefined,
        }
    }

    pub fn to(&self) -> T {
        match self {
            Self::Null => 0,
            Self::ProgBits => 1,
            Self::SymTab => 2,
            Self::StrTab => 3,
            Self::Rela => 4,
            Self::Hash => 5,
            Self::Dynamic => 6,
            Self::Note => 7,
            Self::NoBits => 8,
            Self::Rel => 9,
            Self::Shlib => 10,
            Self::DynSym => 11,
            Self::LoProc => 0x70000000,
            Self::Proc => 0x70000001,
            Self::HiProc => 0x7FFFFFFF,
            Self::LoUser => 0x80000000,
            Self::User => 0x80000001,
            Self::HiUser => 0xFFFFFFFF,
            Self::Undefined => 12,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Null => "(Null) Unused",
            Self::ProgBits => "(ProgBits) program-defined Information",
            Self::SymTab => "(SymTab) Linker symbol table",
            Self::StrTab => "(StrTab) String table",
            Self::Rela => "(Rela) Relocation entries",
            Self::Hash => "(Hash) Symbol hash table",
            Self::Dynamic => "(Dynamic) Dynamic linking tables",
            Self::Note => "(Note) note information",
            Self::NoBits => "(NoBits) Uninitialized space",
            Self::Rel => "(Rel) Relocation entries",
            Self::Shlib => "(Shlib) Unspecified semantics",
            Self::DynSym => "(DynSym) Dynamic loader symbol table",
            Self::LoProc => "(LoProc) Environment-specific use lower bound",
            Self::Proc => "(Proc) Environment-specific use",
            Self::HiProc => "(HiProc) Environment-specific use upper bound",
            Self::LoUser => "(LoUser) Processor-specific use lower bound",
            Self::User => "(User) Processor-specific use",
            Self::HiUser => "(HiUser) Processor-specific use upper bound",
            Self::Undefined => "(Undefined) Undefined",
        }
    }
}

type E = Type;

use std::fmt::{Debug, Display, Formatter, Result};

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
