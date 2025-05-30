#[macro_export]
macro_rules! elf_info {
    ($($arg:tt)*) => {{
        $crate::info!($($arg)*)
    }};
}

#[macro_export]
macro_rules! impl_partial_eq_for_type {
    ($name:ty, $inner:ty, $($t:ty),*) => {
        $(
            // T with our type
            impl PartialEq<$t> for $name {
                fn eq(&self, other: &$t) -> bool {
                    if (*other as u128) <= <$inner>::MAX as u128 {
                        self.0 == *other as $inner
                    } else {
                        false
                    }
                }
            }

            // Our type with T
            impl PartialEq<$name> for $t {
                fn eq(&self, other: &$name) -> bool {
                    if (*self as u128) <= <$inner>::MAX as u128 {
                        *self as $inner == other.0
                    } else {
                        false
                    }
                }
            }

            // Add comparison operators
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

#[macro_export]
macro_rules! impl_into_for_type {
    ($name:ty, $inner:ty, $($t:ty),*) => {
        $(
            impl Into<$t> for $name {
                fn into(val: $name) -> $t {
                    val.0 as $t
                }
            }
        )*
    }
}

#[macro_export]
macro_rules! elf_define_type {
    ($(#[$meta:meta])* pub $name:ident, $inner:ty) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct $name(pub $inner);

        impl $crate::elf::dtype::ELFType for $name {
            type Inner = $inner;
            const SIZE_BYTES: usize = core::mem::size_of::<$inner>();
            const SIZE_BITS: usize = Self::SIZE_BYTES * 8;
        }

        impl $name {
            pub fn new(value: $inner) -> Self {
                Self(value)
            }

            pub fn from_bytes(bytes: [u8; Self::SIZE_BYTES], endianness:Endianness) -> Self {
                match endianness {
                    Endianness::Little => Self(<$inner>::from_le_bytes(bytes)),
                    Endianness::Big => Self(<$inner>::from_be_bytes(bytes)),
                }
            }

            pub fn to_bytes(&self, endianness:Endianness) -> [u8; Self::SIZE_BYTES] {
                match endianness {
                    Endianness::Little => self.0.to_le_bytes(),
                    Endianness::Big => self.0.to_be_bytes(),
                }
            }

            pub async fn read(offset:&mut u64, file : alloc::sync::Arc<$crate::fs::DynFile>, endianness:Endianness) -> $crate::Result<$name> {
                use $crate::fs::File;
                use $crate::data_buffer;
                let mut bytes = [0u8; <$name>::SIZE_BYTES];
                let length = (*file).read(offset, &mut data_buffer::Kernel::new(&mut bytes)).await?;
                return if length == <$name>::SIZE_BYTES as u64 {
                    Ok($name(match endianness {
                        Endianness::Little => <$inner>::from_le_bytes(bytes),
                        Endianness::Big => <$inner>::from_be_bytes(bytes),
                    }))
                } else {
                    Err($crate::Error::NotFound)
                }
            }

            pub async fn read_buffer(offset:&mut u64, file : alloc::sync::Arc<$crate::fs::DynFile>, endianness:Endianness, buffer : &mut $name) -> $crate::Result<u64> {
                use $crate::fs::File;
                use $crate::data_buffer;
                let mut bytes = [0u8; <$name>::SIZE_BYTES];
                let length = (*file).read(offset, &mut data_buffer::Kernel::new(&mut bytes)).await?;
                return if length == <$name>::SIZE_BYTES as u64 {
                    let value = match endianness {
                        Endianness::Little => <$inner>::from_le_bytes(bytes),
                        Endianness::Big => <$inner>::from_be_bytes(bytes),
                    };

                    *buffer = $name(value);
                    Ok(length)
                } else {
                    Err($crate::Error::NotFound)
                }
            }

            pub async fn read_vector(offset:&mut u64, file : alloc::sync::Arc<$crate::fs::DynFile>, endianness:Endianness, vector: &mut alloc::vec::Vec<$name>) -> $crate::Result<usize> {
                let mut byte_counter = 0;
                for _ in  0..vector.len(){
                    let mut value = $name::default();
                    match $name::read_buffer(offset, file.clone(), endianness, &mut value).await {
                        Ok(length) => {
                            byte_counter += length;
                            if length == $name::SIZE_BYTES as u64{
                                vector.push(value);
                            } else {
                                $crate::info!("Failed to read the {:?}th byte\n", byte_counter);
                                return Err($crate::Error::NotFound)
                            }
                        },
                        _ => {
                            $crate::info!("Failed to read the {:?}th byte\n", byte_counter);
                            return Err($crate::Error::NotFound)
                        },
                    };
                };
                Ok(vector.len())
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

        impl_partial_eq_for_type!($name, $inner, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);
        impl_from_for_type!($name, $inner, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);
        // impl_into_for_type!($name, $inner, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);
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
