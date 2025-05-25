pub struct Stdout;

fn print(msg: &str) -> crate::Result<isize> {
    crate::arch::write(1, msg.as_ptr(), msg.len())
}

impl core::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let _ = print(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let mut writer = $crate::stdout::Stdout;
        let _ = write!(&mut writer, $($arg)*);
    }};
}
