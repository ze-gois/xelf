use crate::dtype;

#[repr(C)]
pub struct Rela {
    pub offset: dtype::Addr,   /* Address of reference */
    pub info: dtype::XWord,    /* Symbol index and type of relocation */
    pub addend: dtype::SXWord, /* Constant part of expression */
}
