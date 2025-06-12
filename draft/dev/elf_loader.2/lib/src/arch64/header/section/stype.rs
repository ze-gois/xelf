use crate::arch64::data::Word as T;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Null = 0,            /* Marks an unused section header */
    ProgBits = 1,        /* Contains information defined by the program */
    SymTab = 2,          /* Contains a linker symbol table */
    StrTab = 3,          /* Contains a string table */
    Rela = 4,            /* Contains “Rela” type relocation entries */
    Hash = 5,            /* Contains a symbol hash table */
    Dynamic = 6,         /* Contains dynamic linking tables */
    Note = 7,            /* Contains note information */
    NoBits = 8,          /* Contains uninitialized space; does not occupy any space in the file */
    Rel = 9,             /* Contains “Rel” type relocation  entries */
    ShLib = 10,          /* Reserved */
    DynSym = 11,         /* Contains a dynamic loader symbol table */
    LoProc = 0x70000000, /* Environment-specific use lower bound */
    HiProc = 0x7FFFFFFF, /* Environment-specific use upper bound */
    LoUser = 0x80000000, /* Processor-specific use lower bound */
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
            10 => Self::ShLib,
            11 => Self::DynSym,
            0x70000000 => Self::LoProc,
            0x7FFFFFFF => Self::HiProc,
            0x80000000 => Self::LoUser,
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
            Self::ShLib => 10,
            Self::DynSym => 11,
            Self::LoProc => 0x70000000,
            Self::HiProc => 0x7FFFFFFF,
            Self::LoUser => 0x80000000,
            Self::HiUser => 0xFFFFFFFF,
            Self::Undefined => 12,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Null => "Null (Marks an unused section header)",
            Self::ProgBits => "ProgBits (Contains information defined by the program)",
            Self::SymTab => "SymTab (Contains a linker symbol table)",
            Self::StrTab => "StrTab (Contains a string table)",
            Self::Rela => "Rela (Contains “Rela” type relocation entries)",
            Self::Hash => "Hash (Contains a symbol hash table)",
            Self::Dynamic => "Dynamic (Contains dynamic linking tables)",
            Self::Note => "Note (Contains note information)",
            Self::NoBits => {
                "NoBits (Contains uninitialized space; does not occupy any space in the file)"
            }
            Self::Rel => "Rel (Contains “Rel” type relocation  entries)",
            Self::ShLib => "ShLib (Reserved)",
            Self::DynSym => "DynSym (Contains a dynamic loader symbol table)",
            Self::LoProc => "LoProc (Environment-specific use lower bound)",
            Self::HiProc => "HiProc (Environment-specific use upper bound)",
            Self::LoUser => "LoUser (Processor-specific use lower bound)",
            Self::HiUser => "HiUser (Processor-specific use upper bound)",
            Self::Undefined => "Undefined (Undefined)",
        }
    }
}

use core::fmt::{Debug, Display, Formatter, Result};

type E = Type;

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
