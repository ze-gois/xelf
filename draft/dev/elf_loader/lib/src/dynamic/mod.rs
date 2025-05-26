use crate::arch;

pub mod entry;
pub use entry::Entry;

pub struct Table {
    pub offset: arch::Off,
    pub entries: Vec<Entry>,
}
