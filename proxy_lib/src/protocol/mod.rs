pub mod v1_18_1;

pub use v1_18_1::Packet;
pub use v1_18_1::de_packets;
use crate::varint::VarInt;
use crate::de::{MinecraftDeserialize, Reader};
use std::io::{Cursor, Read};
use flate2::read::ZlibDecoder;
use anyhow::Result;
use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Copy, Clone, Debug, TryFromPrimitive)]
pub enum ConnectionState {
    Handshaking = 0,
    Status = 1,
    Login = 2,
    Play = 3,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PacketDirection {
    ClientToServer,
    ServerToClient,
}

pub fn deserialize_with_header(
    direction: PacketDirection,
    state: ConnectionState,
    bytes: &[u8],
    compression: bool,
) -> Result<Option<(Packet, usize)>> {
    if !has_enough_bytes(bytes) {
        return Ok(None);
    }
    let length: VarInt = MinecraftDeserialize::deserialize(bytes)?;
    let bytes = &bytes[length.size()..length.get() as usize + length.size()];

    let mut reader = Reader { cursor: Cursor::new(bytes) };
    let reader = &mut reader;
    let packet = if compression {
        deserialize_compressed(direction, state, reader)
    } else {
        deserialize_uncompressed(direction, state, reader)
    }?;
    let result = match packet {
        Some(packet) => Some((packet, length.get() as usize + length.size())),
        None => None,
    };
    unimplemented!()
    // Ok(result)
}

fn deserialize_compressed<'r>(
    direction: PacketDirection,
    state: ConnectionState,
    reader: &'r mut Reader<'r>,
) -> Result<Option<Packet<'r>>> {
    let data_length: VarInt = MinecraftDeserialize::deserialize(&mut reader.cursor)?;

    let mut bytes = reader.get_buf_from(data_length.size()..reader.cursor.get_ref().len() as usize)?;

    let mut buffer;
    if *data_length != 0 {
        buffer = Vec::new();

        let mut decompress = ZlibDecoder::new(bytes);
        decompress.read_to_end(&mut buffer)?;
        bytes = &buffer;
    }

    deserialize_uncompressed(direction, state, reader)
}

fn deserialize_uncompressed<'r>(
    direction: PacketDirection,
    state: ConnectionState,
    reader: &'r mut Reader<'r>,
) -> Result<Option<Packet<'r>>> {
    let id: VarInt = MinecraftDeserialize::deserialize(&mut reader.cursor)?;
    println!("state={:?}, id={}", state, *id);

    let packet = v1_18_1::de_packets(state, direction, id.get(), reader)?;
    Ok(Some(packet))
}

fn has_enough_bytes(bytes: &[u8]) -> bool {
    if let Some(x) = VarInt::deserialize(bytes) {
        bytes.len() >= x.get() as usize + x.size()
    } else {
        false
    }
}
