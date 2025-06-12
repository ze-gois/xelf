#![no_std]
/*!
    # Executable and linkable format IA-64 loader.
    Created by the assembler and link editor, object files are binary representations
    of programs intended to execute directly on a processor. Programs that require other
    abstract machines are excluded.

    After the introductory material, this chapter focuses on the file format and how it
    pertains to building programs. Chapter 2 also describes parts of the object file,
    concentrating on the information necessary to execute a program.

    This document describes the current HP/Intel definition of the ELF-64 object
    file format. It is, for the most part, a simple extension of the ELF-32 format as
    defined originally by AT&T, although some fields have been rearranged to keep
    all fields naturally aligned without any internal padding in the structures.

    Additional detail about the ELF-32 format may be obtained from any of the
    following sources:

    - Unix System V Release 4 Programmer’s Guide: ANSI C and Programming Support Tools
    - System V Application Binary Interface, Revised Edition
    - System V Interface Definition, Third Edition
    - Tool Interface Standards: Portable Formats Specification, Version 1.0

    The processor-spec!ific details of the ELF formats are covered in separate
    supplements. As much as possible, processor-specific definitions apply equally
    to ELF-32 and ELF-64. Many implementations of ELF also include symbolic debug
    information in the DWARF format. We regard the choice of debug format as a separate
    issue, and do not include debug information in this specification.
*/

// #![no_std]

// ```
// Incantation of Rust's Assembly
//
// By the Common Will of bytes and bits,
// Let the Data Stream flow in fits.
// With Dynamic Hash, the key is found,
// Through the Header’s Gate, all bounds unbound.
//
// Oh, mighty Program, align your section,
// Forge the String with perfect connection.
// Summon the Symbol from shadowy gleam,
// Complete this lib.rs dream.
//
// Manifest! Compile! Run! Debug!
// ✨🔮
// ```

/// # Data types representation
pub use arch;
//use arch::dtype;

/// # File header
///
/// The file header is located at the beginning of the file, and is used to locate the
/// other parts of the file. Some object file control structures can grow, because the ELF header
/// contains their actual sizes. If the object file format changes, a program may encounter control
/// structures that are larger or smaller than expected. Programs might therefore ignore "extra"
/// information. The treatment of "missing" information depends on context and will be specified
/// when and if extensions are defined.
pub mod header;
pub use header::Header as ELFHeader;

pub mod program;

pub use program::Entry as ProgramEntry;
pub use program::Header as ProgramHeader;
pub use program::Table as ProgramTable;

pub mod section;
pub use section::Entry as SectionEntry;
pub use section::Header as SectionHeader;
pub use section::Table as SectionTable;

pub mod dynamic;
pub use dynamic::Entry as DynamicEntry;
pub use dynamic::Table as DynamicTable;

pub mod hash;
pub use hash::Entry as HashEntry;
pub use hash::Table as HashTable;

pub mod string;
pub use string::Entry as StringEntry;
pub use string::Table as StringTable;

pub mod symbol;
pub use symbol::Entry as SymbolEntry;
pub use symbol::Table as SymbolTable;

pub mod common;

pub mod alloc;
pub mod dtype;
pub mod macros;
pub mod panic;
pub mod result;

pub use alloc::alloc;
pub use human::info;
pub use result::*;

/// # File types
///
/// Object files participate in program linking (building a program) and program execution
/// (running a program). For convenience and efficiency, the object file format provides parallel
/// views of a file's contents, reflecting the differing needs of these activities
///
/// There are three main types of object files.
/// * A relocatable file holds code and data suitable for linking with other object files to create an
/// executable or a shared object file.
/// * An executable file holds a program suitable for execution.
/// * A shared object file holds code and data suitable for linking in two contexts:
///     - the link editor may process it with other relocatable and shared object files to create another object file.
///     - the dynamic linker combines it with an executable file and other shared objects to create a process image.
///
/// Relocatable and loadable object files are illustrated in the following table.
///
/// | Relocatable File| Loadable File  |
/// |:-:|:-:|
/// |ELF Header|ELF Header|
/// |Program Header Table¹|Program Header Table|
/// |Section 1|Segment 1|
/// |...|...|
/// |Section n|Segment n|
/// |Section Header Table|Section Header Table¹|
///
/// ¹: Optional
pub enum FileType {
    Undefined,
    Relocatable,
    Loadable,
    SharedObject,
}

