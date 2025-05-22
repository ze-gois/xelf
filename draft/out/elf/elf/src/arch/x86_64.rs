pub mod start;
pub mod syscall;
pub mod trampoline;

pub use start::startaa;
pub use syscall::*;
pub use trampoline::trampoline;

pub use super::{Arch, Archable};

// impl Archable for Arch {
//     unsafe fn start() -> ! {
//         unsafe { _start() }
//     }

//     unsafe fn trampoline(entry: extern "C" fn(), sp: *mut usize, fini: extern "C" fn()) -> ! {
//         unsafe {
//             trampoline(entry as usize, sp as usize, fini as usize);
//         }
//     }
// }
