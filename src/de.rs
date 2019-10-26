use crate::varint::VarInt;
use crate::MyResult;
use byteorder::{ReadBytesExt, };
use std::io::Read;

pub(crate) trait MinecraftDeserialize {
    fn deserialize<R: Read>(reader: R) -> MyResult<Self>
    where
        Self: Sized;
}

macro_rules! impl_for_numbers {
    ($($number:ident)*) => {
        $(
            impl MinecraftDeserialize for $number {
                fn deserialize<R: Read>(mut reader: R) -> MyResult<Self> {
                    let mut buffer = [0u8; std::mem::size_of::<$number>()];
                    reader.read_exact(&mut buffer)?;
                    let value = $number::from_be_bytes(buffer.into());
                    Ok(value)
                }
            }
        )*
    };
}

impl_for_numbers!(u16 u32 u64 i16 i32 i64);

impl MinecraftDeserialize for u8 {
    fn deserialize<R: Read>(mut reader: R) -> MyResult<Self> {
        let value = reader.read_u8()?;
        Ok(value)
    }
}

impl MinecraftDeserialize for VarInt {
    fn deserialize<R: Read>(reader: R) -> MyResult<Self> {
        VarInt::deserialize_read(reader)
    }
}

impl MinecraftDeserialize for String {
    fn deserialize<R: Read>(mut reader: R) -> MyResult<Self> {
        let size = <VarInt as MinecraftDeserialize>::deserialize(&mut reader)?;
        let mut buffer = vec![0; size.get() as usize];
        reader.read_exact(&mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    }
}
