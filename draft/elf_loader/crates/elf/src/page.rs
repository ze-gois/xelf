pub const SIZE: usize = 0x1000;
pub const ALIGN: usize = SIZE - 1;
pub const MASK: usize = !ALIGN;

pub const ORIGIN: usize = 0x100000;

pub fn floor(addr: usize) -> usize {
    addr & MASK
}

pub fn ceil(addr: usize) -> usize {
    (addr + ALIGN) & MASK
}

pub fn offset(addr: usize) -> usize {
    addr & ALIGN
}
