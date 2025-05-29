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
    pub fn from_pointer(argv_pointer: crate::Pointer) -> (Option<Self>, crate::Pointer) {
        let argv_pointer: *mut u64 = argv_pointer.0;
        let counter;
        let argv_p;
        unsafe {
            counter = *argv_pointer;
            argv_p = argv_pointer.add(1);
        }
        let argv_p = argv_p as *mut *mut u8;

        // info!("Argument count: {:?}\n\n", counter);
        if counter == 0 {
            return (None, crate::Pointer(argv_pointer));
        }

        // Allocate memory using mmap
        let entries_ptr = unsafe {
            let size = core::mem::size_of::<Entry<'e>>() * counter as usize;

            // Align size to page boundary
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
                    // Allocation failed, return default vector
                    info!("Failed to allocate memory for arguments vector\n");
                    return (Some(Self::default()), crate::Pointer(argv_pointer));
                }
            }
        };

        // Create entries from the argument pointers
        for i in 0..counter {
            unsafe {
                let arg_ptr = argv_p.add(i as usize);
                let entry = Entry::from_pointer(arg_ptr, i);
                ptr::write(entries_ptr.add(i as usize), entry);
                // info!("Arg {}: {:?}\n", i, (*entries_ptr.add(i as usize)).value);
            }
        }

        let vector = Self {
            counter,
            entries: entries_ptr,
        };

        (Some(vector), crate::Pointer(argv_pointer))
    }

    pub fn print(&self) {
        info!("{:?}\n", self);
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
