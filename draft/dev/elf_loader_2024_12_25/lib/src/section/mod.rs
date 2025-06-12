use crate::arch;

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
    pub offset: arch::Off,
    pub entries: Vec<Entry>,
}

impl Table {
    pub fn read_from_filepath<P: AsRef<std::path::Path>>(filepath: P) -> Self {
        let file = std::fs::File::open(filepath).unwrap();
        let filemap = unsafe { memmap2::Mmap::map(&file).unwrap() };
        Self::read_from_memmap(&filemap)
    }

    pub fn read_from_memmap(filemap: &memmap2::Mmap) -> Self {
        let elf_header = ELFHeader::read_from_memmap(&filemap);
        Self::read_from_elf_header(&filemap, &elf_header)
    }

    pub fn read_from_elf_header(filemap: &memmap2::Mmap, elf_header: &ELFHeader) -> Self {
        let mut offset = elf_header.shoff;
        let endianess = elf_header.get_identifier().get_endianess();
        let number_of_entries = elf_header.shnum;

        Self {
            offset,
            entries: (0..number_of_entries)
                .map(|_| Entry {
                    header: crate::SectionHeader::read_from_memmap(
                        filemap,
                        &mut offset,
                        &endianess,
                    ),
                    content: vec![],
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
