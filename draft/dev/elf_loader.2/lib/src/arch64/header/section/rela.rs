use crate::arch64::data;

pub struct Rela {
    pub offset: data::Addr,   /* Address of reference */
    pub info: data::XWord,    /* Symbol index and type of relocation */
    pub addend: data::SXWord, /* Constant part of expression */
}
