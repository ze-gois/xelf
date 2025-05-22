#[naked]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn startaa() -> ! {
    unsafe {
        core::arch::naked_asm!("mov rsp, rdi", "mov rdx, rsi", "call entry", "hlt");
    }
}
