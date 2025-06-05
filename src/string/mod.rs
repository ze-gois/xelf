use crate::dtype;

pub mod entry;
pub use entry::Entry;

#[derive(Clone)]
#[repr(C)]
pub struct Table {
    pub offset: dtype::Off,
    pub entries: *mut Entry,
    pub counter: usize,
}

impl Table {
    pub fn read_from_filepath(filepath: &str) -> Self {
        let file_descriptor = crate::open_filepath(filepath);
        Self::read_from_file_descriptor(file_descriptor)
    }

    pub fn read_from_file_descriptor(file_descriptor: isize) -> Vec<Self> {
        let elf_header = crate::ELFHeader::read_from_file_descriptor(&file_descriptor);
        Self::read_from_elf_header(file_descriptor, &elf_header)
    }

    pub fn read_from_elf_header(
        filemap: &memmap2::Mmap,
        elf_header: &crate::ELFHeader,
    ) -> Vec<Self> {
        // let mut offset = elf_header.get_shoff();
        let endianess = elf_header.get_identifier().get_endianess();
        // let number_of_entries = elf_header.get_shnum();

        let section_table = crate::SectionTable::read_from_memmap(&filemap);

        Self::read_from_section_table(&filemap, &endianess, section_table)
    }

    pub fn read_from_section_table(
        filemap: &memmap2::Mmap,
        endianess: &dtype::Endianess,
        section_table: crate::SectionTable,
    ) -> Vec<Self> {
        let string_tables: Vec<crate::SectionEntry> = section_table
            .entries
            .into_iter()
            .filter(|e| e.header.get_stype() == crate::section::header::Type::StrTab)
            .collect();

        string_tables
            .into_iter()
            .map(|entry| Self::read_from_section_header(filemap, &entry, &endianess))
            .collect()
    }

    pub fn read_from_section_header(
        filemap: &memmap2::Mmap,
        string_table_entry: &crate::SectionEntry,
        endianess: &dtype::Endianess,
    ) -> Table {
        let mut offset = string_table_entry.header.offset;

        Table {
            offset,
            entries: {
                dtype::read_and_seek_n_unsigned_char(
                    &filemap,
                    &mut offset,
                    &endianess,
                    string_table_entry.header.size as usize,
                )
                .split(|byte| *byte == 0)
                .map(|segment| Entry {
                    offset: 0,
                    content: String::from_utf8_lossy(segment).into_owned(),
                }) // convert to Stringcrate
                .collect()
            },
        }
    }
}

impl std::fmt::Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "String Table {{")?;
        writeln!(f, "\tOffset {}, ", &self.offset)?;
        writeln!(f, "\tEntries: {{")?;
        for (e, entry) in self.entries.iter().enumerate() {
            writeln!(f, "\t\t{e}:{}", entry.content)?;
        }
        writeln!(f, "\t}}")?;
        writeln!(f, "}}")?;
        Ok(())
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "String Table {{")?;
        writeln!(f, "\tOffset {}, ", &self.offset)?;
        writeln!(f, "\tEntries: {{")?;
        for (e, entry) in self.entries.iter().enumerate() {
            writeln!(f, "\t\t{e}:{}", entry.content)?;
        }
        writeln!(f, "\t}}")?;
        writeln!(f, "}}")?;
        Ok(())
    }
}
