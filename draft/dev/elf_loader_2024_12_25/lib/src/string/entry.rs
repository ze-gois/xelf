use crate::arch;

#[derive(Clone)]
pub struct Entry {
    pub offset: arch::Off,
    pub content: String,
}
