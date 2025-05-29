#![no_std]
#![no_main]

use xelf::{self, info};

#[unsafe(no_mangle)]
pub extern "C" fn entry(stack_pointer: *mut u64, stack_base: *mut u64) -> ! {
    info!(
        "{} = {:?} < {:?}",
        stack_pointer < stack_base,
        stack_pointer,
        stack_base
    );

    let stack = arch::memory::Stack::from_pointer(arch::Pointer(stack_pointer));

    if let Some(stack) = stack.arguments {
        info!("Arguments: {:?}", stack);
    }

    syscall::exit(0);
}
