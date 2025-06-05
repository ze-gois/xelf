use crate::arch;

#[repr(C)]
pub union Union {
    value: arch::XWord,
    pointer: arch::Addr,
}

#[repr(C)]
pub struct Entry {
    tag: arch::SXWord,
    pv: Union,
}
