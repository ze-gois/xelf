use crate::info;
use core::default::Default;
use core::ptr;

pub fn alloc<T: Default + Sized>(counter: usize) -> Option<*mut T> {
    let pointer = unsafe {
        let size = core::mem::size_of::<T>() * counter;

        let aligned_size = (size + arch::memory::page::SIZE - 1) & !(arch::memory::page::SIZE - 1);

        let result = syscall::mmap(
            ptr::null_mut(),
            aligned_size,
            syscall::mmap::prot::READ | syscall::mmap::prot::WRITE,
            syscall::mmap::flag::PRIVATE | syscall::mmap::flag::ANONYMOUS,
            -1,
            0,
        );

        match result {
            Ok(ptr) => ptr as *mut T,
            Err(_) => {
                info!("Failed to allocate arch::memory\n");
                return None;
            }
        }
    };

    Some(pointer)
}
