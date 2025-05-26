use core::result::Result;
use std::error::Error;

pub struct Identifier {
    mag: [u8; 4],
    class: u8,
    data: u8,
    version: u8,
    osabi: u8,
    abiversion: u8,
    pad: u8,
    nident: u8,
}

impl Identifier {
    // fn from_map(fm: memmap2::Mmap) -> Result<Self, Error> {
    //     let mag: [u8; 4] = [fm[0], fm[1], fm[2], fm[3]];
    //     let class = fm[4];
    //     return match class {
    //         1 => Ok(Self {
    //             mag,
    //             class,
    //             data: 0,
    //             osabi: 0,
    //             abiversion: 0,
    //             version: 0,
    //             pad: 0,
    //             nident: 0,
    //         }),
    //         2 => Ok(Self {
    //             mag,
    //             class,
    //             data: 0,
    //             osabi: 0,
    //             abiversion: 0,
    //             version: 0,
    //             pad: 0,
    //             nident: 0,
    //         }),
    //         _ => Err(()),
    //     };
    // }
}
