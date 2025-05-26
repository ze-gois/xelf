use crate::arch32::data::Word as T;

pub enum Link {
    Unused(T),
    StringTableIndex(T),
    SymbolTableIndex(T),
    HashTableIndex(T),
    OperatingSystem(T),
}

impl Link {
    pub fn to(&self) -> T {
        match *self {
            Self::Unused(i) => i,
            Self::OperatingSystem(i) => i,
            Self::StringTableIndex(i) => i,
            Self::SymbolTableIndex(i) => i,
            Self::HashTableIndex(i) => i,
        }
    }

    pub fn as_string(&self) -> String {
        match *self {
            Self::Unused(i) => format!("Unused ({i})"),
            Self::StringTableIndex(i) => format!("String table index ({i})"),
            Self::SymbolTableIndex(i) => format!("Symbol table index ({i})"),
            Self::HashTableIndex(i) => format!("Hash table index ({i})"),
            Self::OperatingSystem(i) => format!("OS index {i}"),
        }
    }
}
