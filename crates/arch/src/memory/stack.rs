pub mod atype;
pub mod auxv;

pub use atype::*;
pub use auxv::*;

use human::info;

#[repr(C)]
pub struct Layout {
    pub pointer: *mut u64,
    pub argc: usize,
    pub argv: *mut *mut u8,
    pub envp: *mut *mut u8,
    pub envc: usize,
    pub auxv: *mut AuxEntry,
    pub auxc: usize,
    saved_pointer: *mut u64,
}

impl Layout {
    pub fn from_pointer(stack_pointer: *mut u64) -> Self {
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
            argv: argv,
            envp: envp,
            envc: unsafe { envp_pointer.offset_from(envp) as usize },
            auxv: auxv,
            auxc: auxc,
            saved_pointer: stack_pointer,
        }
    }

    pub fn from_current_pointer() -> Self {
        let mut stack_pointer: *mut u64;
        unsafe { core::arch::asm!("mov {}, rsp", out(reg) stack_pointer) };
        Self::from_pointer(stack_pointer)
    }

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
            if auxv_entry.atype == 15 || auxv_entry.atype == 31 { // AT_PLATFORM or AT_EXECFN
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

    pub fn print(&self) {
        self.print_args();
        self.print_env();
        self.print_auxv();
    }
}