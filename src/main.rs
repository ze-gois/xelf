#![no_std]
#![no_main]

use xelf;

use human::info;

#[unsafe(no_mangle)]
pub extern "C" fn entry(stack_pointer: *mut u64) -> ! {
    xelf::info!("eXecuting Executable and Linkable Format\n");

    // Create a Stack instance from the provided pointer
    (0..10).for_each(|i| {
        info!("{:?}\n", unsafe { stack_pointer.add(i) });
    });

    let stack = arch::memory::Stack::from_pointer(arch::Pointer(stack_pointer));

    stack.print();
    panic!("Stack demonstration completed successfully!");
}
