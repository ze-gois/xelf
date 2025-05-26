#![no_std]

/// A trait that standardizes error handling across the xelf crates
///
/// This trait allows for consistent error handling patterns by providing:
/// - Error creation from errno values
/// - Error descriptions
/// - Optional advertising of error codes for syscall returns
pub trait ErrorTrait {
    /// Creates an error instance from a numeric error code
    fn from_no(errno: isize) -> Self;

    /// Returns a human-readable description of the error
    fn describe(&self) -> &str;

    /// Returns the numeric error code this error should advertise
    /// to syscalls or None if no specific code should be used
    fn advert(&self) -> Option<isize>;
}
