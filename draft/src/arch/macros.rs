#[macro_export]
macro_rules! wrap_syscall {
    ($name:ident, $syscall:ident, $($arg:ident : $type:ty),*) => {
        fn $name(n: usize, $($arg: $type),*) -> crate::Result<isize> {
            crate::human::print("Preparint to invoke:\n")?;
            let return_value = Arch::$syscall(n, $($arg),*);
            crate::human::print("complete.")?;
            let result = handle_return(return_value);
            crate::human::print("\n")?;
            result
        }
    }
}
