use crate::arch64::data;
use crate::arch64::header;
use crate::arch64::header::elf::identifier::Endianess;
use crate::arch64::table;

#[derive(Clone)]
pub struct Table {
    offset: data::Off,
    entries: Vec<Entry>,
}

#[derive(Clone)]
pub struct Entry {
    name: String,
}

impl Table {
    pub fn load_from_filepath<P: AsRef<std::path::Path>>(fp: P) -> Vec<Self> {
        let f = std::fs::File::open(fp).unwrap();
        let filemap = unsafe { memmap2::Mmap::map(&f).unwrap() };
        Self::load_from_filemap(&filemap)
    }

    pub fn load_from_filemap(filemap: &memmap2::Mmap) -> Vec<Self> {
        let elf_header = header::ELF::load_from_memmap(&filemap);
        Self::load_from_elf_header(filemap, &elf_header)
    }

    pub fn load_from_elf_header(filemap: &memmap2::Mmap, elf_header: &header::ELF) -> Vec<Self> {
        // let mut offset = elf_header.get_shoff();
        let endianess = elf_header.get_identifier().get_endianess();
        // let number_of_entries = elf_header.get_shnum();

        let section_table = table::Section::load_from_memmap(&filemap);

        Self::load_from_section_table(&filemap, &endianess, section_table)
    }

    pub fn load_from_section_table(
        filemap: &memmap2::Mmap,
        endianess: &Endianess,
        section_table: table::Section,
    ) -> Vec<Self> {
        let string_tables: Vec<header::Section> = section_table
            .entries
            .into_iter()
            .filter(|e| e.get_stype() == header::section::Type::StrTab)
            .collect();

        string_tables
            .into_iter()
            .map(|string_table| Self::load_from_section_header(filemap, string_table, &endianess))
            .collect()
    }

    pub fn load_from_section_header(
        filemap: &memmap2::Mmap,
        string_table: header::Section,
        endianess: &Endianess,
    ) -> Table {
        let mut offset = string_table.copy_offset();

        Table {
            offset,
            entries: {
                data::read_and_seek_n_unsigned_char(
                    &filemap,
                    &mut offset,
                    &endianess,
                    string_table.copy_size() as usize,
                )
                .split(|byte| *byte == 0)
                .map(|segment| Entry {
                    name: String::from_utf8_lossy(segment).into_owned(),
                }) // convert to String
                .collect()
            },
        }
    }
}

impl std::fmt::Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "String Table {{")?;
        writeln!(f, "\tOffset {}, ", &self.offset)?;
        writeln!(f, "\tentries: \n{:?}", &self.entries)?;
        writeln!(f, "]}}")
    }
}

impl std::fmt::Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\t\tEntry '{}'", &self.name)
    }
}
