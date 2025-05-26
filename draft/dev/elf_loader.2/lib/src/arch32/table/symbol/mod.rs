use crate::arch32::data;

pub struct Symbol {
    pub name: data::Word,
    pub value: data::Addr,
    pub size: data::Word,
    pub info: data::UnsignedChar,
    pub other: data::UnsignedChar,
    pub shndx: data::Half,
}
