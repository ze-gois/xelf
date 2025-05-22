pub enum Seek {
    SET = 0,
    CUR = 1,
    END = 2,
}

impl Seek {
    pub fn to(self) -> i32 {
        self as i32
    }
}
