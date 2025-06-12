use crate::dtype::Endianness;

use super::Header;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Entry {
    pub header: Header,
    pub content: *mut u8,
}

impl Entry {
    pub fn from_file_descriptor(
        file_descriptor: isize,
        endianness: Endianness,
    ) -> crate::Result<Self> {
        Ok(Entry {
            header: Header::from_file_descriptor(file_descriptor, endianness)?,
            content: core::ptr::null_mut(),
        })
    }
}

impl core::fmt::Debug for Entry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Entry")
            .field("header", &self.header)
            .field("content", &self.content)
            .finish()
    }
}

impl core::fmt::Display for Entry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
