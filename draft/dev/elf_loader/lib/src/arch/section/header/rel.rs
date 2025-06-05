use crate::arch;

#[repr(C)]
pub struct Rel {
    pub offset: arch::Addr, /* Address of reference */
    pub info: arch::XWord,  /* Symbol index and type of relocation */
}
