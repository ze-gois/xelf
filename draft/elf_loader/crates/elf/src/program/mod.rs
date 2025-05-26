pub mod header;

pub use header::Header;
use x86_64::syscall::read;

use crate::dtype;
use crate::error::{Error, Result};
use crate::page;

// use x86_64::syscall;

// pub type Table<'elf> = &'elf [Entry];

pub struct Table<'elf> {
    pub offset: dtype::Off,
    pub nof_entries: dtype::Half,
    pub headers: &'elf [Header],
}

impl<'elf> Table<'elf> {
    pub unsafe fn read(
        fd: i32,
        offset: dtype::Off,
        nof_entries: dtype::Half,
    ) -> Result<Table<'elf>> {
        use x86_64::syscall::{lseek, mmap};
        use x86_64::ToFlags;

        let header_size = core::mem::size_of::<Header>();
        let header_table_size = header_size * nof_entries.0 as usize;

        x86_64::print_str("Header size: ");
        x86_64::print_dec(header_size as u64);
        x86_64::print_str("\nTable size: ");
        x86_64::print_dec(header_table_size as u64);
        x86_64::print_str("\nOffset: ");
        offset.print();

        x86_64::print_str("\nPreparing to map... ");

        let map_ptr = match mmap(
            core::ptr::null_mut(),
            header_table_size,
            mmap::flags::Prot::Read.to() | mmap::flags::Prot::Write.to(),
            mmap::flags::Map::Private.to() | mmap::flags::Map::Anonymous.to(),
            -1,
            0,
        ) {
            Ok(ptr) => {
                x86_64::print_str("Mapped at: ");
                x86_64::print_hex(ptr as u64);
                x86_64::print_str("\n");
                ptr
            }
            Err(e) => {
                x86_64::print_str("Mmap failed with error: ");
                x86_64::print_dec(e as u64);
                x86_64::print_str("\n");
                return Err(Error::from(e));
            }
        };

        match lseek(fd, offset.0 as i64, lseek::flags::Seek::SET.to()) {
            Ok(_) => {}
            Err(e) => {
                x86_64::print_str("Lseek failed with error: ");
                x86_64::print_dec(e as u64);
                x86_64::print_str("\n");
                return Err(Error::from(e));
            }
        };

        match read(fd, map_ptr, header_table_size) {
            Ok(_) => {}
            Err(e) => {
                x86_64::print_str("Read failed with error: ");
                x86_64::print_dec(e as u64);
                x86_64::print_str("\n");
                return Err(Error::from(e));
            }
        };

        x86_64::print_str("\nFirst few program headers (raw):\n");
        let bytes =
            core::slice::from_raw_parts(map_ptr as *const u8, core::mem::size_of::<Header>() * 2); // Show first two headers
        for (i, &byte) in bytes.iter().enumerate() {
            if i % 8 == 0 {
                x86_64::print_str("\n");
            }
            x86_64::print_hex(byte as u64);
            x86_64::print_str(" ");
        }
        x86_64::print_str("\n");

        x86_64::print_str("Mapped\n");
        // Adjust pointer for the offset
        let headers_ptr = (map_ptr as usize) as *mut Header;
        let headers = core::slice::from_raw_parts(headers_ptr, nof_entries.into());

        Ok(Table {
            offset,
            nof_entries,
            headers,
        })
    }

    pub unsafe fn load(&self, fd: i32) {
        // for header in self.headers.iter() {
        //     header.load(fd);
        // }
    }
}
