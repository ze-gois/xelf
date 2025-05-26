// %% Cell 1
#[derive(Debug)]
pub struct A {
    a: u8,
    b: i32,
}

impl A {
    pub fn load(offset:&mut i32) -> Self {
        let a = Self { a: 3, b: *offset };
        *offset = *offset + 10;
        a
    }

    pub fn loadvn(offset:&mut i32, n:usize) -> Vec<Self> {
        (0..n).map(|_|{Self::load(offset)}).collect::<Vec<Self>>()
    }

    // pub fn loadsn(offset:&mut i32, n:usize) -> Vec<Self> {
    //     (0..n).map(|_|{Self::load(offset)}).collect::<[Self;n]>()
    // }
}
// %% Cell
let mut offset = 5;
A::loadvn(&mut offset,3).as_slice()


// %% Cell 2
let mut offset : i32 = 6;
let n = 9;
let x = A::loadvn(&mut offset,n);

x
// %% Cell

let y = x.to_owned().as_slice();

// %% Cell 3
let mut offset = 3;
let array : [A;5] = core::array::from_fn(|_|A::load(&mut offset));
array
