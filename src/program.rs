use crate::dtype;

pub mod header;
pub use header::Header;

pub mod entry;
pub use entry::Entry;

#[derive(Clone)]
#[repr(C)]
pub struct Table {
    pub offset: dtype::Off,
    pub counter: usize,
    pub entries: *mut Entry,
    pub pointer: usize,
}

impl core::iter::Iterator for Table {
    type Item = Entry;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pointer >= self.counter {
            return None;
        }
        let entry = unsafe { (*self.entries.add(self.pointer)).clone() };
        self.pointer += 1;
        Some(entry)
    }
}

impl Table {
    pub fn from_filepath(filepath: &str) -> crate::Result<Self> {
        let file_descriptor = crate::open_filepath(filepath);
        Self::from_file_descriptor(file_descriptor)
    }

    pub fn from_file_descriptor(file_descriptor: isize) -> crate::Result<Self> {
        let elf_header = crate::ELFHeader::from_file_descriptor(file_descriptor);
        Self::from_elf_header(file_descriptor, &elf_header)
    }

    pub fn from_elf_header(
        file_descriptor: isize,
        elf_header: &crate::ELFHeader,
    ) -> crate::Result<Self> {
        let endianess = elf_header.get_identifier().get_endianness();
        let number_of_entries = elf_header.phnum;
        let offset = elf_header.phoff;

        syscall::lseek(
            file_descriptor as i32,
            offset.0 as i64,
            syscall::lseek::Flag::SET.to(),
        );

        let entries_pointer = crate::alloc::<Entry>(number_of_entries.0 as usize).unwrap();

        for e in 0..number_of_entries.0 {
            let header = Header::from_file_descriptor(file_descriptor, endianess)?;
            unsafe {
                *entries_pointer.add(e as usize) = Entry { header };
            }
        }
        Ok(Self {
            offset,
            counter: number_of_entries.0 as usize,
            entries: entries_pointer,
        })
    }

    // pub fn sort(&mut self) {
    //     self.entries.sort_by_key(|e| e.header.vaddr);
    // }
}

impl core::fmt::Debug for Table {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "Program table: {{")?;
        writeln!(f, "\tOffset:{:?}", self.offset)?;
        writeln!(f, "\tEntries: {{")?;
        for c in 0..self.counter {
            let entry = unsafe { (*self.entries.add(c)).clone() };
            writeln!(f, "\t\t{c}: {:?}", entry.header)?;
        }
        writeln!(f, "\t}}")?;
        writeln!(f, "}}")?;
        //     "Table {{ offset: {:?}, entries: {:?} }}",
        //     self.offset, self.entries
        // )
        Ok(())
    }
}

impl core::fmt::Display for Table {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "{:?}", self)?;
        Ok(())
    }
}
