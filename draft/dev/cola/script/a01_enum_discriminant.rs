// %% cell

#[repr(u8)]
#[derive(Copy,Clone)]
#[derive(Debug)]
pub enum Enum {
    A = 0,
    B = 1,
    C = 2,
}

impl Enum {
    fn d(&self) -> u8 {
        self.clone() as _
    }
}

let x = 1;

let y = match x as Enum  {
    Enum::A => "Foi A",
    Enum::B => "Foi B",
    Enum::C => "Foi C",
    _ =>  "None"
};
// %% cell

#[repr(u8)]
#[derive(Copy,Clone)]
#[derive(Debug)]
pub enum Enum {
    A = (0,"Fa"),
    B = (1,"Fb"),
    C = (2,"Fc"),
}

impl Enum {
    fn d(&self) -> u8 {
        self.clone() as _
    }
}

let x = 1;

let y = match x as Enum  {
    Enum::A.0 => Enum::A.1,
    Enum::B.0 => Enum::A.1,
    Enum::C.0 => Enum::A.1,
    _ =>  "None"
};
