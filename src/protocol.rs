use crate::de::MinecraftDeserialize;
use crate::varint::VarInt;
use crate::{MyResult, PacketDirection};
use enum_primitive_derive::*;
use flate2::read::ZlibDecoder;
use std::io::{Cursor, Read};

macro_rules! deserialize_for {
    ($type:ident $($field:ident)*) => {
      impl MinecraftDeserialize for $type {
        fn deserialize<R: Read>(mut _reader: R) -> MyResult<Self> {
          $(
            let $field = MinecraftDeserialize::deserialize(&mut _reader)?;
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
    pub response: String,
    pub position: u8,
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
    if !has_enough_bytes(bytes) {
        return Ok(None);
    }
    let length: VarInt = MinecraftDeserialize::deserialize(bytes)?;
    let bytes = &bytes[length.size()..length.get() as usize + length.size()];

    let packet = match compression {
        Some(x) => deserialize_compressed(direction, state, bytes, x),
        None => deserialize_uncompressed(direction, state, bytes),
    }?;
    let result = match packet {
        Some(packet) => Some((packet, length.get() as usize + length.size())),
        None => None,
    };
    Ok(result)
}

fn deserialize_compressed(
    direction: PacketDirection,
    state: ConnectionState,
    bytes: &[u8],
    _compression: u32,
) -> MyResult<Option<Packet>> {
    let mut reader = Cursor::new(bytes);
    let data_length: VarInt = MinecraftDeserialize::deserialize(&mut reader)?;

    let mut bytes = &bytes[data_length.size()..];

    let mut buffer;
    if data_length.get() != 0 {
        buffer = Vec::new();

        let mut decompresser = ZlibDecoder::new(bytes);
        decompresser.read_to_end(&mut buffer).unwrap();
        bytes = &buffer;
    }

    deserialize_uncompressed(direction, state, bytes)
}

fn deserialize_uncompressed(
    direction: PacketDirection,
    state: ConnectionState,
    bytes: &[u8],
) -> MyResult<Option<Packet>> {
    let mut reader = Cursor::new(bytes);
    let id: VarInt = MinecraftDeserialize::deserialize(&mut reader)?;

    let packet = deserialize(direction, state, id.get(), &mut reader)?;
    Ok(Some(packet))
}

fn has_enough_bytes(bytes: &[u8]) -> bool {
    if let Some(x) = VarInt::deserialize(bytes) {
        bytes.len() >= x.get() as usize + x.size()
    } else {
        false
    }
}
