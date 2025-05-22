#[macro_export]
macro_rules! impl_partial_eq_for_type {
    ($name:ty, $inner:ty, $($t:ty),*) => {
        $(
            impl core::cmp::PartialEq<$t> for $name {
                fn eq(&self, other: &$t) -> bool {
                    if (*other as u128) <= <$inner>::MAX as u128 {
                        self.0 == *other as $inner
                    } else {
                        false
                    }
                }
            }

            impl core::cmp::PartialEq<$name> for $t {
                fn eq(&self, other: &$name) -> bool {
                    if (*self as u128) <= <$inner>::MAX as u128 {
                        *self as $inner == other.0
                    } else {
                        false
                    }
                }
            }

            impl core::cmp::PartialOrd<$t> for $name {
                fn partial_cmp(&self, other: &$t) -> Option<core::cmp::Ordering> {
                    if (*other as u128) <= <$inner>::MAX as u128 {
                        Some(self.0.cmp(&(*other as $inner)))
                    } else {
                        None
                    }
                }
            }

            impl core::cmp::PartialOrd<$name> for $t {
                fn partial_cmp(&self, other: &$name) -> Option<core::cmp::Ordering> {
                    if (*self as u128) <= <$inner>::MAX as u128 {
                        Some((*self as $inner).cmp(&other.0))
                    } else {
                        None
                    }
                }
            }
        )*
    }
}

#[macro_export]
macro_rules! impl_from_for_type {
    ($name:ty, $inner:ty, $($t:ty),*) => {
        $(
            impl From<$name> for $t {
                fn from(val: $name) -> $t {
                    val.0 as $t
                }
            }
        )*
    }
}

// #[macro_export]
// macro_rules! impl_into_for_type {
//     ($name:ty, $inner:ty, $($t:ty),*) => {
//         $(
//             impl Into<$name> for $t {
//                 fn into(val: $name) -> $t {
//                     val.0 as $t
//                 }
//             }
//         )*
//     }
// }

#[macro_export]
macro_rules! define_type {
    ($(#[$meta:meta])* pub $name:ident, $inner:ty) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct $name(pub $inner);

        impl $crate::dtype::ELFType for $name {
            const SIZE_BYTES: usize = core::mem::size_of::<$inner>();
            const SIZE_BITS: usize = Self::SIZE_BYTES * 8;
        }

        impl $name {
            pub fn new(value: $inner) -> Self {
                Self(value)
            }

            pub fn from_bytes(bytes: [u8; Self::SIZE_BYTES]) -> Self {
                match unsafe { *ENDIANNESS.assume_init_ref() } {
                    Endianness::Little => Self(<$inner>::from_le_bytes(bytes)),
                    Endianness::Big => Self(<$inner>::from_be_bytes(bytes)),
                }
            }

            pub fn to_bytes(&self) -> [u8; Self::SIZE_BYTES] {
                match unsafe { *ENDIANNESS.assume_init_ref() } {
                    Endianness::Little => self.0.to_le_bytes(),
                    Endianness::Big => self.0.to_be_bytes(),
                }
            }

            pub unsafe fn read(fd: i32) -> Result<$name> {
                let mut bytes = [0u8; <$name>::SIZE_BYTES];
                match syscall::read(fd, bytes.as_mut_ptr(), <$name>::SIZE_BYTES as usize) {
                    Ok(<$name>::SIZE_BYTES) => {
                        let value = match *ENDIANNESS.assume_init_ref() {
                            Endianness::Little => <$inner>::from_le_bytes(bytes),
                            Endianness::Big => <$inner>::from_be_bytes(bytes),
                        };
                        Ok($name::from(value))
                    },
                    Ok(_) => Err(Error::ShorterData),
                    Err(e) => Err(Error::InvalidData),
                }
            }

            pub unsafe fn print_hex(&self) {
                let bytes = self.to_bytes();
                let mut buf = [0u8; 2 + (Self::SIZE_BYTES * 2)];
                buf[0] = b'0';
                buf[1] = b'x';

                for (i, byte) in bytes.iter().rev().enumerate() {
                    let hex = $crate::macros::format_hex(*byte);
                    buf[2 + i*2] = hex[0];
                    buf[2 + i*2 + 1] = hex[1];
                }

                x86_64::syscall::write(1, buf.as_ptr(), buf.len()).unwrap();
            }

            pub unsafe fn print_dec(&self) {
                let mut n = self.0;
                let mut buf = [0u8; 20];  // Max digits for 64-bit number
                let mut i = buf.len();

                if n == 0 {
                    x86_64::syscall::write(1, b"0".as_ptr(), 1).unwrap();
                    return;
                }

                while n > 0 {
                    i -= 1;
                    buf[i] = b'0' + (n % 10) as u8;
                    n /= 10;
                }

                x86_64::syscall::write(1, buf.as_ptr().add(i), buf.len() - i).unwrap();
            }

            pub unsafe fn print_oct(&self) {
                let mut n = self.0;
                let mut buf = [0u8; 2 + 22];  // "0o" + max octal digits
                buf[0] = b'0';
                buf[1] = b'o';
                let mut i = buf.len();

                if n == 0 {
                    x86_64::syscall::write(1, b"0o0".as_ptr(), 3).unwrap();
                    return;
                }

                while n > 0 {
                    i -= 1;
                    buf[i] = b'0' + (n & 0o7) as u8;
                    n >>= 3;
                }

                x86_64::syscall::write(1, buf.as_ptr().add(i-2), buf.len() - i + 2).unwrap();
            }

            // Default print method
            pub unsafe fn print(&self) {
                self.print_dec()
            }

        }

        impl Default for $name {
            fn default() -> Self {
                Self(0)
            }
        }

        impl From<$inner> for $name {
            fn from(value: $inner) -> Self {
                Self(value)
            }
        }

        crate::impl_partial_eq_for_type!($name, $inner, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);
        crate::impl_from_for_type!($name, $inner, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);
        // crate::impl_into_for_type!($name, $inner, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
    }
}

pub const HEX_CHARS: &[u8] = b"0123456789abcdef";

#[inline]
pub fn format_hex(byte: u8) -> [u8; 2] {
    [
        HEX_CHARS[(byte >> 4) as usize],
        HEX_CHARS[(byte & 0xf) as usize],
    ]
}
