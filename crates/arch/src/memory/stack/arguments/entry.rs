#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Entry<'a> {
    pub index: u64,
    pub pointer: *mut *mut u8,
    pub value: &'a str,
}

impl<'a> Entry<'a> {
    pub fn from_pointer(p: *mut *mut u8, i: u64) -> Self {
        let v;
        unsafe {
            let l = crate::memory::misc::length(*p as *const u8);
            let s = core::slice::from_raw_parts_mut(*p, l);
            v = core::str::from_utf8(s).unwrap();
        }
        Entry {
            index: i,
            pointer: p,
            value: v,
        }
    }

    /// Returns the string value of this entry
    pub fn value(&self) -> &str {
        self.value
    }
    
    /// Returns the index of this entry in the arguments vector
    pub fn index(&self) -> u64 {
        self.index
    }
    
    /// Creates a byte slice from the underlying pointer
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            let len = crate::memory::misc::length(*self.pointer as *const u8);
            core::slice::from_raw_parts(*self.pointer, len)
        }
    }
}
