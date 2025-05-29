pub mod entry;

pub use crate::memory;
pub use entry::*;
use human::info;
use core::ptr;

#[derive(Debug)]
pub struct Vector<'e> {
    pub counter: usize,
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
    pub fn from_pointer(env_pointer: crate::Pointer) -> (Option<Self>, crate::Pointer) {
        let env_pointer: *mut *mut u8 = env_pointer.0 as *mut *mut u8;
        
        // First, count the environment variables (null-terminated array)
        let mut counter = 0;
        unsafe {
            while !(*env_pointer.add(counter)).is_null() {
                counter += 1;
            }
        }
        
        info!("Environment count: {:?}\n\n", counter);
        
        if counter == 0 {
            return (Some(Self::default()), crate::Pointer(env_pointer as *mut u64));
        }

        // Allocate memory using mmap
        let entries_ptr = unsafe {
            let size = core::mem::size_of::<Entry<'e>>() * counter;
            
            // Align size to page boundary
            let page_size = 4096;
            let aligned_size = (size + page_size - 1) & !(page_size - 1);
            
            // Define mmap constants locally since they're not accessible
            const PROT_READ: i32 = 0x1;
            const PROT_WRITE: i32 = 0x2;
            const MAP_PRIVATE: i32 = 0x02;
            const MAP_ANONYMOUS: i32 = 0x20;
            
            let result = memory::mmap::mmap(
                ptr::null_mut(),
                aligned_size,
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANONYMOUS,
                -1,
                0,
            );
            
            match result {
                Ok(ptr) => ptr as *mut Entry<'e>,
                Err(_) => {
                    // Allocation failed, return default vector
                    info!("Failed to allocate memory for environment vector\n");
                    return (Some(Self::default()), crate::Pointer(env_pointer as *mut u64));
                }
            }
        };
        
        // Create entries from the environment pointers
        for i in 0..counter {
            unsafe {
                let env_ptr = env_pointer.add(i);
                let entry = Entry::from_pointer(env_ptr, i);
                ptr::write(entries_ptr.add(i), entry);
                info!("Env {}: {:?}\n", i, (*entries_ptr.add(i)).value);
            }
        }
        
        // Move past the environment array and the NULL terminator
        let next_pointer = unsafe { env_pointer.add(counter + 1) };

        let vector = Self {
            counter,
            entries: entries_ptr,
        };

        (Some(vector), crate::Pointer(next_pointer as *mut u64))
    }

    pub fn print(&self) {
        info!("Environment variables: {}\n", self.counter);
        for i in 0..self.counter {
            if let Some(entry) = self.get(i) {
                info!("Env {}: '{}'\n", i, entry.value);
            }
        }
    }
    
    /// Gets an entry at the specified index
    pub fn get(&self, index: usize) -> Option<&Entry<'e>> {
        if index >= self.counter || self.entries.is_null() {
            return None;
        }
        unsafe { 
            Some(&*self.entries.add(index))
        }
    }

    /// Gets a mutable entry at the specified index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Entry<'e>> {
        if index >= self.counter || self.entries.is_null() {
            return None;
        }
        unsafe { 
            Some(&mut *self.entries.add(index))
        }
    }

    /// Returns the number of environment variables
    pub fn len(&self) -> usize {
        self.counter
    }

    /// Returns true if there are no environment variables
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
        if self.index >= self.vector.counter {
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
                let size = core::mem::size_of::<Entry<'e>>() * self.counter;
                
                // Align size to page boundary
                let page_size = 4096;
                let aligned_size = (size + page_size - 1) & !(page_size - 1);
                
                // Munmap the allocated memory
                let _ = memory::mmap::munmap(self.entries as *mut u8, aligned_size);
            }
        }
    }
}
