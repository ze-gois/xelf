use super::Header;

pub struct Entry {
    pub header: Header,
    pub content: Vec<u8>,
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
        write!(
            f,
            "Entry {{ header: {}, content: {} bytes }}",
            self.header,
            self.content.len()
        )
    }
}
