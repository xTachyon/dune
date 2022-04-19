use crate::game::Gamemode;
use crate::varint::VarInt;
use anyhow::Result;
use byteorder::ReadBytesExt;
use std::convert::TryFrom;
use std::io::Read;

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

impl MinecraftDeserialize for bool {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self> where {
        let value: u8 = MinecraftDeserialize::deserialize(&mut reader)?;
        let result = if value == 0 { false } else { true };
        Ok(result)
    }
}

impl MinecraftDeserialize for VarInt {
    fn deserialize<R: Read>(reader: R) -> Result<Self> {
        VarInt::deserialize_read(reader)
    }
}

impl MinecraftDeserialize for String {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self> {
        let size = <VarInt as MinecraftDeserialize>::deserialize(&mut reader)?;
        let mut buffer = vec![0; size.get() as usize];
        reader.read_exact(&mut buffer)?;
        Ok(String::from_utf8(buffer)?)
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

//macro_rules! impl_for_tuples {
//    ($($template:ident,)*) => {
//        impl<
//        $(
//            $template
//        )* > MinecraftDeserialize for (
//        $(
//            $template
//        )*
//        ) {
//
//        }
//    };
//}
//
//impl_for_tuples!(A, B);

impl<A, B, C> MinecraftDeserialize for (A, B, C)
where
    A: MinecraftDeserialize,
    B: MinecraftDeserialize,
    C: MinecraftDeserialize,
{
    fn deserialize<R: Read>(mut reader: R) -> Result<Self>
    where
        Self: Sized,
    {
        let a = MinecraftDeserialize::deserialize(&mut reader)?;
        let b = MinecraftDeserialize::deserialize(&mut reader)?;
        let c = MinecraftDeserialize::deserialize(&mut reader)?;

        Ok((a, b, c))
    }
}

macro_rules! impl_forward {
    ($enu:ident, $type:ty) => {
        impl MinecraftDeserialize for $enu {
            fn deserialize<R: Read>(reader: R) -> Result<Self> {
                let value = <$type as MinecraftDeserialize>::deserialize(reader)?;
                Ok($enu::try_from(value.get() as u8)?)
            }
        }
    };
}

impl_forward!(Gamemode, VarInt);
