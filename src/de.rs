use std::io::Read;
use crate::MyResult;
use byteorder::ReadBytesExt;
use byteorder::BE;
use crate::varint::VarInt;

pub(crate) trait MinecraftDeserialize {
  fn deserialize(reader: &mut dyn Read) -> MyResult<Self> where Self: Sized;
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