use crate::info;
use core::ptr;

pub fn alloc<T: Sized>(counter: usize) -> Option<*mut T> {
    let pointer = {
        let size = core::mem::size_of::<T>() * counter;

        let aligned_size = (size + arch::memory::page::SIZE - 1) & !(arch::memory::page::SIZE - 1);

        let result = syscall::mmap(
            ptr::null_mut(),
            aligned_size,
            (syscall::mmap::Prot::Read as i32) | (syscall::mmap::Prot::Write as i32),
            (syscall::mmap::Flag::Private as i32) | (syscall::mmap::Flag::Anonymous as i32),
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
