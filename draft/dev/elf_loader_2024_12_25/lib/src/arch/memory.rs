use std::ops::Not;

pub use super::dtype;

pub const PAGE_SIZE: dtype::Addr = 0x1000;
pub const PAGE_ALIGNMENT: dtype::Addr = PAGE_SIZE - 1;
pub const PAGE_MASK: dtype::Addr = !PAGE_ALIGNMENT;

pub const DYNAMIC_OFFSET: dtype::Addr = 0x400000;

pub fn round_address_to_lower_page_boundary(address: dtype::Addr) -> dtype::Addr {
    address & PAGE_MASK
}

pub fn align_to_lower_page(address: dtype::Addr) -> dtype::Addr {
    (address + PAGE_ALIGNMENT) & PAGE_ALIGNMENT.not()
}

pub fn truncate_to_page(address: dtype::Addr) -> dtype::Addr {
    address & PAGE_ALIGNMENT.not()
}
