pub mod atype;
pub mod entry;

pub use atype::Type;
pub use entry::*;

use human::info;

use core::arch::x86_64;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub pointer: crate::Pointer,
    pub counter: Option<usize>,
    pub entries: *mut Entry,
}

impl Default for Vector {
    fn default() -> Self {
        Self {
            counter: None,
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
            counter: None,
        }
    }

    pub fn counter(&mut self) -> usize {
        if let Some(counter) = self.counter {
            return counter;
        }

        let mut counter = 0;
        let mut auxv = self.entries;
        loop {
            let auxv_entry = unsafe { *auxv.offset(counter as isize) };

            if auxv_entry.atype == 0 {
                break;
            }

            counter += 1;
        }

        self.counter = Some(counter);
        counter
    }

    pub fn get_by_index(&mut self, index: usize) -> Option<*mut Entry> {
        if index < self.counter() {
            return Some(unsafe { self.entries.offset(index as isize) });
        }
        None
    }

    pub fn get_by_type_id(&mut self, atype: usize) -> Option<*mut Entry> {
        for av in 0..self.counter() {
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
        let counter = self.counter();

        (0..counter).for_each(|av| {
            let auxv_entry = unsafe { self.entries.offset(av as isize) };

            let auxv_type = unsafe { Type::from((*auxv_entry).atype) as u64 };
            info!("Auxv: {} = ", auxv_type);
            if !unsafe { ((*auxv_entry).value as *const u8).is_null() } {
                // info!(auxv_entry.value as *const u8);
            } else {
                info!("NULL");
            }
            info!("'\n");
        });

        info!("auxv at: {} \n", self.pointer.0 as u64);
        let mut av = 0;
        unsafe {
            while !self.entries.offset(av).is_null() && (*self.entries.offset(av)).atype != 0 {
                let auxv_entry = *self.entries.offset(av);

                let a = Type::from(auxv_entry.atype).as_str();
                let b = auxv_entry.atype as u64;
                info!("\tAuxv: {} ({}) = ", a, b);

                if !(auxv_entry.value as *const u8).is_null() {
                    let s = crate::memory::misc::as_str(auxv_entry.value as *const u8);
                    match auxv_entry.atype {
                        31 => info!("{}", s),
                        _ => info!("{}", auxv_entry.value as u64),
                    }
                } else {
                    info!("NULL");
                }
                info!("'\n");
                av += 1;
            }
        }
        info!("\n=======\nAuxvCount={};\n=======\n", av as u64);
    }
}
