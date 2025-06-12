use crate::dtype;

#[derive(Clone)]
pub struct Entry<'a> {
    pub offset: dtype::Off,
    pub content: &'a str,
}
