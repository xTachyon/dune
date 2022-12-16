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
#[derive(Copy, Clone, Debug, TryFromPrimitive)]
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

pub struct PacketData {
    pub id: u32,
    pub total_size_original: usize,
    pub total_size: usize,
    pub header_size: usize,
    pub compression: bool,
}

impl PacketData {
    pub fn get_data<'b>(&self, normal: &'b [u8], tmp: &'b [u8]) -> &'b [u8] {
        let r = self.header_size..self.total_size;
        if self.compression {
            &tmp[r]
        } else {
            &normal[r]
        }
    }
}

pub fn read_packet_info<'r>(
    buffer: &'r [u8],
    mut compression: bool,
    tmp: &'r mut Vec<u8>,
) -> Result<Option<PacketData>> {
    if !has_enough_bytes(buffer) {
        return Ok(None);
    }
    let mut original = buffer;
    let mut reader = buffer;
    let (length, length_size) = read_varint_with_size(&mut reader)?;

    let total_size_original = length as usize + length_size;
    let mut total_size = total_size_original;
    if compression {
        let data_length = read_varint(&mut reader)?;
        compression = data_length != 0;
        if compression {
            tmp.clear();

            let mut decompress = ZlibDecoder::new(&mut reader);
            decompress.read_to_end(tmp)?;
            reader = &tmp;
            original = reader;
            total_size = tmp.len();
        }
    }

    let id = read_varint(&mut reader)? as u32;
    let result = PacketData {
        id,
        total_size,
        total_size_original,
        header_size: reader.as_ptr() as usize - original.as_ptr() as usize,
        compression,
    };

    Ok(Some(result))
}

pub fn just_deserialize<'r>(
    direction: PacketDirection,
    state: ConnectionState,
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
