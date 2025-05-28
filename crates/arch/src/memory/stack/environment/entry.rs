#[repr(C)]
pub struct Entry<'a> {
    pub index: usize,
    pub pointer: *mut u8,
    pub key: &'a str,
    pub value: &'a str,
}
