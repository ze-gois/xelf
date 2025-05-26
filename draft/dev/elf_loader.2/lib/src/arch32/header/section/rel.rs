use crate::arch32::data;

pub struct Rel {
    pub offset: data::Addr, /* Address of reference */
    pub info: data::Word,   /* Symbol index and type of relocation */
}

pub enum Type {
    T32 = 1,
    Got32 = 3,
    Plt32 = 4,
    Copy = 5,
    GlobDat = 6,
    JumpSlot = 7,
    Relative = 8,
    Gotoff = 9,
    Gotpc = 10,
}
