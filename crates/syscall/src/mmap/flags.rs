#[repr(i32)]
#[derive(Clone, Copy)]
pub enum Flag {
    Shared = 0x01,
    Private = 0x02,
    SharedValidate = 0x03,
    Fixed = 0x10,
    Anonymous = 0x20,
    GrowsDown = 0x0100,
    DenyWrite = 0x0800,
    Executable = 0x1000,
    Locked = 0x2000,
    NoReserve = 0x4000,
    Populate = 0x8000,
    NonBlock = 0x10000,
    Stack = 0x20000,
    HugeTlb = 0x40000,
    Sync = 0x80000,
    FixedNoReplace = 0x100000,
}

impl Flag {
    fn to(self) -> i32 {
        self as i32
    }
}

impl core::ops::BitOr for Flag {
    type Output = i32;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.to() | rhs.to()
    }
}

impl core::ops::BitOr<i32> for Flag {
    type Output = i32;

    fn bitor(self, rhs: i32) -> Self::Output {
        self.to() | rhs
    }
}

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

impl core::ops::BitOr<i32> for Prot {
    type Output = i32;

    fn bitor(self, rhs: i32) -> Self::Output {
        self.to() | rhs
    }
}
