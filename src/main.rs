#![no_std]
#![no_main]

use xelf;

#[unsafe(no_mangle)]
pub extern "C" fn entry(stack_pointer: *mut u64) -> ! {
    xelf::info!("eXecuting Executable and Linkable Format\n");
    let stack = arch::memory::stack::Layout::from_pointer(stack_pointer);

    stack.print_args();

    panic!();
}
