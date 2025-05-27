#![no_std]
#![no_main]

use xelf;

#[unsafe(no_mangle)]
pub extern "C" fn entry(stack_pointer: *mut u64) -> ! {
    xelf::info!("eXecuting Executable and Linkable Format\n");

    // Create a Stack instance from the provided pointer
    let mut stack = unsafe { arch::memory::Stack::from_pointer(stack_pointer) };
    xelf::info!("Stack from pointer:\n");
    stack.print();

    // Access specific stack elements
    unsafe {
        if let Some(arg0) = stack.get_arg(0) {
            xelf::info!("Program name: {}\n", arg0);
        }

        // Check for specific arguments
        for i in 0..stack.argc {
            if let Some(arg) = stack.get_arg(i) {
                if arg == "/" {
                    xelf::info!("Found '/' at argument position {}\n", i);
                }
            }
        }

        // Get environment variables
        if let Some(path) = stack.get_env_by_name("PATH") {
            xelf::info!("PATH environment variable: {}\n", path);
        }

        // Get and modify auxiliary vector entries
        if let Some(entry_point) = stack.get_auxv_by_type(9) {
            // AT_ENTRY
            xelf::info!("Original entry point: {:#x}\n", entry_point);

            // Modify an auxiliary vector entry (example only, not actually changing it)
            if stack.set_auxv_by_type(9, 0x11502) {
                xelf::info!("Entry point updated (demonstrative only)\n");
            }
        }
    }

    xelf::info!("\nDemonstration complete\n");

    stack.print();
    panic!("Stack demonstration completed successfully!");
}
