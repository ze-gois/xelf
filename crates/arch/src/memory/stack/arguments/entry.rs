#[repr(C)]
#[derive(Debug)]
pub struct Entry<'a> {
    pub index: u64,
    pub pointer: *mut u8,
    pub value: &'a str,
}

impl<'a> Entry<'a> {
    pub fn from_pointer(p: *mut u8, i: u64) -> Self {
        let v;
        unsafe {
            let l = crate::memory::misc::length(p as *const u8);
            let s = core::slice::from_raw_parts_mut(p, l);
            v = core::str::from_utf8(s).unwrap();
        }
        Entry {
            index: i,
            pointer: p,
            value: v,
        }
    }
}
