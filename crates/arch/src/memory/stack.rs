pub mod atype;
pub mod auxv;

pub use atype::*;
pub use auxv::*;

use human::info;

#[repr(C)]
pub struct Stack {
    pub pointer: *mut u64,
    pub argc: usize,
    pub argv: *mut *mut u8,
    pub envp: *mut *mut u8,
    pub envc: usize,
    pub auxv: *mut AuxEntry,
    pub auxc: usize,
    saved_pointer: *mut u64,
}

impl Stack {
    /// Create a new Stack instance from a raw stack pointer.
    /// This function is unsafe because it dereferences raw pointers.
    pub unsafe fn from_pointer(stack_pointer: *mut u64) -> Self {
        // Using unsafe blocks for each unsafe operation
        let argc = unsafe { *stack_pointer };
        let argv = unsafe { (stack_pointer.add(1)) as *mut *mut u8 };
        let envp = unsafe { argv.add(argc as usize + 1) };

        let mut envp_pointer = envp;
        while unsafe { !(*envp_pointer).is_null() } {
            envp_pointer = unsafe { envp_pointer.add(1) };
        }

        let auxv = unsafe { envp_pointer.add(1) as *mut AuxEntry };

        // Count auxiliary vector entries
        let mut auxc = 0;
        let mut auxv_ptr = auxv;
        while unsafe { !auxv_ptr.is_null() && ((*auxv_ptr).atype != 0 || (*auxv_ptr).value != 0) } {
            auxc += 1;
            auxv_ptr = unsafe { auxv_ptr.add(1) };
        }

        Self {
            pointer: stack_pointer,
            argc: argc as usize,
            argv,
            envp,
            envc: unsafe { envp_pointer.offset_from(envp) as usize },
            auxv,
            auxc,
            saved_pointer: stack_pointer,
        }
    }

    /// Get the current stack layout by reading the current stack pointer.
    /// This function is unsafe because it directly reads from the CPU stack.
    pub unsafe fn current() -> Self {
        let mut stack_pointer: *mut u64;
        unsafe { core::arch::asm!("mov {}, rsp", out(reg) stack_pointer) };
        
        // We need to adjust the stack pointer to find the real program arguments
        // This requires walking up the stack frames until we find something that looks
        // like the program entry point stack layout
        unsafe { Self::from_pointer(stack_pointer) }
    }

    /// Get a specific argument string from the stack
    pub unsafe fn get_arg(&self, index: usize) -> Option<&'static str> {
        if index >= self.argc {
            return None;
        }
        
        let arg = unsafe { core::ptr::read(self.argv.offset(index as isize)) };
        let len = unsafe { super::misc::length(arg) };
        let s = unsafe { core::slice::from_raw_parts(arg, len) };
        
