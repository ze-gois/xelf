use super::Arch;
use super::Callable;
use super::arch;

impl Callable for Arch {
    fn syscall0(n: usize) -> usize {
        arch::syscall0(n)
    }

    fn syscall1(n: usize, a1: usize) -> usize {
        arch::syscall1(n, a1)
    }

    fn syscall2(n: usize, a1: usize, a2: usize) -> usize {
        arch::syscall2(n, a1, a2)
    }

    fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> usize {
        arch::syscall3(n, a1, a2, a3)
    }

    fn syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> usize {
        arch::syscall4(n, a1, a2, a3, a4)
    }

    fn syscall5(n: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> usize {
        arch::syscall5(n, a1, a2, a3, a4, a5)
    }

    fn syscall6(
        n: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
        a6: usize,
    ) -> usize {
        arch::syscall6(n, a1, a2, a3, a4, a5, a6)
    }
}
