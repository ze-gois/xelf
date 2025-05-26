use crate::arch64::data;

pub struct Rel {
    pub offset: data::Addr, /* Address of reference */
    pub info: data::XWord,  /* Symbol index and type of relocation */
}
