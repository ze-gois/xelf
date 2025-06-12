#![no_std]
#![allow(unused)]
pub mod error;
pub mod page;

pub mod dtype;
pub mod header;
pub mod program;
pub mod section;

use core::ptr::read;
use core::usize;

use dtype::addr;
pub use error::Error;
pub use error::Result;

pub use header::Header as ELFHeader;

pub use program::Header as ProgramHeader;
pub use program::Table as ProgramTable;

pub use section::Header as SectionHeader;
pub use section::Table as SectionTable;

use x86_64::ToFlags;
use x86_64::syscall;

pub static mut IS_DYNAMIC: core::mem::MaybeUninit<bool> = core::mem::MaybeUninit::uninit();

pub unsafe fn set_dynamic(e: bool) {
    IS_DYNAMIC.write(e);
}

pub unsafe fn is_dynamic() -> bool {
    *IS_DYNAMIC.assume_init_ref()
}

pub struct ELF<'elf> {
    pub debug: bool,
    pub file_descriptor: Option<i32>,
    pub header: ELFHeader,
    pub program_table: Option<ProgramTable<'elf>>,
    pub section_table: Option<SectionTable<'elf>>,
    pub origin: Option<dtype::Addr>,
    // pub interpreter: Option<>
}

impl ELF<'_> {
    pub unsafe fn read_filepath(filepath: *const u8) -> Result<Self> {
        use openat::flags::{Open, OpenAt, ToFlags};
        use x86_64::syscall::openat;
        let file_descriptor = match openat::openat(OpenAt::FDCWD.to(), filepath, Open::RDONLY.to())
        {
            Ok(file_descriptor) => file_descriptor,
            Err(_) => return Err(Error::TODO),
        };

        x86_64::print_str("\nGoing ELF::read\n");
        let elf = ELF::read(file_descriptor).unwrap();
        x86_64::print_str("\nBACK ELF::load\n");

        Ok(elf)
    }

    pub unsafe fn load_filepath(filepath: *const u8) -> (usize, Option<usize>) {
        let elf = ELF::read_filepath(filepath).unwrap();

        let program_origin = elf.load_program_michail().unwrap();

        let interpreter_filepath = elf.load_interpreter_michail().unwrap();
        let interpreter_origin = if interpreter_filepath != usize::MAX {
            let elf = ELF::read_filepath(interpreter_filepath as *mut u8).unwrap();
            Some(elf.load_program_michail().unwrap())
        } else {
            None
        };

        (program_origin, interpreter_origin)
    }

    pub unsafe fn execute_filepath(filepath: *const u8) -> ! {
        let elf = ELF::load_filepath(filepath);
        // ELF::execute_address();
        loop {}
    }

    pub unsafe fn read(file_descriptor: isize) -> Result<Self> {
        x86_64::print_str("\nReading Header\n");
        let header = ELFHeader::read(file_descriptor)?;
        x86_64::print_str("\nDone Reading Header\n");
        let section_table: Option<SectionTable> = None;

        x86_64::print_str("\nDone Section\n");

        assert_eq!(ProgramHeader::size(), ProgramHeader::size2());

        let program_table = if header.phnum > 0 {
            x86_64::print_str("\nReading Programs\n");
            let program_table = ProgramTable::read(file_descriptor, header.phoff, header.phnum);
            match program_table {
                Ok(program_table) => {
                    x86_64::print_str("\nPrograms Read\n");
                    Some(program_table)
                }
                Err(e) => {
                    x86_64::print_str("\nError Reading Programs\n");
                    return Err(e);
                }
            }
        } else {
            None
        };

        // let section_table = if header.shnum > 0 {
        //     Some(SectionTable::read(file_descriptor, header.shoff, header.shnum)?)
        // } else {
        //     None
        // };

        Ok(ELF {
            debug: false,
            file_descriptor: Some(file_descriptor),
            header,
            program_table,
            section_table,
            origin: None,
        })
    }

    pub unsafe fn close(&mut self) {
        if let Some(fd) = self.file_descriptor {
            match syscall::close(fd) {
                Ok(_) => {
                    x86_64::print_str("\nFile closed\n");
                }
                Err(_) => {
                    x86_64::print_str("\nError closing file\n");
                }
            };
            self.file_descriptor = None;
        }
    }

    pub unsafe fn load(&self) -> (usize, Option<usize>) {
        let program_origin = self.load_program_michail().unwrap();

        let interpreter_filepath = self.load_interpreter_michail().unwrap();
        let interpreter_origin = if interpreter_filepath != usize::MAX {
            let elf = ELF::read_filepath(interpreter_filepath as *mut u8).unwrap();
            Some(elf.load_program_michail().unwrap())
        } else {
            None
        };

        (program_origin, interpreter_origin)
    }
    pub unsafe fn load_program(&self) -> Result<usize> {
        let mut begining = usize::MAX;

        if let Some(program_table) = &self.program_table {
            let loadable_headers = program_table.headers.iter().filter(|hdr| hdr.ptype == 1);

            for program_header in loadable_headers {
                program_header.print();
                match program_header.load(self.file_descriptor.unwrap()) {
                    Ok(addr) => {
                        x86_64::print_str("\nProgram segment loaded at ");
                        if addr < begining {
                            begining = addr;
                        }
                    }
                    Err(_) => {
                        x86_64::print_str("\nError loading program segment");
                        return Err(Error::TODO);
                    }
                };
                x86_64::print_str("\n----------------------------------------\n");
            }
        }

        Ok(begining)
    }

    pub unsafe fn load_program_michail(&self) -> Result<usize> {
        if let Some(program_table) = &self.program_table {
            x86_64::print_str("\nProgram Table\n");
            let loadable_headers = program_table.headers.iter().filter(|hdr| hdr.ptype == 1);

            let mut minva = usize::MAX;
            let mut maxva = 0;

            for program_header in loadable_headers {
                if program_header.vaddr < minva {
                    minva = program_header.vaddr.0 as usize;
                }
                if program_header.vaddr.0 as usize + program_header.memsz.0 as usize > maxva {
                    maxva = program_header.vaddr.0 as usize + program_header.memsz.0 as usize;
                }
            }

            minva = page::floor(minva);
            maxva = page::ceil(maxva);

            x86_64::print_str("\nMinva: ");
            x86_64::print_hex(minva as u64);
            x86_64::print_str("\nMaxva: ");
            x86_64::print_hex(maxva as u64);

            let hint = if is_dynamic() {
                core::ptr::null()
            } else {
                minva as *mut u8
            } as *mut u8;

            x86_64::print_str("\nHint: ");
            x86_64::print_hex(hint as u64);

            let mut flags = if is_dynamic() {
                0
            } else {
                syscall::mmap::flags::Map::Fixed.to()
            };

            flags |= (syscall::mmap::flags::Map::Private.to()
                | syscall::mmap::flags::Map::Anonymous.to());

            x86_64::print_str("\nMapping memory\n");
            x86_64::print_str("\nHint: ");
            x86_64::print_hex(hint as u64);
            x86_64::print_str("\nMinva: ");
            x86_64::print_hex(minva as u64);
            x86_64::print_str("\nMaxva: ");
            x86_64::print_hex(maxva as u64);
            x86_64::print_str("\nFlags: ");
            x86_64::print_hex(flags as u64);
            x86_64::print_str("\n");

            x86_64::print_str("\nMapping memory\n");
            let base = match syscall::mmap(
                hint,
                maxva - minva,
                syscall::mmap::flags::Prot::None.to(),
                flags,
                -1,
                0,
            ) {
                Ok(addr) => {
                    x86_64::print_str("\nMapped at: ");
                    x86_64::print_hex(addr as u64);
                    x86_64::print_str("\n");
                    addr
                }
                Err(_) => {
                    x86_64::print_str("\nError mapping memory\n");
                    return Err(Error::TODO);
                }
            };

            x86_64::print_str("\nMapped memory\n");
            x86_64::print_str("\nBase: ");
            x86_64::print_hex(base as u64);

            match syscall::munmap(base, maxva - minva) {
                Ok(_) => {}
                Err(_) => {
                    x86_64::print_str("\nError unmapping memory\n");
                    return Err(Error::TODO);
                }
            };

            let flags = {
                let mut flags = 0;
                flags |= syscall::mmap::flags::Map::Fixed.to();
                flags |= syscall::mmap::flags::Map::Private.to();
                flags |= syscall::mmap::flags::Map::Anonymous.to();
                flags
            };

            for program_header in program_table.headers {
                if program_header.ptype != 1 {
                    continue;
                }

                let off: usize = page::offset(program_header.vaddr.0 as usize);

                let mut start: usize = if is_dynamic() { base as usize } else { 0 };
                start += page::floor(program_header.vaddr.0 as usize);

                let size = page::ceil(program_header.memsz.0 as usize + off);
                // let size = program_header.memsz.0 as usize + off;

                x86_64::print_str("\nMapping memory\n");
                x86_64::print_str("\n\tStart: ");
                x86_64::print_hex(start as u64);
                x86_64::print_str("\n\tSize: ");
                x86_64::print_hex(size as u64);
                x86_64::print_str("\n\tFlags: ");
                x86_64::print_hex(flags as u64);
                x86_64::print_str("\n");
                let p = match syscall::mmap(
                    start as *mut u8,
                    size,
                    syscall::mmap::flags::Prot::Write.to(),
                    flags,
                    -1,
                    0,
                ) {
                    Ok(addr) => addr,
                    Err(_) => {
                        x86_64::print_str("\nError mapping memory\n");
                        return Err(Error::TODO);
                    }
                };

                match syscall::lseek(
                    self.file_descriptor.unwrap(),
                    program_header.offset.0 as i64,
                    syscall::lseek::flags::Seek::SET.to(),
                ) {
                    Ok(_) => {}
                    Err(_) => {
                        x86_64::print_str("\nError seeking file\n");
                        return Err(Error::TODO);
                    }
                }

                match syscall::read(
                    self.file_descriptor.unwrap(),
                    p.add(off),
                    program_header.filesz.0 as usize,
                ) {
                    Ok(_) => {}
                    Err(_) => {
                        x86_64::print_str("\nError reading file\n");
                        return Err(Error::TODO);
                    }
                }
                let prot = {
                    let mut p = 0;

                    if program_header.flags.0 & program::header::Flag::R.to() != 0 {
                        p = p | syscall::mmap::flags::Prot::Read.to()
                    }

                    if program_header.flags.0 & program::header::Flag::W.to() != 0 {
                        p = p | syscall::mmap::flags::Prot::Write.to()
                    }

                    if program_header.flags.0 & program::header::Flag::X.to() != 0 {
                        p = p | syscall::mmap::flags::Prot::Exec.to()
                    }

                    p
                };

                match syscall::mprotect(p, size, prot) {
                    Ok(_) => {}
                    Err(_) => {
                        x86_64::print_str("\nError protecting memory\n");
                        return Err(Error::TODO);
                    }
                };
            }
            return Ok(base as usize);
        }

        Ok(usize::MAX)
    }

    pub unsafe fn load_interpreter_michail(&self) -> Result<usize> {
        if let Some(program_table) = &self.program_table {
            x86_64::print_str("\nProgram Table\n");
            let interpreter_headers = program_table.headers.iter().filter(|hdr| hdr.ptype == 3);

            for interpreter_header in interpreter_headers {
                let pointer = match syscall::mmap(
                    core::ptr::null_mut(),
                    interpreter_header.filesz.0 as usize,
                    syscall::mmap::flags::Prot::Read.to() | syscall::mmap::flags::Prot::Write.to(),
                    syscall::mmap::flags::Map::Anonymous.to()
                        | syscall::mmap::flags::Map::Private.to(),
                    -1,
                    0,
                ) {
                    Ok(addr) => {
                        x86_64::print_str("\nInterpreter loaded at ");
                        x86_64::print_hex(addr as u64);
                        addr
                    }
                    Err(_) => {
                        x86_64::print_str("\nError loading interpreter");
                        return Err(Error::TODO);
                    }
                };

                match syscall::lseek(
                    self.file_descriptor.unwrap(),
                    interpreter_header.offset.0 as i64,
                    syscall::lseek::flags::Seek::SET.to(),
                ) {
                    Ok(_) => {}
                    Err(_) => {
                        x86_64::print_str("\nError seeking file\n");
                        return Err(Error::TODO);
                    }
                }

                match syscall::read(
                    self.file_descriptor.unwrap(),
                    pointer,
                    interpreter_header.filesz.0 as usize,
                ) {
                    Ok(_) => {}
                    Err(_) => {
                        x86_64::print_str("\nError reading file\n");
                        return Err(Error::TODO);
                    }
                }

                x86_64::print_str("\nInterpreter: '");
                match syscall::write(1, pointer, interpreter_header.filesz.0 as usize) {
                    Ok(_) => {}
                    Err(_) => {
                        x86_64::print_str("\nError writing file\n");
                        return Err(Error::TODO);
                    }
                }
                x86_64::print_str("'\n");
                return Ok(pointer as usize);
            }
        }
        Ok(usize::MAX)
    }

    pub unsafe fn execute(&self) -> ! {
        loop {}
    }

    pub unsafe fn execute_address(address: usize) -> ! {
        loop {}
    }

    pub fn is_valid(&self) -> bool {
        self.header.is_valid()
    }

    pub fn is_executable(&self) -> bool {
        self.header.etype == 2
    }
}
