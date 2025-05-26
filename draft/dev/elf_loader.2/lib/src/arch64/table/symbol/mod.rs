use crate::arch64::data;

pub struct Symbol {
    pub name: data::Word,          /* Symbol name  */
    pub value: data::Addr,         /* Symbol value */
    pub size: data::XWord,         /* Size of object (e.g., common) */
    pub info: data::UnsignedChar,  /* Type and Binding attributes */
    pub other: data::UnsignedChar, /* Reserved */
    pub shndx: data::Half,         /* Section table index */
}
