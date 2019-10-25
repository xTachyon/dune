use crate::varint::VarInt;
use crate::de::MinecraftDeserialize;
use std::io::Read;
use crate::MyResult;

macro_rules! deserialize_for {
    ($type:ident $($field:ident)*) => {
      impl MinecraftDeserialize for $type {
        fn deserialize(reader: &mut dyn Read) -> MyResult<Self> {
          let mut result = $type::default();
          $(
            result.$field = MinecraftDeserialize::deserialize(reader)?;
          )*
          Ok(resultx)
        }
      }
    };
}

#[derive(Default)]
pub struct PacketHeaderNoCompression {
  length: VarInt,
  id: VarInt,
}

deserialize_for!(PacketHeaderNoCompression length id);