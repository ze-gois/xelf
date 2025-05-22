use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { x86_64::print_str("\n-----> Panicking.") };
    loop {}
}
