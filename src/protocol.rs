use crate::de::MinecraftDeserialize;
use crate::varint::VarInt;
use crate::MyResult;
use std::io::Read;

macro_rules! deserialize_for {
    ($type:ident $($field:ident)*) => {
      impl MinecraftDeserialize for $type {
        fn deserialize(reader: &mut dyn Read) -> MyResult<Self> {
          let mut result = $type::default();
          $(
            result.$field = MinecraftDeserialize::deserialize(reader)?;
          )*
          Ok(result)
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
