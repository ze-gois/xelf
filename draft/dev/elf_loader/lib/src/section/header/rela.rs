use crate::arch;

#[repr(C)]
pub struct Rela {
    pub offset: arch::Addr,   /* Address of reference */
    pub info: arch::XWord,    /* Symbol index and type of relocation */
    pub addend: arch::SXWord, /* Constant part of expression */
}
