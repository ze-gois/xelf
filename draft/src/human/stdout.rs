pub struct Stdout;

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        use ::core::fmt::Write;
        let _ = ::core::write!(&mut $crate::human::Stdout, $($arg)*);
    }};
}

impl core::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.as_bytes() {
            match crate::syscall::write(1, b, 1) {
                Ok(_) => (),
                Err(_e) => return Err(core::fmt::Error),
            }
        }
        Ok(())
    }
}

pub fn print(msg: &str) -> crate::Result<isize> {
    crate::syscall::write(1, msg.as_ptr(), msg.len())
}

pub fn wrapped_print(msg: &str) -> crate::Result<isize> {
    crate::syscall::wrapped_write(1, msg.as_ptr(), msg.len())
}
