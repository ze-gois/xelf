pub mod index;

pub use index::Index;

pub use crate::arch32::header;

#[derive(Clone)]
pub struct Table {
    pub entries: Vec<header::Section>,
}

impl Table {
    pub fn load_from_filepath<P: AsRef<std::path::Path>>(filepath: P) -> Self {
        let file = std::fs::File::open(filepath).unwrap();
        let filemap = unsafe { memmap2::Mmap::map(&file).unwrap() };
        Self::load_from_memmap(&filemap)
    }

    pub fn load_from_memmap(filemap: &memmap2::Mmap) -> Self {
        let elf_header = header::ELF::load_from_memmap(&filemap);
        Self::load_from_elf_header(&filemap, &elf_header)
    }

    pub fn load_from_elf_header(filemap: &memmap2::Mmap, elf_header: &header::ELF) -> Self {
        let mut offset = elf_header.get_shoff();
        let endianess = elf_header.get_identifier().get_endianess();
        let number_of_entries = elf_header.get_shnum();

        // doubt : is slice::from_raw_parts applicable?
        Self {
            entries: (0..number_of_entries)
                .map(|_| header::Section::load_from_memmap(filemap, &mut offset, &endianess))
                .collect::<Vec<header::Section>>(),
        }
    }

    pub fn get_entry_by_type(&self, stype: header::section::Type) -> Vec<header::Section> {
        self.entries
            .clone()
            .into_iter()
            .filter(|section| section.get_stype() == stype)
            .collect::<Vec<header::Section>>()
    }
}
