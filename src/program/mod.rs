use crate::arch;

pub mod header;
pub use header::Header;

pub mod entry;
pub use entry::Entry;

#[derive(Clone)]
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

    pub fn read_from_memmap(file_descriptor: isize) -> Self {
        let elf_header = crate::ELFHeader::read_from_file_descriptor(&file_descriptor);
        Self::read_from_elf_header(&file_descriptor, &elf_header)
    }

    pub fn read_from_elf_header(filemap: &memmap2::Mmap, elf_header: &crate::ELFHeader) -> Self {
        let mut offset = elf_header.phoff;
        let endianess = elf_header.get_identifier().get_endianess();
        let number_of_entries = elf_header.phnum;

        Self {
            offset,
            entries: (0..number_of_entries)
                .map(|_| {
                    let header =
                        crate::ProgramHeader::read_from_memmap(filemap, &mut offset, &endianess);
                    crate::ProgramEntry { header }
                })
                .collect::<Vec<crate::ProgramEntry>>(),
        }
    }

    pub fn sort(&mut self) {
        self.entries.sort_by_key(|e| e.header.vaddr);
    }
}

impl std::fmt::Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Program table: {{")?;
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

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Program table: {{")?;
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
