pub fn setup_initial_stack(elf_header: &crate::ELFHeader) -> *mut u8 {
    let stack = unsafe { prepare_stack() };

    let stack_top = (stack as *mut u8).wrapping_add(super::PAGE_SIZE as usize);

    let mut initial_stack = InitialStack::new(
        "/proc/self/exe",
        &[],
        &["PATH=/usr/local/bin:/usr/bin:/bin"],
    );

    initial_stack.add_auxv(AuxiliarType::Phdr.to(), elf_header.phoff);
    initial_stack.add_auxv(AuxiliarType::Phent.to(), elf_header.phentsize as u64);
    initial_stack.add_auxv(AuxiliarType::Phnum.to(), elf_header.phnum as u64);
    initial_stack.add_auxv(AuxiliarType::Pagesz.to(), 4096);
    initial_stack.add_auxv(AuxiliarType::Entry.to(), elf_header.entry);

    initial_stack.finalize();

    unsafe { initial_stack.write_to(stack_top.wrapping_sub(initial_stack.size() as usize)) }
}
