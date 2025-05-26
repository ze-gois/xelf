#[macro_export]
macro_rules! wrap_syscall {
    ($name:ident, $syscall:ident, $($arg:ident : $type:ty),*) => {
        fn $name(n: usize, $($arg: $type),*) -> $crate::Result<isize> {
            let return_value = $crate::Arch::$syscall(n, $($arg),*);
            return_value
        }
    }
}
