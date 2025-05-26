pub mod auxv;

use core::arch::x86_64;

pub use auxv::AuxEntry;
pub use auxv::AuxVec;

#[repr(C)]
pub struct Layout {
    pub pointer: *mut u64,
    pub argc: *mut u64,
    pub argv: *mut *mut u8,
    pub envp: *mut *mut u8,
    pub auxv: *mut AuxEntry,
}

impl Layout {
    pub unsafe fn from_stack_pointer(stack_pointer: *mut u64) -> Self {
        let argc = stack_pointer;
        let argv = (stack_pointer.add(1)) as *mut *mut u8;
        let envp = argv.add(*argc as usize + 1);

        crate::print_str("EnvP: ");

        let mut envp_pointer = envp;
        while !(*envp_pointer).is_null() {
            envp_pointer = envp_pointer.add(1);
        }

        let auxv = envp_pointer.add(1) as *mut AuxEntry;

        Self {
            pointer: stack_pointer,
            argc,
            argv,
            envp,
            auxv,
        }
    }

    pub unsafe fn from_current_stack_pointer() -> Self {
        let mut stack_pointer: *mut u64;
        core::arch::asm!("mov {}, rsp", out(reg) stack_pointer);
        Self::from_stack_pointer(stack_pointer)
    }

    pub unsafe fn print_args(&self) {
        for i in 0..*self.argc {
            crate::print::print_str("\tArgument #");
            crate::print::print_dec(i as u64);
            crate::print::print_str(": \"");
            let arg = core::ptr::read(self.argv.offset(i as isize));
            let len = crate::memory::length(arg);
            crate::syscall::write(1, arg, len);
            crate::print::print_str("\"\n");
        }
    }

    pub unsafe fn print(&self) {
        crate::print::print_str("argc: ");
        crate::print::print_dec(*self.argc as u64);
        crate::print::print_str("\n");

        crate::print::print_str("argv at: ");
        crate::print::print_hex(self.argv as u64);
        crate::print::print_str("\n");

        self.print_args();

        crate::print::print_str("envp at: ");
        crate::print::print_hex(self.envp as u64);
        crate::print::print_str("\n");
        let mut envp_pointer = self.envp;
        let mut envc = 0;
        while !envp_pointer.is_null() {
            envc += 1;
            crate::print::print_str("\tEnv: '");
            if !(*envp_pointer).is_null() {
                crate::print::print(*envp_pointer);
            } else {
                crate::print::print_str("NULL'\n");
                break;
            }
            crate::print::print_str("'\n");
            envp_pointer = envp_pointer.add(1);
        }
        crate::print_str("\n=======\nEnvpCount=");
        crate::print_dec(envc as u64);
        crate::print_str(";\n=======\n");

        crate::print::print_str("auxv at: ");
        crate::print::print_hex(self.auxv as u64);
        crate::print::print_str("\n");
        let mut av = 0;
        while !self.auxv.offset(av).is_null() && (*self.auxv.offset(av)).atype != 0 {
            let auxv_entry = *self.auxv.offset(av);
            crate::print::print_str("\tAuxv: ");
            crate::print::print_str(auxv::Type::from(auxv_entry.atype).as_str());
            crate::print::print_str(" (");
            crate::print::print_dec(auxv_entry.atype as u64);
            crate::print::print_str(")");
            crate::print::print_str(" = '");
            if !(auxv_entry.value as *const u8).is_null() {
                match auxv_entry.atype {
                    31 => crate::print(auxv_entry.value as *mut u8),
                    _ => crate::print::print_dec(auxv_entry.value as u64),
                }
            } else {
                crate::print::print_str("NULL");
            }
            crate::print::print_str("'\n");
            av += 1;
        }
        crate::print_str("\n=======\nAuxvCount=");
        crate::print_dec(av as u64);
        crate::print_str(";\n=======\n");

        // AuxVec::new(self.auxv).print();
    }
}
