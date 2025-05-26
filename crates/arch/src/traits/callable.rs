type C = usize;

pub trait Callable {
    fn syscall0(n: C) -> crate::Result<isize>;
    fn _syscall1(n: C, a1: C) -> crate::Result<isize>;
    fn _syscall2(n: C, a1: C, a2: C) -> crate::Result<isize>;
    fn _syscall3(n: C, a1: C, a2: C, a3: C) -> crate::Result<isize>;
    fn _syscall4(n: C, a1: C, a2: C, a3: C, a4: C) -> crate::Result<isize>;
    fn _syscall5(n: C, a1: C, a2: C, a3: C, a4: C, a5: C) -> crate::Result<isize>;
    fn _syscall6(n: C, a1: C, a2: C, a3: C, a4: C, a5: C, a6: C) -> crate::Result<isize>;

    crate::wrap_syscall!(syscall1, _syscall1, a1: C);
    crate::wrap_syscall!(syscall2, _syscall2, a1: C, a2: C);
    crate::wrap_syscall!(syscall3, _syscall3, a1: C, a2: C, a3: C);
    crate::wrap_syscall!(syscall4, _syscall4, a1: C, a2: C, a3: C, a4: C);
    crate::wrap_syscall!(syscall5, _syscall5, a1: C, a2: C, a3: C, a4: C, a5: C);
    crate::wrap_syscall!(syscall6, _syscall6, a1: C, a2: C, a3: C, a4: C, a5: C, a6: C);
}
