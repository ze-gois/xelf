// #![no_std]
#![no_main]

use lib;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        core::arch::asm!(
            "mov rsp, rdi",
            "mov rdx, rsi",
            "call {entry}",
            "hlt",
            entry = sym _entry,
            options(noreturn)
        );
    }
}

#[no_mangle]
pub extern "C" fn _entry(stack: *const u64, rdx: u64) -> ! {
    std::println!("ok");
    loop {}
}

/*
// #![no_std]
// use lib::arch::memory::*;

fn main() {
    // println!("PAGE_SIZE = 0x{:x};", PAGE_SIZE);
    // println!("ALIGNMENT = 0x{:x};", ALIGNMENT);
    // println!("!ALIGNMENT = 0x{:x};", !ALIGNMENT);
    // println!("DYNAMIC_OFFSET = 0x{:x};", DYNAMIC_OFFSET);
    // println!("PAGE_MASK = 0x{:x};", PAGE_MASK);
    // return;
    let args: Vec<String> = std::env::args().collect();
    // let argc = args.len();
    let argv: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    print!("Dbg");
    // let fp = "/home/zegois/lang/rust/project/rustics/dev/elf_loader/data/main_dyn";
    for arg in argv.iter().skip(1) {
        println!("{}", arg);
        lib::ELF::execute(arg);
    }
}
*/
