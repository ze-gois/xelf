#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Entry {
    pub atype: usize,
    pub value: usize,
}

impl Entry {
    pub fn is_null(&self) -> bool {
        self.atype == 0
    }
}
