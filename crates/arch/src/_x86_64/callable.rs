use crate::*;

impl Callable for Arch {
    fn syscall0(n: usize) -> Result<isize> {
        let syscall_result = syscall0(n);
        syscall_result
    }

    fn _syscall1(n: usize, a1: usize) -> Result<isize> {
        let syscall_result = syscall1(n, a1);
        syscall_result
    }

    fn _syscall2(n: usize, a1: usize, a2: usize) -> Result<isize> {
        let syscall_result = syscall2(n, a1, a2);
        syscall_result
    }

    fn _syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> Result<isize> {
        let syscall_result = syscall3(n, a1, a2, a3);
        syscall_result
    }

    fn _syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> Result<isize> {
        let syscall_result = syscall4(n, a1, a2, a3, a4);
        syscall_result
    }

    fn _syscall5(n: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> Result<isize> {
        let syscall_result = syscall5(n, a1, a2, a3, a4, a5);
        syscall_result
    }

    fn _syscall6(
        n: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
        a6: usize,
    ) -> Result<isize> {
        let syscall_result = syscall6(n, a1, a2, a3, a4, a5, a6);
        syscall_result
    }
}
