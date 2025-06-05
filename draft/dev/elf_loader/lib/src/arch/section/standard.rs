use crate::section::header::Flag;
use crate::section::header::Type;

pub enum Special {
    Bss(Type, Flag),
    Comment(Type, Flag),
    Data(Type, Flag),
    Data1(Type, Flag),
    Debug(Type, Flag),
    Dynamic(Type, Flag),
    Hash(Type, Flag),
    Line(Type, Flag),
    Note(Type, Flag),
    RoData(Type, Flag),
    RoData1(Type, Flag),
    ShStrTab(Type, Flag),
    StrTab(Type, Flag),
    SymTab(Type, Flag),
    Text(Type, Flag),
}

impl Special {
    pub fn default(&self) -> Self {
        match self {
            Self::Bss(_, _) => Self::Bss(Type::NoBits, Flag::Allocate + Flag::Write),
            Self::Comment(_, _) => Self::Comment(Type::ProgBits, Flag::Blank),
            Self::Data(_, _) => Self::Data(Type::ProgBits, Flag::Allocate + Flag::Write),
            Self::Data1(_, _) => Self::Data1(Type::ProgBits, Flag::Allocate + Flag::Write),
            Self::Debug(_, _) => Self::Debug(Type::ProgBits, Flag::Blank),
            Self::Dynamic(_, _) => Self::Dynamic(Type::Dynamic, Flag::Blank),
            Self::Hash(_, _) => Self::Hash(Type::Hash, Flag::Allocate),
            Self::Line(_, _) => Self::Line(Type::ProgBits, Flag::Blank),
            Self::Note(_, _) => Self::Note(Type::Note, Flag::Blank),
            Self::RoData(_, _) => Self::RoData(Type::ProgBits, Flag::Allocate),
            Self::RoData1(_, _) => Self::RoData1(Type::ProgBits, Flag::Allocate),
            Self::ShStrTab(_, _) => Self::ShStrTab(Type::StrTab, Flag::Blank),
            Self::StrTab(_, _) => Self::StrTab(Type::StrTab, Flag::Blank),
            Self::SymTab(_, _) => Self::SymTab(Type::SymTab, Flag::Blank),
            Self::Text(_, _) => Self::Text(Type::ProgBits, Flag::Allocate + Flag::Execute),
        }
    }
}
