// #[link_section = ".text._start"]
// #[no_mangle]
// pub unsafe extern "C" fn main(stack_pointer: *mut u64) -> ! {
//     x86_64::print_str("Hello, World!\n");
//     let stack = x86_64::memory::stack::Layout::from_stack_pointer(stack_pointer);
//     stack.print();

//     let filepath = core::ptr::read(stack.argv.offset(1));

//     let mut elf = match ELF::read_filepath(filepath) {
//         Ok(elf) => {
//             // elf.header.print();
//             x86_64::print_str("\n---------------\n\n");
//             elf
//         }
//         Err(e) => {
//             x86_64::print_str("Error reading ELF file: ");
//             x86_64::print_str(e.to_str());
//             x86_64::print_str("\n");
//             loop {}
//         }
//     };

//     let (elf_address, interpreter_address) = elf.load();

//     elf.close();

//     let mut auxv = x86_64::memory::stack::AuxVec::new(stack.auxv);

//     auxv.set_by_type(
//         x86_64::memory::stack::auxv::Type::PHdr,
//         elf_address + elf.header.phoff.0 as usize,
//     );
//     auxv.set_by_type(
//         x86_64::memory::stack::auxv::Type::PHNum,
//         elf.header.phnum.0 as usize,
//     );
//     auxv.set_by_type(
//         x86_64::memory::stack::auxv::Type::PHEnt,
//         elf.header.phentsize.0 as usize,
//     );
//     auxv.set_by_type(x86_64::memory::stack::auxv::Type::Entry, elf_address);
//     auxv.set_by_type(
//         x86_64::memory::stack::auxv::Type::ExecFn,
//         stack.argv.offset(1) as usize,
//     );

//     if let Some(interpreter_address) = interpreter_address {
//         auxv.set_by_type(x86_64::memory::stack::auxv::Type::Base, interpreter_address);
//     }

//     let stack_end = auxv.entries.offset((auxv.count() + 1) as isize);

//     let new_stack_length = stack_end as usize - stack.argv.offset(1) as usize;

//     x86_64::memory::copy(
//         stack.argv as *mut u8,
//         stack.argv.offset(1) as *mut u8,
//         new_stack_length,
//     );
//     *(stack.argc) = *(stack.argc) - 1;

//     x86_64::print_str("Stack #1: \n");
//     stack.print();
//     let stack2 = x86_64::memory::stack::Layout::from_stack_pointer(stack_pointer);
//     x86_64::print_str("Stack #2: \n");
//     stack2.print();

//     let jumpping_address = if let Some(interpreter_address) = interpreter_address {
//         interpreter_address
//     } else {
//         elf_address
//     };

//     x86_64::print_str("Jumping to: ");
//     x86_64::print_hex(jumpping_address as u64);
//     x86_64::print_str("\n");
//     x86_64::print_str("ELF entry point: ");
//     x86_64::print_hex(elf.header.entry.0);
//     x86_64::print_str("\nELF address: ");
//     x86_64::print_hex(elf_address as u64);
//     x86_64::print_str("\nCalculated jump address: ");
//     x86_64::print_hex(jumpping_address as u64);
//     // core::arch::asm!(
//     //     "mov rdi, {stack}",
//     //     "jmp {address}",
//     //     "hlt",
//     //     stack = in(reg) stack_pointer,
//     //     address = in(reg) jumpping_address,
//     //     options(noreturn) // This informs the compiler the function does not return.
//     // );

//     loop {}
//     // let stack = x86_64::memory::stack::Layout::from_stack_pointer(stack_pointer);
//     // stack.print();

//     // x86_64::syscall::execve(stack.argv.offset(1), stack.argv, stack.envp, auxv.entries);
// }

// #[no_mangle]
// pub unsafe extern "C" fn main(stack_pointer: *mut u64) -> ! {
//     let stack = x86_64::stack::Layout::from_stack_pointer(stack_pointer);
//     stack.print();

//     let filepath = core::ptr::read(stack.argv.offset(1));

//     let mut elf = match ELF::read_filepath(filepath) {
//         Ok(elf) => {
//             // elf.header.print();
//             x86_64::print_str("\n---------------\n\n");
//             elf
//         }
//         Err(e) => {
//             x86_64::print_str("Error reading ELF file: ");
//             x86_64::print_str(e.to_str());
//             x86_64::print_str("\n");
//             loop {}
//         }
//     };

