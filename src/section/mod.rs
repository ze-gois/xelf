use crate::dtype;

/// # Sections header
///
/// An object file's section header table lets one locate all the file's sections. The section header
/// table is an array of `header::Section` structures. A section header table index
/// is a subscript into this array. The ELF header's e_shoff member gives the byte offset from
/// the beginning of the file to the section header table; e_shnum tells how many entries the
/// section header table contains; e_shentsize gives the size in bytes of each entry.
pub mod header;
pub use header::Header;

pub mod index;
pub use index::Index;

pub mod entry;
pub use entry::Entry;

use crate::ELFHeader;

/// # Standard sections
///
/// The standard sections used for program code and data are shown in Table 12.
/// The standard sections used for other object file information are shown in
/// Table 13. In the flags column, “A” stands for SHF_ALLOC, “W” for SHF_WRITE, and
/// “X” for SHF_EXECINSTR.
pub mod standard;

#[repr(C)]
pub struct Table {
    pub offset: dtype::Off,
    pub counter: usize,
    pub entries: *mut Entry,
    pub cursor: usize,
}

impl core::iter::Iterator for Table {
    type Item = Entry;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.counter {
            return None;
        }
        let entry = unsafe { (*self.entries.add(self.cursor)).clone() };
        Some(entry)
    }
}

impl Table {
    pub fn read_from_filepath(filepath: &str) -> Self {
        let file_descriptor = crate::open_filepath(filepath);
        Self::read_from_file_descriptor(file_descriptor)
    }

    pub fn read_from_file_descriptor(file_descriptor: isize) -> Self {
        let elf_header = ELFHeader::read_from_file_descriptor(file_descriptor);
        Self::read_from_elf_header(file_descriptor, &elf_header)
    }

    pub fn read_from_elf_header(
        file_descriptor: isize,
        elf_header: &ELFHeader,
    ) -> crate::Result<Self> {
        let mut offset = elf_header.shoff;
        let endianness = elf_header.get_identifier().get_endianness();
        let number_of_entries = elf_header.shnum;

        let entries_pointer = crate::alloc::<Entry>(number_of_entries.0 as usize)?;

        for e in 0..number_of_entries.0 as usize {
            unsafe {
                *entries_pointer.add(e) =
                    Entry::read_from_file_descriptor(file_descriptor, endianness)?
            }
        }

        Self {
            offset,
            entries: (0..number_of_entries)
                .map(|_| Entry {
                    header: crate::SectionHeader::read_from_file_descriptor(
                        filemap,
                        &mut offset,
                        &endianess,
                    ),
                    content: core::ptr::null_mut(),
                })
                .collect::<Vec<crate::SectionEntry>>(),
        }
    }
}

impl core::fmt::Debug for Table {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "Section table: {{")?;
        writeln!(f, "\tOffset:{:?}", self.offset)?;
        writeln!(f, "\tEntries: {{")?;
        for (e, entry) in self.entries.iter().enumerate() {
            writeln!(f, "\t\t{e}: {:?}", entry.header)?;
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
        writeln!(f, "Section table: {{")?;
        writeln!(f, "\tOffset:{:?}", self.offset)?;
        writeln!(f, "\tEntries: {{")?;
        for (e, entry) in self.entries.iter().enumerate() {
            writeln!(f, "\t\t{e}: {:?}", entry.header)?;
        }
        writeln!(f, "\t}}")?;
        writeln!(f, "}}")?;
        //     "Table {{ offset: {:?}, entries: {:?} }}",
        //     self.offset, self.entries
        // )
        Ok(())
    }
}
