#![no_std]
#![no_main]

use xelf::*;

#[unsafe(no_mangle)]
pub extern "C" fn entry(_stack_pointer: *mut u64) -> Result<isize> {
    human::wrapped_print("This is the computer-human interface\n")?;
    human::print("in desguise ;-)")?;

    syscall::exit(33)
}
