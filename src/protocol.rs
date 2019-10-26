use crate::de::MinecraftDeserialize;
use crate::varint::VarInt;
use crate::{MyResult, PacketDirection};
use enum_primitive_derive::*;
use std::io::{Cursor, Read};

macro_rules! deserialize_for {
    ($type:ident $($field:ident)*) => {
      impl MinecraftDeserialize for $type {
        fn deserialize(_reader: &mut dyn Read) -> MyResult<Self> {
          $(
            let $field = MinecraftDeserialize::deserialize(_reader)?;
          )*
          let result = $type {
            $(
                $field,
            )*
          };
          Ok(result)
        }
      }
    };
}

#[derive(Copy, Clone, Debug, Primitive)]
pub enum ConnectionState {
    Handshake = 0,
    Status = 1,
    Login = 2,
    Play = 3,
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

#[derive(Debug, Default)]
pub struct SetCompression {
    pub value: VarInt,
}

deserialize_for!(SetCompression value);

#[derive(Debug, Default)]
pub struct LoginSuccess {
    pub uuid: String,
    pub username: String,
}

deserialize_for!(LoginSuccess uuid username);

macro_rules! packet_macro {
    ($($name:ident $state:ident $direction:ident $id:expr)*) => {
      #[derive(Debug)]
      pub enum Packet {
        $(
          $name($name),
        )*
        Unknown(ConnectionState, u32)
      }

      fn deserialize<R: Read>(direction: PacketDirection, state: ConnectionState, id: u32, mut reader: R) -> MyResult<Packet> {
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

LoginSuccess   Login     ServerToClient 0x02
SetCompression Login     ServerToClient 0x03

ChatResponse   Play      ServerToClient 0x0E
);

pub fn deserialize_with_header(
    direction: PacketDirection,
    state: ConnectionState,
    bytes: &[u8],
    compression: Option<u32>,
) -> MyResult<Option<(Packet, usize)>> {
    match compression {
        Some(x) => deserialize_compressed(direction, state, bytes, x),
        None => deserialize_uncompressed(direction, state, bytes),
    }
}

fn deserialize_compressed(
    direction: PacketDirection,
    state: ConnectionState,
    bytes: &[u8],
    compression: u32,
) -> MyResult<Option<(Packet, usize)>> {
    if !has_enough_bytes(bytes) {
        return Ok(None);
    }
    let mut reader = Cursor::new(bytes);
    let length: VarInt = MinecraftDeserialize::deserialize(&mut reader)?;
    let data_length: VarInt = MinecraftDeserialize::deserialize(&mut reader)?;
    let packet = if length.get() >= compression {
        let id: VarInt = MinecraftDeserialize::deserialize(&mut reader)?;
        deserialize(direction, state, id.get(), reader)?
    } else {
        let id: VarInt = MinecraftDeserialize::deserialize(&mut reader)?;
        deserialize(direction, state, id.get(), reader)?
    };
    dbg!(packet);

    unimplemented!()
}

fn deserialize_uncompressed(
    direction: PacketDirection,
    state: ConnectionState,
    bytes: &[u8],
) -> MyResult<Option<(Packet, usize)>> {
    if !has_enough_bytes(bytes) {
        return Ok(None);
    }
    let mut reader = Cursor::new(bytes);
    let header = match PacketHeaderNoCompression::deserialize(&mut reader) {
        Ok(x) => x,
        Err(_) => return Ok(None),
    };

    let packet = deserialize(direction, state, header.id.get(), &mut reader)?;
    dbg!(&packet);
    let new_position = header.length.get() as usize + header.length.size();
    let result = (packet, new_position);
    Ok(Some(result))
}

fn has_enough_bytes(bytes: &[u8]) -> bool {
    if let Some(x) = VarInt::deserialize(bytes) {
        bytes.len() >= x.get() as usize + x.size()
    } else {
        false
    }
}
