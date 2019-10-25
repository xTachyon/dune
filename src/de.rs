use crate::varint::VarInt;
use crate::MyResult;
use byteorder::ReadBytesExt;
use byteorder::BE;
use std::io::Read;

pub(crate) trait MinecraftDeserialize {
    fn deserialize(reader: &mut dyn Read) -> MyResult<Self>
    where
        Self: Sized;
}

impl MinecraftDeserialize for u32 {
    fn deserialize(reader: &mut dyn Read) -> MyResult<Self> {
        let value = reader.read_u32::<BE>()?;
        Ok(value)
    }
}

impl MinecraftDeserialize for VarInt {
    fn deserialize(reader: &mut dyn Read) -> MyResult<Self> {
        VarInt::deserialize_read(reader)
    }
}
