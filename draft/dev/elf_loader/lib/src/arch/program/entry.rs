use super::Header;

#[derive(Clone)]
pub struct Entry {
    pub header: Header,
}

impl Entry {
    /// 5.1 Program Loading
    ///
    /// Program loading is a process of mapping file segments to virtual memory seg-
    /// ments. For efficient mapping executable and shared object files must have seg-
    /// ments whose file offsets and virtual addresses are congruent modulo the page
    /// size.
    ///
    /// To save space the file page holding the last page of the text segment may
    /// also contain the first page of the data segment. The last data page may contain file
    /// information not relevant to the running process. Logically, the system enforces the
    /// memory permissions as if each segment were complete and separate; segments’
    /// addresses are adjusted to ensure each logical page in the address space has a single
    /// set of permissions. In the example above, the region of the file holding the end
    /// of text and the beginning of data will be mapped twice: at one virtual address for
    /// text and at a different virtual address for data.
    ///
    /// The end of the data segment requires special handling for uninitialized data,
    /// which the system defines to begin with zero values. Thus if a file’s last data page
    /// includes information not in the logical memory page, the extraneous data must be
    /// set to zero, not the unknown contents of the executable file. “Impurities” in the
    /// other three pages are not logically part of the process image; whether the system
    /// expunges them is unspecified.
    ///
    /// One aspect of segment loading differs between executable files and shared
    /// objects. Executable file segments typically contain absolute code (see section 3.5
    /// “Coding Examples”). For the process to execute correctly, the segments must
    /// reside at the virtual addresses used to build the executable file. Thus the system
    /// uses the p_vaddr values unchanged as virtual addresses.
    ///
    /// On the other hand, shared object segments typically contain position-independent
    /// code. This lets a segments virtual address change from one process to another,
    /// without invalidating execution behavior. Though the system chooses virtual ad-
    /// dresses for individual processes, it maintains the segments’ relative positions. Be-
    /// cause position-independent code uses relative addressing between segments, the
    /// difference between virtual addresses in memory must match the difference be-
    /// tween virtual addresses in the file.
    pub fn load(&self, elf_file: &crate::ELF) -> *mut libc::c_void {
        // if self.header.get_ptype() != crate::program::header::Type::Load {
        //     return core::ptr::null_mut();
        // }

        // // let mut offset = self.header.offset;
        // let mut offset = self.header.offset & crate::data::memory::ALIGN;

        // let start = if elf_file.is_dynamic() {
        //     self.header.vaddr
        // } else {
        //     0
        // };

        // let mut address = self.header.vaddr;
        // let mut align = self.header.align;
        // let mut size = self.header.memsz;
        // let mut flags = self.header.flags;
        // println!("{:?}", self.header);
        // print!("{:?},\t", flags);
        // // let flags = libc::MAP_FIXED | libc::MAP_ANONYMOUS | libc::MAP_PRIVATE;
        // print!("{:?},\n", flags);

        //libc::mmap(addr, len, prot, flags, fd, offset)
        // offset =
        // crate::data::memory::truncate_to_page(size + offset)

        // let map_minimum_address = libc::mmap(addr, len, prot, flags, fd, offset);
        // unsafe {
        //     let protected_output = libc::mprotect(
        //         map_minimum_address as *mut libc::c_void,
        //         size as libc::size_t,
        //         crate::program::header::Flag::from_posix(flags).to_posix() as libc::c_int,
        //     );
        // }
        core::ptr::null_mut()
    }

    pub fn is_interpreter(&self) -> bool {
        self.header.ptype == super::header::Type::Interp.to()
    }

    pub fn is_loadable(&self) -> bool {
        self.header.ptype == super::header::Type::Load.to()
    }

    pub fn is_dynamic(&self) -> bool {
        self.header.ptype == super::header::Type::Dynamic.to()
    }
}
