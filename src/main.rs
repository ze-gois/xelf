#![no_std]
#![no_main]

use xelf;

use result::ErrorTrait;

#[unsafe(no_mangle)]
pub extern "C" fn entry(_stack_pointer: *mut u64) -> ! {
    xelf::info!("eXecuting Executable and Linkable Format");
    let a: &str = "a";
    match syscall::write(1, a.as_bytes().as_ptr(), a.as_bytes().len()) {
        Ok(e) => Some(e),
        Err(e) => e.advert(),
    };

    syscall::write::Error::NoSpaceLeft.advert();
    panic!();
}