        core::str::from_utf8(s).ok()
    }
    
    /// Print all command line arguments
    pub fn print_args(&self) {
        info!("Arguments {{");
        for i in 0..self.argc {
            info!("\n\tArgument #{i}: \"");
            let arg = unsafe { core::ptr::read(self.argv.offset(i as isize)) };
            let len = unsafe { super::misc::length(arg) };

            // Print the entire string
            let s = unsafe { core::slice::from_raw_parts(arg, len) };
            if let Ok(arg_str) = core::str::from_utf8(s) {
                info!("{}\"", arg_str);
            } else {
                info!("<invalid UTF-8>\"");
            }
        }
        info!("\n}} Arguments \n");
    }

    /// Get a specific environment variable string from the stack
    pub unsafe fn get_env(&self, index: usize) -> Option<&'static str> {
        if index >= self.envc {
            return None;
        }
        
        let mut envp_pointer = self.envp;
        let mut current_index = 0;
        
        while !envp_pointer.is_null() {
            if unsafe { (*envp_pointer).is_null() } {
                break;
            }
            
            if current_index == index {
                let env_ptr = unsafe { *envp_pointer };
                let len = unsafe { super::misc::length(env_ptr) };
                let s = unsafe { core::slice::from_raw_parts(env_ptr, len) };
                return core::str::from_utf8(s).ok();
            }
            
            current_index += 1;
            envp_pointer = unsafe { envp_pointer.add(1) };
        }
        
        None
    }
    
    /// Get an environment variable by name
    pub unsafe fn get_env_by_name(&self, name: &str) -> Option<&'static str> {
        let mut envp_pointer = self.envp;
        
        while !envp_pointer.is_null() {
            if unsafe { (*envp_pointer).is_null() } {
                break;
            }
            
            let env_ptr = unsafe { *envp_pointer };
            let len = unsafe { super::misc::length(env_ptr) };
            let s = unsafe { core::slice::from_raw_parts(env_ptr, len) };
            
            if let Ok(env_str) = core::str::from_utf8(s) {
                if let Some((key, value)) = env_str.split_once('=') {
                    if key == name {
                        return Some(value);
                    }
                }
            }
            
            envp_pointer = unsafe { envp_pointer.add(1) };
        }
        
        None
    }
    
    /// Print all environment variables
    pub fn print_env(&self) {
        info!("Environment {{");
        let mut envp_pointer = self.envp;
        let mut envc = 0;
        while !envp_pointer.is_null() {
            if (unsafe { *envp_pointer }).is_null() {
                break;
            }

            envc += 1;
            info!("\n\tEnv: '");
            let env_ptr = unsafe { *envp_pointer };
            let len = unsafe { super::misc::length(env_ptr) };

            // Print the actual environment variable string
            let s = unsafe { core::slice::from_raw_parts(env_ptr, len) };
            if let Ok(env_str) = core::str::from_utf8(s) {
                info!("{}", env_str);
            } else {
                info!("<invalid UTF-8>");
            }
            info!("'");

            envp_pointer = unsafe { envp_pointer.add(1) };
        }
        info!("\n}} Environment \n");
    }

    pub fn print_auxv(&self) {
        info!("Auxiliary Vector {{");
        let mut auxv_pointer = self.auxv;
        let mut auxv_count = 0;

        while !auxv_pointer.is_null() {
            let auxv_entry = unsafe { *auxv_pointer };

            if auxv_entry.atype == 0 && auxv_entry.value == 0 {
                // Found AT_NULL which marks the end of auxv
                break;
            }

            auxv_count += 1;
            info!("\n\tAux: ");

            // Get type as string
            let atype_str = atype::Type::from(auxv_entry.atype).as_str();
            info!("{} ({})", atype_str, auxv_entry.atype);

            // Print value based on type
            info!(" = '");
            if auxv_entry.atype == 15 || auxv_entry.atype == 31 {
                // AT_PLATFORM or AT_EXECFN
                let ptr = auxv_entry.value as *const u8;
                if !ptr.is_null() {
                    let len = unsafe { super::misc::length(ptr) };
                    let s = unsafe { core::slice::from_raw_parts(ptr, len) };
                    if let Ok(str_value) = core::str::from_utf8(s) {
                        info!("{}", str_value);
                    } else {
                        info!("<invalid UTF-8>");
                    }
                } else {
                    info!("NULL");
                }
            } else {
                info!("{:#x}", auxv_entry.value);
            }
            info!("'");

            auxv_pointer = unsafe { auxv_pointer.add(1) };
        }

        info!("\n}} Auxiliary Vector \n");
    }

    /// Get a specific auxiliary vector entry by type
    pub unsafe fn get_auxv_by_type(&self, atype: usize) -> Option<usize> {
        let mut auxv_pointer = self.auxv;
        
        while !auxv_pointer.is_null() {
            let auxv_entry = unsafe { *auxv_pointer };
            
            if auxv_entry.atype == 0 && auxv_entry.value == 0 {
                break;
            }
            
            if auxv_entry.atype == atype {
                return Some(auxv_entry.value);
            }
            
            auxv_pointer = unsafe { auxv_pointer.add(1) };
        }
        
        None
    }
    
    /// Set a specific auxiliary vector entry value
    pub unsafe fn set_auxv_by_type(&mut self, atype: usize, value: usize) -> bool {
        let mut auxv_pointer = self.auxv;
        
        while !auxv_pointer.is_null() {
            let auxv_entry_ptr = auxv_pointer;
            let auxv_entry = unsafe { *auxv_entry_ptr };
            
            if auxv_entry.atype == 0 && auxv_entry.value == 0 {
                break;
            }
            
            if auxv_entry.atype == atype {
                unsafe { (*auxv_pointer).value = value };
                return true;
            }
            
            auxv_pointer = unsafe { auxv_pointer.add(1) };
        }
        
        false
    }
    
    /// Print full stack contents (arguments, environment variables, and auxiliary vector)
    pub fn print(&self) {
        self.print_args();
        self.print_env();
        self.print_auxv();
    }
}