//     let (elf_address, interpreter_address) = elf.load();

//     elf.close();

//     let mut auxv = x86_64::stack::AuxVec::new(stack.auxv);

//     auxv.set_by_type(
//         x86_64::stack::auxv::Type::PHdr,
//         elf_address + elf.header.phoff.0 as usize,
//     );
//     auxv.set_by_type(
//         x86_64::stack::auxv::Type::PHNum,
//         elf.header.phnum.0 as usize,
//     );
//     auxv.set_by_type(
//         x86_64::stack::auxv::Type::PHEnt,
//         elf.header.phentsize.0 as usize,
//     );
//     auxv.set_by_type(x86_64::stack::auxv::Type::Entry, elf_address);
//     auxv.set_by_type(
//         x86_64::stack::auxv::Type::ExecFn,
//         stack.argv.offset(1) as usize,
//     );

//     if let Some(interpreter_address) = interpreter_address {
//         auxv.set_by_type(x86_64::stack::auxv::Type::Base, interpreter_address);
//     }

//     let stack_end = auxv.entries.offset((auxv.count() + 1) as isize);

//     let new_stack_length = stack_end as usize - stack.argv.offset(1) as usize;

//     x86_64::memory::copy(
//         stack.argv as *mut u8,
//         stack.argv.offset(1) as *mut u8,
//         new_stack_length,
//     );
//     *(stack.argc) = *(stack.argc) - 1;

//     x86_64::print_str("Stack #1: \n");
//     stack.print();
//     let stack2 = x86_64::stack::Layout::from_stack_pointer(stack_pointer);
//     x86_64::print_str("Stack #2: \n");
//     stack2.print();

//     let jumpping_address = if let Some(interpreter_address) = interpreter_address {
//         interpreter_address
//     } else {
//         elf_address
//     };

//     x86_64::print_str("Jumping to: ");
//     x86_64::print_hex(jumpping_address as u64);
//     x86_64::print_str("\n");

//     core::arch::asm!(
//         "mov rdi, {stack}",
//         "jmp {address}",
//         "hlt",
//         stack = in(reg) stack_pointer,
//         address = in(reg) jumpping_address,
//         options(noreturn) // This informs the compiler the function does not return.
//     );

//     // loop {}
//     // let stack = x86_64::stack::Layout::from_stack_pointer(stack_pointer);
//     // stack.print();

//     // x86_64::syscall::execve(stack.argv.offset(1), stack.argv, stack.envp, auxv.entries);
// }

// pub extern "C" fn entry(stack_pointer: *mut u64) -> ! {
//     xelf::info!("eXecuting Executable and Linkable Format\n");

//     // Create a Stack instance from the provided pointer
//     (0..10).for_each(|i| {
//         info!("{:?}\n", unsafe { stack_pointer.add(i) });
//     });
//     let mut stack = arch::memory::Stack::from_pointer(arch::Pointer(stack_pointer));
//     // xelf::info!("Stack from pointer:\n");
//     // stack.print();

//     // // Access specific stack elements
//     // unsafe {
//     //     if let Some(arg0) = stack.get_arg(0) {
//     //         xelf::info!("Program name: {}\n", arg0);
//     //     }

//     //     // Check for specific arguments
//     //     for i in 0..stack.argc {
//     //         if let Some(arg) = stack.get_arg(i) {
//     //             if arg == "/" {
//     //                 xelf::info!("Found '/' at argument position {}\n", i);
//     //             }
//     //         }
//     //     }

//     //     // Get environment variables
//     //     if let Some(path) = stack.get_env_by_name("PATH") {
//     //         xelf::info!("PATH environment variable: {}\n", path);
//     //     }

//     //     // Get and modify auxiliary vector entries
//     //     if let Some(entry_point) = stack.get_auxv_by_type(9) {
//     //         // AT_ENTRY
//     //         xelf::info!("Original entry point: {:#x}\n", entry_point);

//     //         // Modify an auxiliary vector entry (example only, not actually changing it)
//     //         if stack.set_auxv_by_type(9, 0x11502) {
//     //             xelf::info!("Entry point updated (demonstrative only)\n");
//     //         }
//     //     }
//     // }

//     // xelf::info!("\nDemonstration complete\n");

//     // stack.print();
//     panic!("Stack demonstration completed successfully!");
// }