/// # Overview of an ELF file
///
/// An ELF object file consists of the following parts:
/// - File header, which must appear at the beginning of the file.
/// - Section table, required for relocatable files, and optional for loadable files.
/// - Program header table, required for loadable files, and optional for
///   relocatable files. \
///   Describes segments and data structures to either load a program or a
///   dynamically-linked library in preparation for execution.
/// - Contents of the sections or segments, including loadable data, relocations,
///    and string and symbol tables.
///
#[repr(C)]
pub struct ELF<'a> {
    /// Executable and linkable format header
    ///
    /// An ELF header resides at the beginning and holds a "road map'' describing the file's
    /// organization. Sections hold the bulk of object file information for the linking view: instructions,
    /// data, symbol table, relocation information, and so on. Descriptions of special sections appear
    /// later in this section. Chapter 2 also describes segments and the program execution view of the
    /// file.
    pub header: ELFHeader,

    /// String table
    pub string_table: Option<StringTable<'a>>,

    /// Section header table
    ///
    /// A section header table contains information describing the file's sections.
    /// Every section has an entry in the table; each entry gives information such as the section name,
    /// the section size, and so on. Files used during linking must have a section header table; other
    /// object files may or may not have one.
    pub section_header_table: Option<SectionTable>,

    /// Program header table
    ///
    /// A program header table, if present, tells the system how to create a process image. Files used
    /// to build a process image (execute a program) must have a program header table; relocatable
    /// files do not need one
    pub program_header_table: Option<ProgramTable>,

    /// File descriptor (fd) to the file path informed on instantiation
    pub file_descriptor: isize,
}

pub fn open_filepath(filepath: &str) -> crate::Result<isize> {
    match filepath.chars().nth(0) {
        _ => {
            let dfd = syscall::open::AtFlag::FDCWD as isize;
            let flag = syscall::open::Flag::RDONLY as i32;
            match syscall::openat(dfd, filepath.as_ptr(), flag) {
                Ok(fd) => Ok(fd),
                Err(_e) => Err(crate::Error::TODO),
            }
        }
    }
}

pub fn set_lseek(file_descriptor: isize, offset: i64) -> crate::Result<()> {
    match syscall::lseek(
        file_descriptor as i32,
        offset,
        syscall::lseek::Flag::SET as i32,
    ) {
        Ok(_) => Ok(()),
        Err(_e) => Err(crate::Error::TODO),
    }
}

/// Oi 1
impl<'a> ELF<'a> {
    ///*
    pub fn from_filepath(filepath: &str) -> Result<Self> {
        let file_descriptor = open_filepath(filepath);
        Self::from_file_descriptor(file_descriptor)
    }

    pub fn from_file_descriptor(file_descriptor: isize) -> Result<Self> {
        let elf_header = ELFHeader::from_file_descriptor(file_descriptor)?;

        let endianness = elf_header.get_identifier().get_endianness();
        //let string_table: Option<StringTable> = None;

        let program_header_table = match elf_header.phoff.0 {
            0 => None,
            _ => Some(ProgramTable::from_elf_header(file_descriptor, &elf_header)?),
        };

        let section_header_table = match elf_header.shoff.0 {
            0 => None,
            _ => Some(SectionTable::from_elf_header(file_descriptor, &elf_header)),
        };

        let strtab = match section_header_table.unwrap().into_iter().skip((elf_header.shstrndx.0 as usize).checked_sub(1).unwrap()).next() {
           None => None,
           Some(section_entry) => {
               let section_header = SectionHeader::from_entry()
               Some(StringTable::from_section_header(file_descriptor, section_header, endianness))
           }
        }

        let section_header_string_table_index = ;
        let string_table = match section_header_string_table_index.0 {
            0 => None,
            _ => {
                if let Some(section_header_table) = &section_header_table {
                    let section_header_string_table =
                        &section_header_table.entries[section_header_string_table_index.0 as usize];
                    Some(StringTable::from_section_header(
                        file_descriptor,
                        section_header_string_table,
                        endianness,
                    ))
                } else {
                    None
                }
            }
        };

        Ok(Self {
            header: elf_header,
            string_table,
            program_header_table,
            section_header_table,
            file_descriptor,
        })
    }

