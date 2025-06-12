use crate::dtype;

#[repr(C)]
pub struct Rel {
    pub offset: dtype::Addr, /* Address of reference */
    pub info: dtype::XWord,  /* Symbol index and type of relocation */
}
