pub mod atype;
pub mod entry;

pub use atype::Type;
pub use entry::*;

use human::info;

use core::arch::x86_64;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub count: Option<usize>,
    entries: *mut Entry,
}

impl Default for Vector {
    fn default() -> Self {
        Self {
            count: None,
            entries: 0x0 as *mut Entry,
        }
    }
}

impl Vector {
    pub fn from_pointer(auxv_pointer: crate::Pointer) -> Self {
        Self::default()
    }

    pub fn new(entries: *mut Entry) -> Self {
        Self {
            entries,
            count: None,
        }
    }

    pub fn count(&mut self) -> usize {
        if let Some(count) = self.count {
            return count;
        }

        let mut count = 0;
        let mut auxv = self.entries;
        loop {
            let auxv_entry = unsafe { *auxv.offset(count as isize) };

            if auxv_entry.atype == 0 {
                break;
            }

            count += 1;
        }

        self.count = Some(count);
        count
    }

    pub fn get_by_index(&mut self, index: usize) -> Option<*mut Entry> {
        if index < self.count() {
            return Some(unsafe { self.entries.offset(index as isize) });
        }
        None
    }

    pub fn get_by_type_id(&mut self, atype: usize) -> Option<*mut Entry> {
        for av in 0..self.count() {
            let entry = unsafe { self.entries.offset(av as isize) };

            if (unsafe { *entry }).atype == atype {
                return Some(entry);
            }
        }
        None
    }

    pub fn get_by_type(&mut self, atype: Type) -> Option<*mut Entry> {
        self.get_by_type_id(atype.to())
    }

    pub fn set_by_type_id(&mut self, atype: usize, value: usize) {
        if let Some(entry) = self.get_by_type_id(atype) {
            unsafe { (*entry).value = value };
        }
    }

    pub fn set_by_type(&mut self, atype: Type, value: usize) {
        self.set_by_type_id(atype.to(), value);
    }

    pub fn print(self) {
        info!("tobeaprint");
        return ();
        let count = self.count();

        (0..count).for_each(|av| {
            let auxv_entry = unsafe { self.entries.offset(av as isize) };

            let auxv_type = unsafe { Type::from((*auxv_entry).atype) as u64 };
            info!("Auxv: {} = ", auxv_type);
            if !unsafe { ((*auxv_entry).value as *const u8).is_null() } {
                // crate::print::print(auxv_entry.value as *const u8);
            } else {
                info!("NULL");
            }
            info!("'\n");
        })
    }
}