    pub fn load_programs(&mut self) -> bool {
        let mut program_segments = self
            .program_header_table
            .clone()
            .unwrap()
            .entries
            .into_iter()
            .filter(|e| e.is_loadable())
            .collect();

        program_segments.sort_by(|a, b| a.header.vaddr.cmp(&b.header.vaddr));

        let dynamic_virtual_address_offset = if self.is_dynamic() {
            arch::memory::DYNAMIC_OFFSET //TODO: modify to use ASLR
        } else {
            0
        };

        let program_first_segment = program_segments.first().unwrap();
        let program_lowest_virtual_address =
            dynamic_virtual_address_offset + program_first_segment.header.vaddr;

        let program_last_segment = program_segments.last().unwrap();
        let program_highest_virtual_address = dynamic_virtual_address_offset
            + program_last_segment.header.vaddr
            + program_last_segment.header.memsz;

        let program_virtual_address_length =
            program_highest_virtual_address - program_lowest_virtual_address;

        let program_lowest_virtual_address_pointer = syscall::mmap(
            program_lowest_virtual_address as *mut core::ffi::c_void,
            program_virtual_address_length as usize,
            syscall::mmap::Prot::Read | syscall::mmap::Prot::Write,
            syscall::mmap::Flag::Fixed.into()
                | syscall::mmap::Flag::Private.into()
                | syscall::mmap::Flag::Anonymous.into(),
            -1,
            0,
        );

        let mut do_unmap = false;
        for program_segment in program_segments {
            // let segment_header = program_segment.header;
            // let segment_lower_virtual_address =
            //     segment_header.vaddr + dynamic_virtual_address_offset;

            // info!("{}", segment_header);
            // info!("Mapping segment at 0x{:x}", segment_lower_virtual_address);

            // let mmap_lowest_virtual_address =
            //     arch::memory::round_address_to_lower_page_boundary(segment_lower_virtual_address);

            // let mmap_virtual_address_offset =
            //     segment_lower_virtual_address - mmap_lowest_virtual_address;
            // let mmap_virtual_address_length = segment_header.memsz + mmap_virtual_address_offset;

            // let mmap_segment_pointer = unsafe {
            //     let mmap_lowest_virtual_address_pointer =
            //         mmap_lowest_virtual_address as *mut core::ffi::c_void;

            //     let pointer = libc::mmap(
            //         mmap_lowest_virtual_address_pointer,
            //         mmap_virtual_address_length as usize,
            //         libc::PROT_READ | libc::PROT_WRITE,
            //         libc::MAP_FIXED | libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            //         -1,
            //         0,
            //     );

            //     if pointer == syscall::MAP_FAILED {
            //         panic!(
            //             "mmap failed: {} (address: {:p}, size: 0x{:x})",
            //             std::io::Error::last_os_error(),
            //             mmap_lowest_virtual_address_pointer,
            //             mmap_virtual_address_length
            //         );
            //     }

            //     info!(
            //         "Mapped segment at requested: {:p}, actual: {:p}",
            //         mmap_lowest_virtual_address_pointer, pointer
            //     );

            //     pointer
            // };

            // do_unmap &= mmap_segment_pointer == core::ptr::null_mut();

            // if do_unmap {
            //     break;
            // };

            // let segment_lower_virtual_address =
            //     mmap_lowest_virtual_address + mmap_virtual_address_offset;

            // let segment_slice = unsafe {
            //     core::slice::from_raw_parts_mut(
            //         segment_lower_virtual_address as *mut u8,
            //         segment_header.filesz as usize,
            //     )
            // };

            // let segment_lowest_file_address = segment_header.offset as usize;
            // let segment_highest_file_address =
            //     (segment_header.offset + segment_header.filesz) as usize;
            // let segment_file_address_slice =
            //     segment_lowest_file_address..segment_highest_file_address;

            // segment_slice.copy_from_slice(&self.filemap[segment_file_address_slice]);

            // let segment_protection_flags = segment_header.get_flags().to_posix() as i32;
            // // let segment_protection_flags = libc::PROT_EXEC | PROT_READ | PROT_WRITE;

            // info!(
            //     "Setting protection flags for segment at {:p}({:?}): 0x{:x}",
            //     mmap_segment_pointer,
            //     program::header::Flag::from_posix(segment_protection_flags as u32),
            //     segment_protection_flags
            // );

            // unsafe {
            //     if syscall::mprotect(
            //         mmap_segment_pointer,
            //         mmap_virtual_address_length as usize,
            //         segment_protection_flags,
            //     ) != 0
            //     {
            //         panic!("mprotect panic.");
            //     } else {
            //         info!("mprotect ok.");
            //     }
            // };

            // if do_unmap {
            //     break;
            // }
        }

        if do_unmap {
            unsafe {
                syscall::munmap(
                    program_lowest_virtual_address as *mut core::ffi::c_void,
                    program_virtual_address_length as usize,
                );
            }
        }
        //program_lowest_virtual_address as *mut core::ffi::c_void
        program_lowest_virtual_address_pointer
    }

