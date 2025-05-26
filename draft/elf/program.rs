pub mod header;

pub use header::Header;

use super::dtype::Endianness;

use crate::arch::Va;
use crate::fs::DynFile;
use crate::mm::map::Map;
use crate::{mm, Error, Result};

use alloc::sync::Arc;
use alloc::vec::Vec;

use super::ELF;

pub struct Segment {
    header: Header,
    boundaries: Option<(Va, Va)>,
}

impl Segment {
    pub async fn from_file(
        offset: &mut u64,
        file: Arc<DynFile>,
        endianness: Endianness,
    ) -> Result<Self> {
        let header = Header::from_file(offset, file, endianness).await?;
        Ok(Segment {
            header,
            boundaries: None,
        })
    }

    pub async fn load_file(
        &mut self,
        file: Arc<DynFile>,
        map: &mut Map,
        offset: &mut Va,
    ) -> Result {
        let flags = self.header.p_flags.0;

        let _flags = mm::Flags {
            user: true,
            writable: flags & 2 != 0,
            executable: flags & 1 != 0,
        };

        Ok(())
    }

    pub async fn unload(&self) -> Result {
        Ok(())
    }
}

pub struct Table {
    segments: Option<Vec<Segment>>,
    boundaries: Option<(Va, Va)>,
}

impl Default for Table {
    fn default() -> Self {
        Table {
            segments: None,
            boundaries: None,
        }
    }
}

impl Table {
    pub async fn is_loaded(&self) -> bool {
        if self.segments.is_some() {
            for section in self.segments.as_ref().unwrap() {
                if section.boundaries.is_none() {
                    return false;
                }
            }
        }
        return false;
    }

    pub async fn read_elf<'map>(elf: &mut ELF<'map>) -> Result<()> {
        let mut segments = Vec::<Segment>::with_capacity(elf.header.e_phnum.0 as usize);
        let mut offset = elf.header.e_phoff.0;
        let endianness = elf.endianness()?;

        for _ in 0..elf.header.e_phnum.0 {
            let segment = Segment::from_file(&mut offset, elf.file.clone(), endianness).await?;
            segments.push(segment);
        }

        elf.program_table = Some(Table {
            segments: Some(segments),
            boundaries: None,
        });

        Ok(())
    }

    pub async fn load_elf<'map>(elf: &mut ELF<'map>) -> Result<()> {
        if elf.program_table.is_none() {
            Self::read_elf(elf).await?;
        }

        let mut memory_offset = if elf.is_dynamically_linked() {
            Va(0)
        } else {
            Va(0)
        };

        let mut lower_boundary = Va(0);
        let mut upper_boundary = Va(0);

        let mut do_unload = false;

        for segment in elf
            .program_table
            .as_mut()
            .unwrap()
            .segments
            .as_mut()
            .unwrap()
        {
            match segment
                .load_file(elf.file.clone(), &mut elf.map, &mut memory_offset)
                .await
            {
                Ok(_) => {}
                Err(_) => {
                    do_unload = true;
                    break;
                }
            }

            if let Some((segment_lower_boundary, segment_upper_boundary)) = segment.boundaries {
                if segment_lower_boundary.0 < lower_boundary.0 {
                    lower_boundary.0 = segment_lower_boundary.0;
                }

                if segment_upper_boundary.0 > upper_boundary.0 {
                    upper_boundary.0 = segment_upper_boundary.0;
                }
            }
        }

        if do_unload {
            elf.program_table.as_mut().unwrap().unload().await?;
            return Err(Error::InvalidParam);
        }

        elf.program_table.as_mut().unwrap().boundaries = Some((lower_boundary, upper_boundary));

        Ok(())
    }

    pub async fn unload(&mut self) -> Result {
        for segment in self.segments.as_mut().unwrap() {
            segment.unload().await?;
        }
        self.boundaries = None;
        Ok(())
    }
}
