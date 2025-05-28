pub mod entry;

pub use crate::memory::misc;
pub use entry::*;
use human::info;

pub struct Vector<'e> {
    pub counter: usize,
    pub entries: *mut Entry<'e>,
}

impl<'e> Default for Vector<'e> {
    fn default() -> Self {
        Self {
            counter: 0,
            entries: 0x0 as *mut Entry,
        }
    }
}

impl<'e> Vector<'e> {
    pub fn print(&self) {
        info!("Environment {{}}\n");
    }
}
