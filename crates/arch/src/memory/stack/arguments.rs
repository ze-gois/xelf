pub mod entry;

use crate::memory;
pub use entry::*;
use human::info;

#[derive(Debug)]
pub struct Vector<'e> {
    pub counter: u64,
    entries: *mut Entry<'e>,
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
    pub fn from_pointer(p: crate::Pointer) -> (Option<Self>, crate::Pointer) {
        let counter = unsafe { *p.0 };
        let entries = (0..counter).for_each(|i| unsafe {
            let argv_p = p.0.add(1);
            let argv_p = argv_p as *mut *mut u8;
            let argv_pe = argv_p.offset(i as isize);
            let argv_e = *argv_pe;

            let e = Entry::from_pointer(argv_e, i);
            // e.print();
            info!("\n{:?}\n", e);
        });
        // let vector = Self { counter, entries };
        let vector = Self::default();

        (Some(vector), crate::Pointer(p.0))
    }

    pub fn print(&self) {
        info!("{:?}\n", self);
    }
}
