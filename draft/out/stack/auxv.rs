pub mod atype;

pub use atype::Type;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AuxEntry {
    pub atype: usize,
    pub value: usize,
}

pub struct AuxVec {
    pub entries: *mut AuxEntry,
    pub count: Option<usize>,
}

impl AuxVec {
    pub unsafe fn new(entries: *mut AuxEntry) -> Self {
        Self {
            entries,
            count: None,
        }
    }

    pub unsafe fn count(&mut self) -> usize {
        if let Some(count) = self.count {
            return count;
        }

        let mut count = 0;
        let mut auxv = self.entries;
        loop {
            let auxv_entry = *auxv.offset(count as isize);

            if auxv_entry.atype == 0 {
                break;
            }

            count += 1;
        }

        self.count = Some(count);
        count
    }

    pub unsafe fn get_by_index(&mut self, index: usize) -> Option<*mut AuxEntry> {
        if index < self.count() {
            return Some(self.entries.offset(index as isize));
        }
        None
    }

    pub unsafe fn get_by_type_id(&mut self, atype: usize) -> Option<*mut AuxEntry> {
        for av in 0..self.count() {
            let entry = self.entries.offset(av as isize);

            if (*entry).atype == atype {
                return Some(entry);
            }
        }
        None
    }

    pub unsafe fn get_by_type(&mut self, atype: Type) -> Option<*mut AuxEntry> {
        self.get_by_type_id(atype.to())
    }

    pub unsafe fn set_by_type_id(&mut self, atype: usize, value: usize) {
        if let Some(entry) = self.get_by_type_id(atype) {
            (*entry).value = value;
        }
    }

    pub unsafe fn set_by_type(&mut self, atype: Type, value: usize) {
        self.set_by_type_id(atype.to(), value);
    }

    pub unsafe fn print(&mut self) {
        let count = self.count();

        (0..count).for_each(|av| {
            let auxv_entry = self.entries.offset(av as isize);

            crate::misc::print_str("Auxv: ");
            crate::misc::print_dec(Type::from((*auxv_entry).atype) as u64);
            crate::misc::print_str(" = '");
            if !((*auxv_entry).value as *const u8).is_null() {
                // crate::misc::print(auxv_entry.value as *const u8);
            } else {
                crate::misc::print_str("NULL");
            }
            crate::misc::print_str("'\n");
        })
    }
}
