pub mod entry;

use crate::memory;
use core::ptr;
pub use entry::*;
use human::info;

#[derive(Debug)]
pub struct Vector<'e> {
    pub counter: u64,
    pub entries: *mut Entry<'e>,
}

impl<'e> Default for Vector<'e> {
    fn default() -> Self {
        Self {
            counter: 0,
            entries: ptr::null_mut(),
        }
    }
}

impl<'e> Vector<'e> {
    pub fn from_pointer(argv_pointer: crate::Pointer) -> Self {
        let counter = unsafe { *argv_pointer.0 };
        let entries_pointer = unsafe { argv_pointer.0.add(1) as *mut *mut u8 };
        let environment_pointer = unsafe { argv_pointer.0.add(2 + counter as usize) };

        if counter == 0 {
            return Self::default();
        }

        let entries_mmap_pointer = unsafe {
            let size = core::mem::size_of::<Entry<'e>>() * counter as usize;

            let aligned_size = (size + memory::page::SIZE - 1) & !(memory::page::SIZE - 1);

            let result = memory::mmap(
                ptr::null_mut(),
                aligned_size,
                memory::mmap::prot::READ | memory::mmap::prot::WRITE,
                memory::mmap::flag::PRIVATE | memory::mmap::flag::ANONYMOUS,
                -1,
                0,
            );

            match result {
                Ok(ptr) => ptr as *mut Entry<'e>,
                Err(_) => {
                    info!("Failed to allocate memory for arguments vector\n");
                    return Self::default();
                }
            }
        };

        // Create entries from the argument pointers
        for i in 0..counter {
            unsafe {
                let entry_pointer = entries_pointer.add(i as usize);
                let entry = Entry::from_pointer(entry_pointer, i);
                ptr::write(entries_mmap_pointer.add(i as usize), entry);
            }
        }

        let arguments = Self {
            counter,
            entries: entries_mmap_pointer,
        };

        arguments
    }

    pub fn print(&self) {
        info!("Arguments {{\n");
        for c in 0..self.counter {
            info!("\t{:?}\n", unsafe { *self.entries.add(c as usize) })
        }
        info!("}} Arguments \n");
    }

    /// Prints all arguments for debugging purposes
    pub fn print_arguments(&self) {
        info!("Argument count: {}\n", self.counter);
        for i in 0..self.counter as usize {
            if let Some(entry) = self.get(i) {
                info!("Arg {}: '{}'\n", i, entry.value);
            }
        }
    }

    /// Gets an entry at the specified index
    pub fn get(&self, index: usize) -> Option<&Entry<'e>> {
        if index >= self.counter as usize || self.entries.is_null() {
            return None;
        }
        unsafe { Some(&*self.entries.add(index)) }
    }

    /// Gets a mutable entry at the specified index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Entry<'e>> {
        if index >= self.counter as usize || self.entries.is_null() {
            return None;
        }
        unsafe { Some(&mut *self.entries.add(index)) }
    }

    /// Returns the number of arguments
    pub fn len(&self) -> usize {
        self.counter as usize
    }

    /// Returns true if there are no arguments
    pub fn is_empty(&self) -> bool {
        self.counter == 0
    }

    /// Returns an iterator over the entries
    pub fn iter(&self) -> Iter<'_, 'e> {
        Iter {
            vector: self,
            index: 0,
        }
    }
}

/// Iterator for Vector entries
pub struct Iter<'a, 'e> {
    vector: &'a Vector<'e>,
    index: usize,
}

impl<'a, 'e> Iterator for Iter<'a, 'e> {
    type Item = &'a Entry<'e>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.vector.counter as usize {
            return None;
        }
        let item = self.vector.get(self.index);
        self.index += 1;
        item
    }
}

impl<'e> Drop for Vector<'e> {
    fn drop(&mut self) {
        if !self.entries.is_null() && self.counter > 0 {
            unsafe {
                let size = core::mem::size_of::<Entry<'e>>() * self.counter as usize;

                // Align size to page boundary
                let aligned_size = (size + memory::page::SIZE - 1) & !(memory::page::SIZE - 1);
                // let aligned_size = memory::page::align_to_lower_page(size);

                let _ = memory::munmap(self.entries as *mut u8, aligned_size);
            }
        }
    }
}
