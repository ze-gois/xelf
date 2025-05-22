pub mod flag;
pub mod ptype;

use crate::dtype::*;
use crate::page;
use crate::Result;

use crate::error::*;

pub use flag::Flag;
pub use ptype::PType;

use mmap::flags::Map as MapFlag;
use x86_64::syscall::mprotect;
use x86_64::syscall::{lseek, mmap, read};
use x86_64::ToFlags;

#[repr(C)]
pub struct Header {
    pub ptype: Word,   /* Type of segment */
    pub flags: Word,   /* Segment attributes */
    pub offset: Off,   /* Offset in file */
    pub vaddr: Addr,   /* Virtual address in memory */
    pub paddr: Addr,   /* Reserved */
    pub filesz: XWord, /* Size of segment in file */
    pub memsz: XWord,  /* Size of segment in memory */
    pub align: XWord,  /* Alignment of segment */
}

impl Header {
    pub fn size() -> usize {
        core::mem::size_of::<Header>()
    }

    pub fn size2() -> usize {
        Word::SIZE_BYTES
            + Word::SIZE_BYTES
            + Off::SIZE_BYTES
            + Addr::SIZE_BYTES
            + Addr::SIZE_BYTES
            + XWord::SIZE_BYTES
            + XWord::SIZE_BYTES
            + XWord::SIZE_BYTES
    }

    pub unsafe fn read(fd: i32) -> Result<Header> {
        // Read program header
        Ok(Header {
            ptype: Word::read(fd)?,
            flags: Word::read(fd)?,
            offset: Off::read(fd)?,
            vaddr: Addr::read(fd)?,
            paddr: Addr::read(fd)?,
            filesz: XWord::read(fd)?,
            memsz: XWord::read(fd)?,
            align: XWord::read(fd)?,
        })
    }

    pub unsafe fn print(&self) {
        x86_64::print_str("\tSegmentHeader { ");
        x86_64::print_str("ptype: ");
        self.ptype.print();
        x86_64::print_str(" (");
        self.ptype.print_hex();
        x86_64::print_str("), ");
        x86_64::print_str("flags: ");
        self.flags.print();
        x86_64::print_str(" (");
        self.flags.print_hex();
        x86_64::print_str("), ");
        x86_64::print_str("offset: ");
        self.offset.print();
        x86_64::print_str(" (");
        self.offset.print_hex();
        x86_64::print_str("), ");
        x86_64::print_str("vaddr: ");
        self.vaddr.print();
        x86_64::print_str(" (");
        self.vaddr.print_hex();
        x86_64::print_str("), ");
        x86_64::print_str("paddr: ");
        self.paddr.print();
        x86_64::print_str(" (");
        self.paddr.print_hex();
        x86_64::print_str("), ");
        x86_64::print_str("filesz: ");
        self.filesz.print();
        x86_64::print_str(" (");
        self.filesz.print_hex();
        x86_64::print_str("), ");
        x86_64::print_str("memsz: ");
        self.memsz.print();
        x86_64::print_str(" (");
        self.memsz.print_hex();
        x86_64::print_str("), ");
        x86_64::print_str("align: ");
        self.align.print();
        x86_64::print_str(" (");
        self.align.print_hex();
        x86_64::print_str(")\n");
    }

