use crate::protocol::varint::read_varint;
use anyhow::Result;
use dune_common::nbt;
use std::io::{self, Read, Result as IoResult, Write};

use super::varint::write_varint;
use super::{ChunkBlockEntity, IndexedNbt, IndexedOptionNbt, InventorySlot, InventorySlotData};

pub trait MemoryExt<'x> {
    fn read_mem(&mut self, size: usize) -> IoResult<&'x [u8]>;
}
impl<'x> MemoryExt<'x> for &'x [u8] {
    fn read_mem(&mut self, size: usize) -> IoResult<&'x [u8]> {
        if size > self.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "failed to fill whole buffer",
            ));
        }
        let b = &self[..size];
        *self = &self[size..];
        Ok(b)
    }
}

pub trait MD<'x> {
    fn serialize<W: Write>(&self, writer: &mut W) -> IoResult<()>;
    fn deserialize(memory: &mut &'x [u8]) -> Result<Self>
    where
        Self: Sized;
}

macro_rules! impl_for_numbers {
    ($($number:ident)*) => {
        $(
            impl<'x> MD<'x> for $number {
                fn deserialize(memory: &mut &'x [u8]) -> Result<Self> {
                    let mut buffer = [0u8; std::mem::size_of::<$number>()];
                    memory.read_exact(&mut buffer)?;
                    let value = $number::from_be_bytes(buffer.into());
                    Ok(value)
                }
                fn serialize<W: Write>(&self, writer: &mut W) -> IoResult<()> {
                    let buffer = self.to_be_bytes();
                    writer.write_all(&buffer)?;
                    Ok(())
                }
            }
        )*
    };
}

impl_for_numbers!(u16 u32 u64 u128 i16 i32 i64 f32 f64);

impl<'x, const SIZE: usize> MD<'x> for &'x [u8; SIZE] {
    fn deserialize(memory: &mut &'x [u8]) -> Result<Self> {
        let slice = memory.read_mem(SIZE)?;
        let ret = slice.try_into().expect("the slice should always have SIZE elements");
        Ok(ret)
    }
    fn serialize<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        writer.write_all(self.as_slice())
    }
}

impl<'x> MD<'x> for &'x str {
    fn deserialize(memory: &mut &'x [u8]) -> Result<Self> {
        let slice: &[u8] = MD::deserialize(memory)?;
        let s = std::str::from_utf8(slice)?;
        Ok(s)
    }
    fn serialize<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        self.as_bytes().serialize(writer)
    }
}

impl<'x> MD<'x> for &'x [u8] {
    fn deserialize(mut memory: &mut &'x [u8]) -> Result<Self> {
        let size: usize = read_varint(&mut memory)?.try_into()?;
        let slice = memory.read_mem(size)?;
        Ok(slice)
    }
    fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
        write_varint(&mut writer, self.len() as u32)?;
        writer.write_all(self)?;
        Ok(())
    }
}

pub(super) fn read_u8<R: Read>(mut reader: R) -> Result<u8> {
    let mut arr = [0; 1];
    reader.read_exact(&mut arr)?;
    Ok(arr[0])
}

impl<'x> MD<'x> for u8 {
    fn deserialize(memory: &mut &'x [u8]) -> Result<Self> {
        read_u8(memory)
    }
    fn serialize<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        writer.write_all(&[*self])?;
        Ok(())
    }
}

impl<'x> MD<'x> for i8 {
    fn deserialize(memory: &mut &'x [u8]) -> Result<Self> {
        let value = read_u8(memory)?;
        Ok(value as i8)
    }
    fn serialize<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        writer.write_all(&[*self as u8])?;
        Ok(())
    }
}

impl<'x> MD<'x> for bool {
    fn deserialize(memory: &mut &'x [u8]) -> Result<Self> {
        let value: u8 = MD::deserialize(memory)?;
        let result = value != 0;
        Ok(result)
    }
    fn serialize<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        writer.write_all(&[*self as u8])?;
        Ok(())
    }
}

