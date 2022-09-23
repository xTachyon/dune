use crate::game::GameMode;
use crate::nbt;
use crate::protocol::varint::read_varint;
use anyhow::{anyhow, Result};
use byteorder::ReadBytesExt;
use std::convert::TryFrom;
use std::io::{Cursor, Read};
use std::ops::Range;

use super::{IndexedBuffer, IndexedString, InventorySlot, InventorySlotData};

pub(crate) trait MD {
    fn deserialize(reader: &mut Reader) -> Result<Self>
    where
        Self: Sized;
}

macro_rules! impl_for_numbers {
    ($($number:ident)*) => {
        $(
            impl MD for $number {
                fn deserialize(mut reader: &mut Reader) -> Result<Self> {
                    let mut buffer = [0u8; std::mem::size_of::<$number>()];
                    reader.read_exact(&mut buffer)?;
                    let value = $number::from_be_bytes(buffer.into());
                    Ok(value)
                }
            }
        )*
    };
}

impl_for_numbers!(u16 u32 u64 u128 i16 i32 i64 f32 f64);

impl MD for u8 {
    fn deserialize(mut reader: &mut Reader) -> Result<Self> {
        let value = reader.read_u8()?;
        Ok(value)
    }
}

impl MD for i8 {
    fn deserialize(mut reader: &mut Reader) -> Result<Self> {
        let value = reader.read_i8()?;
        Ok(value)
    }
}

impl MD for bool {
    fn deserialize(reader: &mut Reader) -> Result<Self> where {
        let value: u8 = MD::deserialize(reader)?;
        let result = value != 0;
        Ok(result)
    }
}

const MAX_DATA_SIZE: usize = 5 * 1024 * 1024;

impl MD for String {
    fn deserialize(mut reader: &mut Reader) -> Result<Self> {
        let size = read_varint(&mut reader)? as usize;
        if size > MAX_DATA_SIZE {
            return Err(anyhow!("string size too big"));
        }
        let mut buffer = vec![0; size];
        reader.read_exact(&mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    }
}

impl MD for Vec<u8> {
    fn deserialize(mut reader: &mut Reader) -> Result<Self> {
        let size = read_varint(&mut reader)? as usize;
        if size > MAX_DATA_SIZE {
            return Err(anyhow!("buffer size too big"));
        }
        let mut buffer = vec![0; size];
        reader.read_exact(&mut buffer)?;
        Ok(buffer)
    }
}

impl MD for IndexedBuffer {
    fn deserialize(reader: &mut Reader) -> Result<Self> {
        reader.read_indexed_buffer()
    }
}

impl MD for IndexedString {
    fn deserialize(reader: &mut Reader) -> Result<Self> {
        reader.read_indexed_string()
    }
}

impl MD for InventorySlot {
    fn deserialize(mut reader: &mut Reader) -> Result<Self> {
        let present: bool = MD::deserialize(reader)?;

        let data = if present {
            let item_id = read_varint(&mut reader)?;
            let count = MD::deserialize(reader)?;
            let start = reader.offset() as u32;

            let nbt = if nbt::skip_option(&mut reader)? {
                let end = reader.offset() as u32;
                Some(IndexedBuffer { start, end })
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
}

impl<T: MD> MD for Option<T> {
    fn deserialize(reader: &mut Reader) -> Result<Option<T>>
    where
        Self: Sized,
    {
        let b = MD::deserialize(reader)?;
        let result = if b {
            Some(MD::deserialize(reader)?)
        } else {
            None
        };
        Ok(result)
    }
}

macro_rules! impl_with_varint {
    ($enu:ident) => {
        impl MD for $enu {
            fn deserialize(reader: &mut Reader) -> Result<Self> {
                let value = read_varint(reader)?;
                Ok($enu::try_from(value as u8)?)
            }
        }
    };
}

impl_with_varint!(GameMode);

pub struct Reader<'r> {
    cursor: Cursor<&'r [u8]>,
}

impl<'r> Reader<'r> {
    pub fn new(buffer: &[u8]) -> Reader {
        Reader {
            cursor: Cursor::new(buffer),
        }
    }

    pub fn get_buf_from(&self, r: Range<usize>) -> Result<&[u8]> {
        let bytes = &self.get()[r];
        Ok(bytes)
    }

    pub fn read_range(&mut self) -> Result<Range<u32>> {
        let size = read_varint(&mut self.cursor)? as usize;
        self.read_range_size(size)
    }

    pub fn read_range_size(&mut self, size: usize) -> Result<Range<u32>> {
        let start = self.offset();
        let end = start + size;
        let vec_len = self.get().len();

        if end <= vec_len {
            self.cursor.set_position(end as u64);
            Ok(start as u32..end as u32)
        } else {
            Err(anyhow!("not enough bytes for str"))
        }
    }

    pub fn read_indexed_string(&mut self) -> Result<IndexedString> {
        let size = read_varint(&mut self.cursor)? as usize;
        let r = self.read_range_size(size)?;
        Ok(IndexedString {
            start: r.start,
            end: r.end,
        })
    }

    pub fn read_indexed_buffer(&mut self) -> Result<IndexedBuffer> {
        let size = read_varint(&mut self.cursor)? as usize;
        self.read_indexed_buffer_size(size)
    }

    pub fn read_indexed_buffer_size(&mut self, size: usize) -> Result<IndexedBuffer> {
        let r = self.read_range_size(size)?;
        Ok(IndexedBuffer {
            start: r.start,
            end: r.end,
        })
    }

    pub fn read_rest_buffer(&mut self) -> IndexedBuffer {
        let end = self.get().len() ;
        let r = self.offset() as u32..end as u32;
        self.cursor.set_position(end as u64);
        IndexedBuffer {
            start: r.start,
            end: r.end,
        }
    }

    pub fn get(&self) -> &[u8] {
        self.cursor.get_ref()
    }

    pub fn offset(&self) -> usize {
        self.cursor.position() as usize
    }
}

impl<'r> Read for &mut Reader<'r> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.cursor.read(buf)
    }
}

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl MD for Position {
    fn deserialize(reader: &mut Reader) -> Result<Position>
    where
        Self: Sized,
    {
        let val: u64 = MD::deserialize(reader)?;
        let x = (val >> 38) as i32;
        let y = (val & 0xFFF) as i32;
        let z = ((val >> 12) & 0x3FFFFFF) as i32;

        Ok(Position { x, y, z })
    }
}