    pub unsafe fn load_mikhail(&self, fd: i32, base: *mut u8) -> Result<usize> {
        if self.ptype != PType::Load.to() {
            return Ok(usize::MAX);
        }

        let flags = MapFlag::Private.to() | MapFlag::Anonymous.to() | MapFlag::Fixed.to();

        let off = page::offset(self.vaddr.0 as usize);

        let mut start = if crate::is_dynamic() {
            base as usize
        } else {
            0
        };
        x86_64::print_str("\nBase: ");
        x86_64::print_hex(start as u64);
        start += page::floor(self.vaddr.0 as usize);

        let sz = page::ceil(self.memsz.0 as usize + off);

        let p = match mmap(
            start as *mut u8,
            sz,
            mmap::flags::Prot::Write.to(),
            flags,
            -1,
            0,
        ) {
            Ok(p) => p as *mut u8,
            Err(e) => return Err(Error::from(e)),
        };

        match lseek(fd, self.offset.0 as i64, lseek::flags::Seek::SET.to()) {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::from(e));
            }
        };

        match read(fd, p.add(off), self.filesz.0 as usize) {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::from(e));
            }
        };

        let prot = {
            let mut p = 0;

            if self.flags.0 & Flag::R.to() != 0 {
                p = p | mmap::flags::Prot::Read.to()
            }

            if self.flags.0 & Flag::W.to() != 0 {
                p = p | mmap::flags::Prot::Write.to()
            }

            if self.flags.0 & Flag::X.to() != 0 {
                p = p | mmap::flags::Prot::Exec.to()
            }

            p
        };
        match mprotect(p, sz, prot) {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::from(e));
            }
        };

        Ok(p as usize)
    }

    pub unsafe fn load(&self, fd: i32) -> Result<usize> {
        let flags = MapFlag::Private.to() | MapFlag::Anonymous.to() | MapFlag::Fixed.to();

        match x86_64::syscall::prctl(
            x86_64::syscall::prctl::PR_SET_MM,
            x86_64::syscall::prctl::MM_MAP_MIN_ADDR,
            0,
            0,
            0,
        ) {
            Ok(_) => x86_64::print_str("succeeded\n"),
            Err(e) => {
                x86_64::print_str("failed with error: ");
                x86_64::print_dec(e as u64);
                x86_64::print_str("\n");
            }
        }

        let mmap_address = page::floor(self.vaddr.0 as usize);
        let mmap_offset = self.vaddr.0 as usize - mmap_address;
        let mmap_length = self.memsz.0 as usize + mmap_offset;

        let mmap_address_terminus = page::ceil(self.memsz.0 as usize + mmap_address + mmap_offset);
        let mmap_length_total = mmap_address_terminus - mmap_address;

        x86_64::print_str("\nMapping segment at ");
        x86_64::print_hex(mmap_address as u64);
        x86_64::print_str("\nMapping offset: ");
        x86_64::print_hex(mmap_offset as u64);
        x86_64::print_str("\nMapping end: ");
        x86_64::print_hex(mmap_address_terminus as u64);
        x86_64::print_str("\nMapping length: ");
        x86_64::print_hex(mmap_length as u64);
        x86_64::print_str("( ");
        x86_64::print_hex(mmap_length as u64);
        x86_64::print_str(" )");

        // Map segment
        x86_64::print_str("\nMapping... ");
        let mmap_address_pointer = match mmap(
            mmap_address as *mut u8, // Virtual address to the begining of the page just before the segment vaddr
            mmap_length,             // Length of maped memory
            mmap::flags::Prot::Write.to(),
            flags,
            -1,
            0,
        ) {
            Ok(p) => {
                x86_64::print_str("\nMmap succeeded: ");
                x86_64::print_hex(p as u64);
                p as *mut u8
            }
            Err(e) => {
                x86_64::print_str("\nMmap failed with error: ");
                x86_64::print_dec(e as u64);
                x86_64::print_str("\n");
                return Err(Error::from(e));
            }
        };
        x86_64::print_str(" done.\n");

        if self.filesz > 0 {
            // Seek to the segment data
            match lseek(
                fd,
                self.offset.0 as i64,
                lseek::flags::Seek::SET.to() as i32,
            ) {
                Ok(_) => {
                    x86_64::print_str("\nLseek succeeded: ");
                    x86_64::print_hex(self.offset.0 as u64);
                }
                Err(e) => {
                    x86_64::print_str("\nLseek failed with error: ");
                    x86_64::print_dec(e as u64);
                    x86_64::print_str("\n");
                    return Err(Error::from(e));
                }
            };

            // Read the file content
            x86_64::print_str("\nReading... ");
            x86_64::print_hex(self.filesz.0 as u64);
            x86_64::print_str(" bytes from ");
            x86_64::print_hex(fd as u64);
            x86_64::print_str(" to ");
            x86_64::print_hex(mmap_address_pointer as u64);
            // match read(fd, self.vaddr.0 as *mut u8, self.filesz.0 as usize) {
            match read(
                fd,
                mmap_address_pointer.add(mmap_offset),
                self.filesz.0 as usize,
            ) {
                Ok(_) => {
                    x86_64::print_str("\nRead succeeded: ");
                    x86_64::print_hex(self.filesz.0 as u64);
                }
                Err(e) => {
                    x86_64::print_str("\nRead failed with error: ");
                    x86_64::print_dec(e as u64);
                    x86_64::print_str("\n");
                    return Err(Error::from(e));
                }
            };
        }

        // Calculate protection flags
        let prot = {
            let mut p = 0;

            if self.flags.0 & Flag::R.to() != 0 {
                p = p | mmap::flags::Prot::Read.to()
            }

            if self.flags.0 & Flag::W.to() != 0 {
                p = p | mmap::flags::Prot::Write.to()
            }

            if self.flags.0 & Flag::X.to() != 0 {
                p = p | mmap::flags::Prot::Exec.to()
            }

            p
        };

        match mprotect(mmap_address_pointer, mmap_length, prot) {
            Ok(_) => {
                x86_64::print_str("\nMprotect succeeded: ");
                x86_64::print_hex(prot as u64);
            }
            Err(e) => {
                x86_64::print_str("\nMprotect failed with error: ");
                x86_64::print_dec(e as u64);
                x86_64::print_str("\n");
                return Err(Error::from(e));
            }
        };

        x86_64::print_str("\nRestoring min map addr... ");
        match x86_64::syscall::prctl(
            x86_64::syscall::prctl::PR_SET_MM,
            x86_64::syscall::prctl::MM_MAP_MIN_ADDR,
            0x10000, // Default value
            0,
            0,
        ) {
            Ok(_) => x86_64::print_str("succeeded\n"),
            Err(e) => {
                x86_64::print_str("failed with error: ");
                x86_64::print_dec(e as u64);
                x86_64::print_str("\n");
            }
        }

        x86_64::print_str("\nSegment loaded\n");
        Ok(mmap_address_pointer as usize)
    }
}
