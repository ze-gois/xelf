use crate::arch32::data::Word as T;

pub enum Info {
    Unused(T),
    OperatingSystem(T),
    ReallocationIndex(T),
}

impl Info {
    pub fn to(&self) -> T {
        match *self {
            Self::Unused(i) => i,
            Self::OperatingSystem(i) => i,
            Self::ReallocationIndex(i) => i,
        }
    }

    pub fn as_string(&self) -> String {
        match *self {
            Self::Unused(i) => format!("Unused ({i})"),
            Self::OperatingSystem(i) => format!("OS index {i}"),
            Self::ReallocationIndex(i) => format!("Reallocation index ({i})"),
        }
    }
}
