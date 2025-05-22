pub fn print(msg: &str) -> crate::Result<isize> {
    crate::syscall::write(1, msg.as_ptr(), msg.len())
}

impl core::fmt::Write for super::Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        print(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let mut writer = $crate::human::Stdout;
        let _ = write!(&mut writer, $($arg)*);
    }};
}

pub fn wrapped_print(msg: &str) -> crate::Result<isize> {
    crate::syscall::wrapped_write(1, msg.as_ptr(), msg.len())
}

impl core::fmt::Write for super::WrappedStdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        wrapped_print(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! wrapped_info {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let mut writer = $crate::human::WrappedStdout;
        let _ = write!(&mut writer, $($arg)*);
    }};
}
