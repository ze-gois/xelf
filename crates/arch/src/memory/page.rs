use core::ops::Not;

// pub use super::dtype;

pub mod dtype {
    pub type Addr = usize;
}

pub const SIZE: dtype::Addr = 0x1000;
pub const ALIGNMENT: dtype::Addr = SIZE - 1;
pub const MASK: dtype::Addr = !ALIGNMENT;

pub const DYNAMIC_OFFSET: dtype::Addr = 0x400000;

pub fn round_address_to_lower_page_boundary(address: dtype::Addr) -> dtype::Addr {
    address & MASK
}

pub fn align_to_lower_page(address: dtype::Addr) -> dtype::Addr {
    (address + ALIGNMENT) & ALIGNMENT.not()
}

pub fn truncate_to_page(address: dtype::Addr) -> dtype::Addr {
    address & ALIGNMENT.not()
}
