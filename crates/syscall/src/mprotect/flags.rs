#[repr(i32)]
#[derive(Clone, Copy)]
pub enum Prot {
    None = 0,
    Read = 1,
    Write = 2,
    Exec = 4,
}

impl Prot {
    fn to(self) -> i32 {
        self as i32
    }
}

impl core::ops::BitOr for Prot {
    type Output = i32;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.to() | rhs.to()
    }
}
