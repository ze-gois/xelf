use crate::arch;

#[repr(C)]
pub struct Entry {
    pub name: arch::Word,          /* Symbol name */
    pub info: arch::UnsignedChar,  /* Type and Binding attributes */
    pub other: arch::UnsignedChar, /* Reserved */
    pub shndx: arch::Half,         /* Section table index */
    pub value: arch::Addr,         /* Symbol value */
    pub size: arch::XWord,         /* Size of object (e.g., common) */
}
