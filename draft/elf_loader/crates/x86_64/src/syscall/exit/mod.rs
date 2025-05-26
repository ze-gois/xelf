use crate::syscall::syscall1;
use crate::Syscall;

#[inline(always)]
pub unsafe fn exit(status: i32) -> ! {
    syscall1(Syscall::Exit.to(), status as usize);
    // We use unreachable!() because exit() never returns
    unreachable!()
}
