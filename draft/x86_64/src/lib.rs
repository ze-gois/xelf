#![no_std]

mod syscall;

pub use syscall::syscall0;
pub use syscall::syscall1;
pub use syscall::syscall2;
pub use syscall::syscall3;
pub use syscall::syscall4;
pub use syscall::syscall5;
pub use syscall::syscall6;

// mod stack;
