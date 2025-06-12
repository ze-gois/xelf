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
macro_rules! elf_define_type {
    ($(#[$meta:meta])* pub $name:ident, $inner:ty) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct $name(pub $inner);

        impl $crate::dtype::ELFType for $name {
            type Inner = $inner;
            const SIZE_BYTES: usize = core::mem::size_of::<$inner>();
            const SIZE_BITS: usize = Self::SIZE_BYTES * 8;
        }

        impl $name {
            pub fn new(value: $inner) -> Self {
                Self(value)
            }

            pub fn from_bytes(bytes: [u8; Self::SIZE_BYTES], endianness: $crate::dtype::Endianness) -> Self {
                match endianness {
                    Endianness::None => {
                        $crate::info!("Endianness is zeroed.");
                        panic!("none.")
                    },
                    Endianness::LSB => Self(<$inner>::from_le_bytes(bytes)),
                    Endianness::MSB => Self(<$inner>::from_be_bytes(bytes)),
                    Endianness::Number => {
                        $crate::info!("Endianness is invalid number.");
                        panic!("none.")
                    },
                    Endianness::Undefined => {
                        $crate::info!("Endianness is undefined.");
                        panic!("none.")
                    },
                }
            }

            pub fn to_bytes(&self, endianness: $crate::dtype::Endianness) -> [u8; Self::SIZE_BYTES] {
                match endianness {
                    Endianness::None => {
                        $crate::info!("Endianness is zeroed.");
                        panic!("none.")
                    },
                    Endianness::LSB => self.0.to_le_bytes(),
                    Endianness::MSB => self.0.to_be_bytes(),
                    Endianness::Number => {
                        $crate::info!("Endianness is invalid number.");
                        panic!("none.")
                    },
                    Endianness::Undefined => {
                        $crate::info!("Endianness is undefined.");
                        panic!("none.")
                    },
                }
            }

            pub fn read(fd: isize, endianness: $crate::dtype::Endianness) -> Result<$name> {
                let mut bytes = [0u8; <$name>::SIZE_BYTES];
                const INNER_SIZE : isize = <$name>::SIZE_BYTES as isize;
                match syscall::read(fd, bytes.as_mut_ptr(), <$name>::SIZE_BYTES) {
                    Ok(INNER_SIZE) => {
                        let value = match endianness {
                            Endianness::None => {
                                $crate::info!("Endianness is zeroed.");
                                panic!("none.")
                            },
                            Endianness::LSB => Self(<$inner>::from_le_bytes(bytes)),
                            Endianness::MSB => Self(<$inner>::from_be_bytes(bytes)),
                            Endianness::Number => {
                                $crate::info!("Endianness is invalid number.");
                                panic!("none.")
                            },
                            Endianness::Undefined => {
                                $crate::info!("Endianness is undefined.");
                                panic!("none.")
                            },
                        };
                        Ok($name::from(value))
                    },
                    Ok(e) => Err($crate::result::Error::DType(Error::ShorterData(e))),
                    Err(e) => Err($crate::result::Error::DType(Error::InvalidData(e.into()))),
                }
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
