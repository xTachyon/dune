use crate::de::MinecraftDeserialize;
use crate::varint::VarInt;
use crate::{MyResult, PacketDirection};
use enum_primitive_derive::*;
use num_traits::{FromPrimitive, ToPrimitive};
use std::io::{Cursor, Read};

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

#[derive(Copy, Clone, Debug, Primitive)]
pub enum ConnectionState {
    Handshake = 0,
    Status = 1,
    Play = 2,
    Login = 3,
}

pub struct PacketInfo {
    pub direction: PacketDirection,
    pub connection_state: ConnectionState,
    pub id: u16,
}

impl PacketInfo {
    pub fn big_id(&self) -> u64 {
        let mut result = self.id as u64;
        result += 1000 * self.connection_state as u64;
        result += 100000 * self.direction as u64;
        result
    }
}

#[derive(Default)]
pub struct PacketHeaderNoCompression {
    length: VarInt,
    id: VarInt,
}

deserialize_for!(PacketHeaderNoCompression length id);

#[derive(Debug, Default)]
pub struct Handshake {
    pub version: VarInt,
    pub address: String,
    pub port: u16,
    pub next_state: VarInt,
}

deserialize_for!(Handshake version address port next_state);

#[derive(Debug, Default)]
pub struct StatusRequest {}

deserialize_for!(StatusRequest);

#[derive(Debug, Default)]
pub struct StatusResponse {
    response: String,
}

deserialize_for!(StatusResponse response);

#[derive(Debug, Default)]
pub struct ChatResponse {
    response: String,
    position: u8,
}

deserialize_for!(ChatResponse response position);

macro_rules! packet_macro {
    ($($name:ident $state:ident $direction:ident $id:expr)*) => {
      #[derive(Debug)]
      pub enum Packet {
        $(
          $name($name),
        )*
        Unknown(ConnectionState, u16)
      }

      pub fn deserialize<R: Read>(direction: PacketDirection, state: ConnectionState, id: u16, mut reader: R) -> MyResult<Packet> {
        let result = match (direction, state, id) {
          $(
            (PacketDirection::$direction, ConnectionState::$state, $id) => Packet::$name($name::deserialize(&mut reader)?),
          )*
          _ => Packet::Unknown(state, id)
        };
        Ok(result)
      }
    };
}

packet_macro!(
Handshake      Handshake ClientToServer 0x00

StatusRequest  Status    ClientToServer 0x00
StatusResponse Status    ServerToClient 0x00

ChatResponse   Play      ServerToClient 0x0E
);

pub fn deserialize_with_header(
    direction: PacketDirection,
    state: ConnectionState,
    bytes: &[u8],
) -> MyResult<Option<(Packet, usize)>> {
    let mut reader = Cursor::new(bytes);
    let header = match PacketHeaderNoCompression::deserialize(&mut reader) {
        Ok(x) => x,
        Err(_) => return Ok(None),
    };

    let packet = deserialize(direction, state, header.id.get() as u16, &mut reader)?;
    dbg!(&packet);
    let new_position = header.length.get() as usize + header.length.size();
    let result = (packet, new_position);
    Ok(Some(result))
}