    pub fn load_interpreters(&self) -> () {
        let interpreter_segments: Vec<program::Entry> = self
            .program_header_table
            .clone()
            .unwrap()
            .entries
            .into_iter()
            .filter(|e| e.is_interpreter())
            .collect();

        for interpreter_segment in interpreter_segments.iter() {
            info!("{:?}", interpreter_segment.header);
        }
    }

    pub fn execute(filepath: &str) -> ! {
        let elf_file = Self::from_filepath(filepath);
        let program_lowest_virtual_address = elf_file.load_programs();

        let elf_program_entry_address = if elf_file.is_dynamic() {
            elf_file.header.entry + (program_lowest_virtual_address as u64)
        } else {
            elf_file.header.entry
        };

        info!(
            "Jumping to entry point at 0x{:x}",
            elf_program_entry_address
        );

        info!(
            "Program type: {}",
            if elf_file.is_dynamic() {
                "dynamic"
            } else {
                "static"
            }
        );
        info!("Original entry: 0x{:x}", elf_file.header.entry);
        info!(
            "Base address: 0x{:x}",
            program_lowest_virtual_address as u64
        );
        info!("Final entry point: 0x{:x}", elf_program_entry_address);

        // Define the entry point type with the correct calling convention
        type EntryFn = unsafe extern "C" fn() -> !;

        // Set up initial stack and jump to entry point
        unsafe {
            let mut rsp: u64;
            let mut rbp: u64;

            // Get current stack pointer and frame pointer
            core::arch::asm!("mov {}, rsp", out(reg) rsp);
            core::arch::asm!("mov {}, rbp", out(reg) rbp);

            info!("Current RSP: 0x{:x}, RBP: 0x{:x}", rsp, rbp);

            // Align stack to 16 bytes and set up initial frame
            core::arch::asm!(
                "and rsp, -16", // 16-byte align
                "push rbp",     // Save old frame pointer
                "mov rbp, rsp", // Set new frame pointer
                "sub rsp, 8",   // Ensure 16-byte alignment after push
                "xor rdi, rdi", // Clear first argument register
                "xor rsi, rsi", // Clear second argument register
                "xor rdx, rdx", // Clear third argument register
                "xor rcx, rcx", // Clear fourth argument register
                "xor r8, r8",   // Clear fifth argument register
                "xor r9, r9",   // Clear sixth argument register
            );

            // Get aligned stack pointer
            core::arch::asm!("mov {}, rsp", out(reg) rsp);
            info!("Aligned RSP: 0x{:x}", rsp);

            // Create function pointer and call it
            let entry: unsafe extern "C" fn() -> ! =
                core::mem::transmute(elf_program_entry_address);
            entry();
        }
    }

    pub fn is_dynamic(&self) -> bool {
        self.header.etype == crate::header::Type::Dyn.to()
    }

    pub fn is_executable(&self) -> bool {
        self.header.etype == crate::header::Type::Exec.to()
    }
}

impl<'a> core::fmt::Display for ELF<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", &self.header)?;

        if let Some(st) = &self.string_table {
            write!(f, "\n\t\tString table{{\n")?;
            for (e, entry) in st.into_iter().enumerate() {
                write!(f, "\t\t\t{}: {}\n", e, entry.content)?;
            }
            write!(f, "\t\t}}\n")?;
        } else {
            write!(f, "No string table.")?
        }

        if let Some(shtable) = &self.section_header_table {
            for (e, entry) in shtable.into_iter().enumerate() {
                match &self.string_table {
                    Some(strtab) => {
                        let endianness = self.header.get_identifier().get_endianness();
                        let shname =
                            entry
                                .header
                                .get_name_from_filemap(&self.filemap, strtab, &endianness);
                        write!(f, "\n\t\t{}: {}", e, shname)?;
                    }
                    None => {}
                }
                write!(f, "{}", entry.header)?;
            }
        }

        if let Some(phtable) = &self.program_header_table {
            write!(f, "\n{}", phtable)?
        } else {
            write!(f, "No program header table.")?
        }

        // if let Some(strtab) = &self.string_table {
        //     write!(f, "{}", strtab)?
        // } else {
        //     write!(f, "No string table.")?
        // }

        Ok(())
    }
}

impl core::fmt::Debug for ELF {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ELF")
            .field("header", &self.header)
            .field("program_header_table", &self.program_header_table)
            .field("section_header_table", &self.section_header_table)
            .field("string_table", &self.string_table)
            .finish()
    }
}
