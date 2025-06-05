pub use crate::arch::dtype::XWord as T;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Auxiliar {
    pub atype: T,
    pub value: T,
}

// Auxiliary vector types
#[repr(u64)]
pub enum AuxiliarType {
    Null = 0,      // End of auxiliary vector
    Phdr = 3,      // Program headers location
    Phent = 4,     // Size of program header entry
    Phnum = 5,     // Number of program headers
    Pagesz = 6,    // System page size
    Base = 7,      // Base address of interpreter
    Flags = 8,     // Flags
    Entry = 9,     // Program entry point
    Uid = 11,      // Real UID
    Euid = 12,     // Effective UID
    Gid = 13,      // Real GID
    Egid = 14,     // Effective GID
    Platform = 15, // String identifying platform
    Hwcap = 16,    // Machine-dependent CPU capability hints
    Clktck = 17,   // Frequency of times()
    Random = 25,   // Address of 16 random bytes
    ExecFn = 31,   // Filename of executed program
    Undefined = 33,
}

impl AuxiliarType {
    pub fn from(input: T) -> Self {
        match input {
            0 => Self::Null,
            3 => Self::Phdr,
            4 => Self::Phent,
            5 => Self::Phnum,
            6 => Self::Pagesz,
            7 => Self::Base,
            8 => Self::Flags,
            9 => Self::Entry,
            11 => Self::Uid,
            12 => Self::Euid,
            13 => Self::Gid,
            14 => Self::Egid,
            15 => Self::Platform,
            16 => Self::Hwcap,
            17 => Self::Clktck,
            25 => Self::Random,
            31 => Self::ExecFn,
            _ => Self::Undefined,
        }
    }

    pub fn to(&self) -> T {
        match &self {
            Self::Null => 0,
            Self::Phdr => 3,
            Self::Phent => 4,
            Self::Phnum => 5,
            Self::Pagesz => 6,
            Self::Base => 7,
            Self::Flags => 8,
            Self::Entry => 9,
            Self::Uid => 11,
            Self::Euid => 12,
            Self::Gid => 13,
            Self::Egid => 14,
            Self::Platform => 15,
            Self::Hwcap => 16,
            Self::Clktck => 17,
            Self::Random => 25,
            Self::ExecFn => 31,
            Self::Undefined => 33,
        }
    }
}

impl Auxiliar {
    pub fn new(atype: T, value: T) -> Self {
        Self {
            atype: atype as u64,
            value,
        }
    }

    pub fn null() -> Self {
        Self::new(AuxiliarType::Null.to(), 0)
    }
}

impl core::fmt::Debug for Auxiliar {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let type_name = match self.atype {
            0 => "Null",
            3 => "Phdr",
            4 => "Phent",
            5 => "Phnum",
            6 => "Pagesz",
            7 => "Base",
            8 => "Flags",
            9 => "Entry",
            11 => "Uid",
            12 => "Euid",
            13 => "Gid",
            14 => "Egid",
            15 => "Platform",
            16 => "Hwcap",
            17 => "Clktck",
            25 => "Random",
            31 => "ExecFn",
            _ => "Unknown",
        };
        write!(
            f,
            "Auxv{{ type: {} ({}), value: 0x{:x} }}",
            self.atype, type_name, self.value
        )
    }
}

#[repr(C)]
pub struct InitialStack {
    // The stack grows downward, but we'll build it bottom-up
    auxv: Vec<Auxiliar>,  // Auxiliary vector entries, null-terminated
    envp: Vec<*const u8>, // Environment pointers, null-terminated
    argv: Vec<*const u8>, // Argument pointers, null-terminated
    argc: u64,            // Number of arguments
}

impl InitialStack {
    pub fn new(program_path: &str, args: &[&str], envs: &[&str]) -> Self {
        let mut stack = Self {
            auxv: Vec::new(),
            envp: Vec::new(),
            argv: Vec::new(),
            argc: (args.len() + 1) as u64, // +1 for program name
        };

        // Add program name and arguments
        stack.argv.push(program_path.as_ptr());
        for arg in args {
            stack.argv.push(arg.as_ptr());
        }
        stack.argv.push(std::ptr::null()); // Null terminator

        // Add environment variables
        for env in envs {
            stack.envp.push(env.as_ptr());
        }
        stack.envp.push(std::ptr::null()); // Null terminator

        stack
    }

    pub fn add_auxv(&mut self, atype: T, value: T) {
        self.auxv.push(Auxiliar::new(atype, value));
    }

    pub fn finalize(&mut self) {
        self.auxv.push(Auxiliar::null()); // Add null terminator
    }

    // Calculate total size needed on stack
    pub fn size(&self) -> usize {
        8 + // argc
        (self.argv.len() * 8) + // argv pointers
        (self.envp.len() * 8) + // envp pointers
        (self.auxv.len() * 16) // auxv entries (each 16 bytes)
    }

    // Write the stack layout to a given memory address
    pub unsafe fn write_to(&self, stack_top: *mut u8) -> *mut u8 {
        let mut current = stack_top;

        // Write argc
        let argc_ptr = current as *mut u64;
        *argc_ptr = self.argc;
        current = current.add(8);

        // Write argv
        for arg in &self.argv {
            *(current as *mut *const u8) = *arg;
            current = current.add(8);
        }

        // Write envp
        for env in &self.envp {
            *(current as *mut *const u8) = *env;
            current = current.add(8);
        }

        // Write auxv
        for aux in &self.auxv {
            *(current as *mut Auxiliar) = *aux;
            current = current.add(16);
        }

        current
    }
}

pub unsafe fn prepare_stack() -> *mut u8 {
    // Get current stack pointer
    let mut stack_ptr: *mut u8;
    core::arch::asm!("mov {}, rsp", out(reg) stack_ptr);

    // Align stack to 16 bytes
    stack_ptr = ((stack_ptr as usize) & !15) as *mut u8;

    // Now we can modify the existing stack
    // Clear/setup the registers as per System V ABI
    core::arch::asm!(
        "xor rdi, rdi", // Clear argc
        "xor rsi, rsi", // Clear argv
        "xor rdx, rdx", // Clear envp
    );

    stack_ptr
}

// Usage example:
pub fn setup_initial_stack(elf_header: &crate::ELFHeader) -> *mut u8 {
    let stack = unsafe { prepare_stack() };

    // Calculate stack top (stack grows down)
    let stack_top = (stack as *mut u8).wrapping_add(super::PAGE_SIZE as usize);

    // Create and populate initial stack
    let mut initial_stack = InitialStack::new(
        "/proc/self/exe",
        &[],
        &["PATH=/usr/local/bin:/usr/bin:/bin"],
    );

    // Add auxiliary vectors
    initial_stack.add_auxv(AuxiliarType::Phdr.to(), elf_header.phoff);
    initial_stack.add_auxv(AuxiliarType::Phent.to(), elf_header.phentsize as u64);
    initial_stack.add_auxv(AuxiliarType::Phnum.to(), elf_header.phnum as u64);
    initial_stack.add_auxv(AuxiliarType::Pagesz.to(), 4096);
    initial_stack.add_auxv(AuxiliarType::Entry.to(), elf_header.entry);
    // Add more auxv entries as needed...

    initial_stack.finalize();

    // Write stack contents
    unsafe { initial_stack.write_to(stack_top.wrapping_sub(initial_stack.size() as usize)) }
}
