use crate::game::GameMode;
use crate::protocol::varint::read_varint;
use anyhow::{anyhow, Result};
use byteorder::ReadBytesExt;
use std::convert::TryFrom;
use std::io::{Cursor, Read};
use std::ops::Range;

pub(crate) trait MinecraftDeserialize {
    fn deserialize<R: Read>(reader: R) -> Result<Self>
    where
        Self: Sized;
}

macro_rules! impl_for_numbers {
    ($($number:ident)*) => {
        $(
            impl MinecraftDeserialize for $number {
                fn deserialize<R: Read>(mut reader: R) -> Result<Self> {
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

impl MinecraftDeserialize for u8 {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self> {
        let value = reader.read_u8()?;
        Ok(value)
    }
}

impl MinecraftDeserialize for i8 {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self> {
        let value = reader.read_i8()?;
        Ok(value)
    }
}

impl MinecraftDeserialize for bool {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self> where {
        let value: u8 = MinecraftDeserialize::deserialize(&mut reader)?;
        let result = value != 0;
        Ok(result)
    }
}

const MAX_DATA_SIZE: usize = 5 * 1024 * 1024;

impl MinecraftDeserialize for String {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self> {
        let size = read_varint(&mut reader)? as usize;
        if size > MAX_DATA_SIZE {
            return Err(anyhow!("string size too big"));
        }
        let mut buffer = vec![0; size];
        reader.read_exact(&mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    }
}

impl MinecraftDeserialize for Vec<u8> {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self> {
        let size = read_varint(&mut reader)? as usize;
        if size > MAX_DATA_SIZE {
            return Err(anyhow!("buffer size too big"));
        }
        let mut buffer = vec![0; size];
        reader.read_exact(&mut buffer)?;
        Ok(buffer)
    }
}

impl<T: MinecraftDeserialize> MinecraftDeserialize for Option<T> {
    fn deserialize<R: Read>(mut reader: R) -> Result<Option<T>>
    where
        Self: Sized,
    {
        let b = MinecraftDeserialize::deserialize(&mut reader)?;
        let result = if b {
            Some(MinecraftDeserialize::deserialize(&mut reader)?)
        } else {
            None
        };
        Ok(result)
    }
}

macro_rules! impl_with_varint {
    ($enu:ident) => {
        impl MinecraftDeserialize for $enu {
            fn deserialize<R: Read>(reader: R) -> Result<Self> {
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

    pub fn read_range(&mut self) -> Result<Range<usize>> {
        let size = read_varint(&mut self.cursor)? as usize;
        self.read_range_size(size)
    }

    pub fn read_range_size(&mut self, size: usize) -> Result<Range<usize>> {
        let start = self.offset();
        let end = start + size;
        let vec_len = self.get().len();

        if end <= vec_len {
            self.cursor.set_position(end as u64);
            Ok(start..end)
        } else {
            Err(anyhow!("not enough bytes for str"))
        }
    }

    pub fn get_str_from(&self, r: Range<usize>) -> Result<&str> {
        let bytes = &self.get()[r];
        let result = std::str::from_utf8(bytes)?;
        Ok(result)
    }

    pub fn get_buf_from(&self, r: Range<usize>) -> Result<&[u8]> {
        let bytes = &self.get()[r];
        Ok(bytes)
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

impl MinecraftDeserialize for Position {
    fn deserialize<R: Read>(mut reader: R) -> Result<Position>
    where
        Self: Sized,
    {
        let val: u64 = MinecraftDeserialize::deserialize(&mut reader)?;
        let x = (val >> 38) as i32;
        let y = (val & 0xFFF) as i32;
        let z = ((val >> 12) & 0x3FFFFFF) as i32;

        Ok(Position { x, y, z })
    }
}