pub(crate) mod de;
pub(crate) mod v1_18_2;
pub(crate) mod varint;

use crate::protocol::varint::{read_varint, read_varint_with_size};
use anyhow::Result;
use flate2::read::ZlibDecoder;
use num_enum::TryFromPrimitive;
use std::io::Read;
pub use v1_18_2::de_packets;
pub use v1_18_2::Packet;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum ConnectionState {
    Handshaking = 0,
    Status = 1,
    Login = 2,
    Play = 3,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum PacketDirection {
    C2S,
    S2C,
}

#[derive(Debug)]
pub struct InventorySlotData<'x> {
    pub item_id: i32,
    pub count: u8,
    pub nbt: Option<&'x [u8]>,
}

#[derive(Debug)]
pub struct InventorySlot<'x> {
    pub data: Option<InventorySlotData<'x>>,
}

#[derive(Debug)]
pub struct IndexedOptionNbt<'x> {
    pub nbt: Option<&'x [u8]>,
}
#[derive(Debug)]
pub struct IndexedNbt<'x> {
    pub nbt: &'x [u8],
}

pub(crate) struct PacketData<'x> {
    pub id: u32,
    pub total_size: usize,
    pub data: &'x [u8],
}

#[derive(Debug)]
#[allow(dead_code)] // TODO: remove?
pub struct ChunkBlockEntity<'x> {
    x: u8,
    z: u8,
    y: i16,
    type_: i32,
    nbt_data: IndexedOptionNbt<'x>,
}

pub(crate) fn read_packet_info<'r>(
    buffer: &'r [u8],
    tmp: &'r mut Vec<u8>,
    compression: bool,
) -> Result<Option<PacketData<'r>>> {
    if !has_enough_bytes(buffer) {
        return Ok(None);
    }
    let mut reader = buffer;
    let (length, length_size) = read_varint_with_size(&mut reader)?;

    if compression {
        let data_length = read_varint(&mut reader)?;
        if data_length != 0 {
            tmp.clear();

            let mut decompress = ZlibDecoder::new(&mut reader);
            decompress.read_to_end(tmp)?;
            reader = tmp;
        }
    }

    let total_size = length as usize + length_size;
    let id = read_varint(&mut reader)? as u32;
    let result = PacketData {
        id,
        total_size,
        data: reader,
    };

    Ok(Some(result))
}

pub fn deserialize<'r>(
    state: ConnectionState,
    direction: PacketDirection,
    id: u32,
    reader: &mut &'r [u8],
) -> Result<Packet<'r>> {
    let packet = de_packets(state, direction, id, reader)?;
    Ok(packet)
}

fn has_enough_bytes(bytes: &[u8]) -> bool {
    let mut tmp = bytes;
    match read_varint_with_size(&mut tmp) {
        Ok((value, size)) => size + value as usize <= bytes.len(),
        Err(_) => false,
    }
}
