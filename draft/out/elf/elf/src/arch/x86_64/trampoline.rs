#[naked]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trampoline(entry: usize, sp: usize, _fini: usize) -> ! {
    unsafe {
        core::arch::naked_asm!("mov rsi, rsp", "jmp [rdi]", "hlt");
    }
}