impl<'x> MD<'x> for InventorySlot<'x> {
    fn deserialize(mut memory: &mut &'x [u8]) -> Result<Self> {
        let present: bool = MD::deserialize(memory)?;

        let data = if present {
            let item_id = read_varint(&mut memory)?;
            let count = MD::deserialize(memory)?;
            let original = *memory;

            let nbt = if nbt::skip_option(&mut memory)? {
                let size = memory.as_ptr() as usize - original.as_ptr() as usize;
                let buf = &original[..size];
                Some(buf)
            } else {
                None
            };

            Some(InventorySlotData {
                item_id,
                count,
                nbt,
            })
        } else {
            None
        };

        Ok(InventorySlot { data })
    }
    fn serialize<W: Write>(&self, _writer: &mut W) -> IoResult<()> {
        unimplemented!()
    }
}

impl<'x> MD<'x> for IndexedNbt<'x> {
    fn deserialize(mut memory: &mut &'x [u8]) -> Result<Self> {
        let tmp = *memory;
        nbt::skip(&mut memory)?;
        let size = memory.as_ptr() as usize - tmp.as_ptr() as usize;
        let nbt = &tmp[..size];
        Ok(IndexedNbt { nbt })
    }
    fn serialize<W: Write>(&self, _writer: &mut W) -> IoResult<()> {
        unimplemented!()
    }
}

impl<'x> MD<'x> for IndexedOptionNbt<'x> {
    fn deserialize(mut memory: &mut &'x [u8]) -> Result<Self> {
        let tmp = *memory;
        let nbt = if nbt::skip_option(&mut memory)? {
            let size = memory.as_ptr() as usize - tmp.as_ptr() as usize;
            let nbt = &tmp[..size];
            Some(nbt)
        } else {
            None
        };
        Ok(IndexedOptionNbt { nbt })
    }
    fn serialize<W: Write>(&self, _writer: &mut W) -> IoResult<()> {
        unimplemented!()
    }
}

impl<'x, T: MD<'x>> MD<'x> for Option<T> {
    fn deserialize(memory: &mut &'x [u8]) -> Result<Self> {
        let b = MD::deserialize(memory)?;
        let result = if b {
            Some(MD::deserialize(memory)?)
        } else {
            None
        };
        Ok(result)
    }
    fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
        match self {
            None => false.serialize(&mut writer)?,
            Some(x) => {
                true.serialize(&mut writer)?;
                x.serialize(&mut writer)?;
            }
        }
        Ok(())
    }
}

// macro_rules! impl_with_varint {
//     ($enu:ident) => {
//         impl<'x> MD<'x> for $enu {
//             fn deserialize(memory: &mut &'x [u8]) -> Result<Self> {
//                 let value = read_varint(memory)?;
//                 Ok($enu::try_from(value as u8)?)
//             }
//             fn serialize<W: Write>(&self, writer: &mut W) -> IoResult<()> {
//                 write_varint(writer, *self as u32)?;
//                 Ok(())
//             }
//         }
//     };
// }

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
// impl Position {
//     pub(crate) fn write<W: Write>(&self, mut writer: W) -> IoResult<()> {
//         writer.write_u64::<BE>(
//             ((self.x as u64 & 0x3FFFFFF) << 38)
//                 | ((self.z as u64 & 0x3FFFFFF) << 12)
//                 | (self.y as u64 & 0xFFF),
//         )
//     }
// }

impl<'x> MD<'x> for Position {
    fn deserialize(memory: &mut &'x [u8]) -> Result<Self> {
        let val: i64 = MD::deserialize(memory)?;
        let x = (val >> 38) as i32;
        let y = (val << 52 >> 52) as i32;
        let z = (val << 26 >> 38) as i32;

        Ok(Position { x, y, z })
    }
    fn serialize<W: Write>(&self, _writer: &mut W) -> IoResult<()> {
        unimplemented!()
    }
}

pub(super) fn cautious_size(size: usize) -> usize {
    const LIMIT: usize = 4096;
    size.min(LIMIT)
}

impl<'x> MD<'x> for ChunkBlockEntity<'x> {
    fn deserialize(mut memory: &mut &'x [u8]) -> Result<Self> {
        let xz = u8::deserialize(memory)?;
        let x = xz & 0b1111;
        let z = xz >> 4;
        let y = i16::deserialize(memory)?;
        let type_ = read_varint(&mut memory)?;
        let nbt_data = IndexedOptionNbt::deserialize(memory)?;

        Ok(ChunkBlockEntity {
            x,
            z,
            y,
            type_,
            nbt_data,
        })
    }
    fn serialize<W: Write>(&self, _writer: &mut W) -> IoResult<()> {
        unimplemented!()
    }
}
