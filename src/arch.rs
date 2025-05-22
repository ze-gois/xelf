#[cfg(target_arch = "x86_64")]
pub use x86_64 as arch;
#[cfg(target_arch = "x86_64")]
pub mod _x86_64;
#[cfg(target_arch = "x86_64")]
pub use _x86_64 as _arch;

pub mod error;
pub mod macros;

// pub use _arch::*;
pub use error::*;

pub struct Arch;

type C = usize;

pub trait Callable {
    fn syscall0(n: C) -> C;
    fn syscall1(n: C, a1: C) -> C;
    fn syscall2(n: C, a1: C, a2: C) -> C;
    fn syscall3(n: C, a1: C, a2: C, a3: C) -> C;
    fn syscall4(n: C, a1: C, a2: C, a3: C, a4: C) -> C;
    fn syscall5(n: C, a1: C, a2: C, a3: C, a4: C, a5: C) -> C;
    fn syscall6(n: C, a1: C, a2: C, a3: C, a4: C, a5: C, a6: C) -> C;

    crate::wrap_syscall!(wrapped_syscall1, syscall1, a1: usize);
    crate::wrap_syscall!(wrapped_syscall2, syscall2, a1: usize, a2: usize);
    crate::wrap_syscall!(wrapped_syscall3, syscall3, a1: usize, a2: usize, a3: usize);
    crate::wrap_syscall!(wrapped_syscall4, syscall4, a1: usize, a2: usize, a3: usize, a4: usize);
    crate::wrap_syscall!(wrapped_syscall5, syscall5, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize);
    crate::wrap_syscall!(wrapped_syscall6, syscall6, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize, a6: usize);
}

pub trait Stacking {}
