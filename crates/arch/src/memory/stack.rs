mod atype;
mod auxv;

use auxv::AuxEntry;

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
        let argc = stack_pointer;
        let argv = unsafe { (stack_pointer.add(1)) as *mut *mut u8 };
        let envp = unsafe { argv.add(*argc as usize + 1) };

        let mut envp_pointer = envp;
        while !unsafe { (*envp_pointer).is_null() } {
            envp_pointer = unsafe { envp_pointer.add(1) };
        }

        let auxv = unsafe { envp_pointer.add(1) as *mut AuxEntry };

        Self {
            pointer: stack_pointer,
            argc: unsafe { *argc } as usize,
            argv: argv,
            envp: envp,
            envc: envp_pointer as usize - envp as usize,
            auxv: auxv,
            auxc: auxv as usize - envp_pointer as usize,
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
            info!("{}\"", unsafe { *arg });
        }
        info!("\n}} Arguments \n");
    }

    pub fn print_env(&self) {
        info!("Environment {{");
        let mut envp_pointer = self.envp;
        let mut envc = 0;
        while !envp_pointer.is_null() {
            envc += 1;
            info!("\n\tEnv: '");
            if !(unsafe { *envp_pointer }).is_null() {
                info!("{:?}", unsafe { *envp_pointer });
            } else {
                info!("NULL'");
                break;
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
            auxv_count += 1;
            info!("\n\tAux: '");
            if !(unsafe { *auxv_pointer }).is_null() {
                info!("{:?}", unsafe { *auxv_pointer });
            } else {
                info!("NULL'");
                break;
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
// crate::print::print_str("envp at: ");
// crate::print::print_hex(self.envp as u64);
// crate::print::print_str("\n");
// let mut envp_pointer = self.envp;
// let mut envc = 0;
// while !envp_pointer.is_null() {
//     envc += 1;
//     crate::print::print_str("\tEnv: '");
//     if !(*envp_pointer).is_null() {
//         crate::print::print(*envp_pointer);
//     } else {
//         crate::print::print_str("NULL'\n");
//         break;
//     }
//     crate::print::print_str("'\n");
//     envp_pointer = envp_pointer.add(1);
// }
// crate::print_str("\n=======\nEnvpCount=");
// crate::print_dec(envc as u64);
// crate::print_str(";\n=======\n");

// AuxVec::new(self.auxv).print();

// crate::print::print_str("auxv at: ");
// crate::print::print_hex(self.auxv as u64);
// crate::print::print_str("\n");
// let mut av = 0;
// while !self.auxv.offset(av).is_null() && (*self.auxv.offset(av)).atype != 0 {
//     let auxv_entry = *self.auxv.offset(av);
//     crate::print::print_str("\tAuxv: ");
//     crate::print::print_str(auxv::Type::from(auxv_entry.atype).as_str());
//     crate::print::print_str(" (");
//     crate::print::print_dec(auxv_entry.atype as u64);
//     crate::print::print_str(")");
//     crate::print::print_str(" = '");
//     if !(auxv_entry.value as *const u8).is_null() {
//         match auxv_entry.atype {
//             31 => crate::print(auxv_entry.value as *mut u8),
//             _ => crate::print::print_dec(auxv_entry.value as u64),
//         }
//     } else {
//         crate::print::print_str("NULL");
//     }
//     crate::print::print_str("'\n");
//     av += 1;
// }
// crate::print_str("\n=======\nAuxvCount=");
// crate::print_dec(av as u64);
// crate::print_str(";\n=======\n");
