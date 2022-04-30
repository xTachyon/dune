pub mod v1_18_1;

use crate::de::{MinecraftDeserialize, Reader};
use crate::varint::{read_varint, VarInt};
use anyhow::Result;
use flate2::read::ZlibDecoder;
use num_enum::TryFromPrimitive;
use std::io::Read;
pub use v1_18_1::de_packets;
pub use v1_18_1::Packet;

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

pub fn deserialize_with_header<'r>(
    direction: PacketDirection,
    state: ConnectionState,
    mut reader: &'r mut Reader<'r>,
    compression: bool,
    tmp: &'r mut Vec<u8>,
) -> Result<Option<(Packet<'r>, usize)>> {
    if !has_enough_bytes(reader.get()) {
        return Ok(None);
    }
    let length: VarInt = MinecraftDeserialize::deserialize(&mut reader)?;

    let packet = if compression {
        deserialize_compressed(direction, state, reader, tmp)
    } else {
        deserialize_uncompressed(direction, state, reader)
    }?;
    let result = match packet {
        Some(packet) => Some((packet, length.get() as usize + length.size())),
        None => None,
    };
    Ok(result)
}

fn deserialize_compressed<'r>(
    direction: PacketDirection,
    state: ConnectionState,
    mut reader: &'r mut Reader<'r>,
    tmp: &'r mut Vec<u8>,
) -> Result<Option<Packet<'r>>> {
    let data_length = read_varint(&mut reader)?;

    if data_length != 0 {
        let mut decompress = ZlibDecoder::new(&mut reader);
        decompress.read_to_end(tmp)?;
        *reader = Reader::new(tmp);
    }

    deserialize_uncompressed(direction, state, reader)
}

fn deserialize_uncompressed<'r>(
    direction: PacketDirection,
    state: ConnectionState,
    mut reader: &'r mut Reader<'r>,
) -> Result<Option<Packet<'r>>> {
    let id = read_varint(&mut reader)?;
    // println!("state={:?}, id={}", state, *id);

    let packet = de_packets(state, direction, id as u32, reader)?;
    Ok(Some(packet))
}

fn has_enough_bytes(bytes: &[u8]) -> bool {
    match VarInt::deserialize(bytes) {
        Some(x) => bytes.len() >= x.get() as usize + x.size(),
        None => false,
    }
}
