#![no_std]
#![no_main]

use xelf;

#[unsafe(no_mangle)]
pub extern "C" fn entry(_stack_pointer: *mut u64) -> ! {
    xelf::info!("eXecuting Executable and Linkable Format");
    let x = 32;
    xelf::wrapped_info!("\n{x} {} is", "ok");
    panic!();
}
