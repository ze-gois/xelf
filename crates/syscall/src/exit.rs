use super::Number;
use arch::{Arch, Callable};

pub fn exit(status_code: i32) -> ! {
    let number = Number::Exit as usize;
    let status_code = status_code as usize;

    unsafe {
        let _ = Arch::syscall1(number, status_code);
        core::hint::unreachable_unchecked()
    }
}
