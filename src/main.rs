#![no_std]
#![no_main]

use xelf;

#[unsafe(no_mangle)]
pub extern "C" fn entry(stack_pointer: *mut u64, _stack_base: *mut u64) -> ! {
    let _ = arch::memory::Stack::from_pointer(arch::Pointer(stack_pointer));

    // let Some(argv) = stack.arguments else arch::memory::stack::arguments::Vector::default();

    // info!("\n{:?}\n", stack);
    // let argv = stack.arguments.unwrap();

    // info!("\nArguments {{\n");
    // let _ = (0..argv.counter)
    //     .for_each(|o| info!("\t{:?}\n", unsafe { *argv.entries.offset(o as isize) }));
    // info!("}}\n");

    syscall::exit(0);
}
