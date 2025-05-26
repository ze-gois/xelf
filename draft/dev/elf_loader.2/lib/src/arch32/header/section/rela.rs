use crate::arch32::data;

pub struct Rela {
    pub offset: data::Addr,  /* Address of reference */
    pub info: data::Word,    /* Symbol index and type of relocation */
    pub addend: data::SWord, /* Constant part of expression */
}

impl Rela {}
