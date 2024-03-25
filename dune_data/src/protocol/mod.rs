pub mod common_states;
pub mod de;
pub mod v1_20_2;
pub mod varint;

use crate::protocol::de::MD;
use crate::protocol::varint::{read_varint, read_varint_with_size};
use anyhow::anyhow;
use anyhow::Result;
use flate2::read::ZlibDecoder;
use num_enum::TryFromPrimitive;
use std::fmt::Debug;
use std::io::Read;
use std::mem::size_of;

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

pub struct PacketData<'x> {
    pub id: PacketId,
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

macro_rules! impl_unaligned_slice_be {
    ($name:ident, $name_it:ident, $t:ty) => {
        #[derive(Copy, Clone)]
        pub struct $name<'x> {
            inner: &'x [[u8; $name::ITEM_SIZE]],
        }
        impl<'x> $name<'x> {
            const ITEM_SIZE: usize = size_of::<$t>();

            fn new(buffer: &'x [u8]) -> Self {
                Self {
                    inner: bytemuck::try_cast_slice(buffer)
                        .expect("unaligned slice should be passed a correct sized buffer"),
                }
            }
        }
        impl<'x> IntoIterator for $name<'x> {
            type Item = $t;
            type IntoIter = $name_it<'x>;

            fn into_iter(self) -> Self::IntoIter {
                $name_it { slice: self }
            }
        }

        pub struct $name_it<'x> {
            slice: $name<'x>,
        }
        impl<'x> Iterator for $name_it<'x> {
            type Item = $t;

            fn next(&mut self) -> Option<Self::Item> {
                match self.slice.inner {
                    [] => None,
                    [first, rest @ ..] => {
                        self.slice.inner = rest;
                        Some(<$t>::from_be_bytes(*first))
                    }
                }
            }
        }
        impl<'x> Debug for $name<'x> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "[{} values x {} bytes]",
                    self.inner.len(),
                    size_of::<$t>()
                )
            }
        }
    };
}
impl_unaligned_slice_be!(UnalignedSliceI64, UnalignedSliceI64Iterator, i64);
impl_unaligned_slice_be!(UnalignedSliceU128, UnalignedSliceU128Iterator, u128);

pub fn read_packet_info<'r>(
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
        id: PacketId(id),
        total_size,
        data: reader,
    };

    Ok(Some(result))
}

fn has_enough_bytes(bytes: &[u8]) -> bool {
    let mut tmp = bytes;
    match read_varint_with_size(&mut tmp) {
        Ok((value, size)) => size + value as usize <= bytes.len(),
        Err(_) => false,
    }
}

#[derive(Clone, Copy)]
pub struct PacketId(pub u32);
impl std::fmt::Display for PacketId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Debug)]
pub enum Handshaking<'x> {
    SetProtocolRequest(common_states::handshaking::SetProtocolRequest<'x>),
}
pub fn handshaking<'r>(
    state: ConnectionState,
    direction: PacketDirection,
    id: PacketId,
    reader: &mut &'r [u8],
) -> Result<Handshaking<'r>> {
    use ConnectionState as S;
    use PacketDirection as D;

    let packet = match (state, direction, id) {
        (S::Handshaking, D::C2S, PacketId(0x0)) => {
            let p = MD::deserialize(reader)?;
            Handshaking::SetProtocolRequest(p)
        }
        _ => {
            return Err(anyhow!("unknown handshaking packet id={}", id));
        }
    };
    Ok(packet)
}

#[derive(Debug)]
pub enum Status<'x> {
    PingStartRequest(common_states::status::PingStartRequest),
    PingRequest(common_states::status::PingRequest),
    ServerInfoResponse(common_states::status::ServerInfoResponse<'x>),
    PingResponse(common_states::status::PingResponse),
}

pub fn status<'r>(
    state: ConnectionState,
    direction: PacketDirection,
    id: PacketId,
    reader: &mut &'r [u8],
) -> Result<Status<'r>> {
    use ConnectionState as S;
    use PacketDirection as D;

    let packet = match (state, direction, id) {
        (S::Status, D::C2S, PacketId(0x0)) => {
            let p = MD::deserialize(reader)?;
            Status::PingStartRequest(p)
        }
        (S::Status, D::C2S, PacketId(0x1)) => {
            let p = MD::deserialize(reader)?;
            Status::PingRequest(p)
        }
        (S::Status, D::S2C, PacketId(0x0)) => {
            let p = MD::deserialize(reader)?;
            Status::ServerInfoResponse(p)
        }
        (S::Status, D::S2C, PacketId(0x1)) => {
            let p = MD::deserialize(reader)?;
            Status::PingResponse(p)
        }
        _ => {
            return Err(anyhow!("unknown status packet id={}", id));
        }
    };
    Ok(packet)
}

#[derive(Debug)]
pub enum Login<'x> {
    LoginStartRequest(common_states::login::LoginStartRequest<'x>),
    EncryptionBeginRequest(common_states::login::EncryptionBeginRequest<'x>),
    LoginPluginResponse(common_states::login::LoginPluginResponse<'x>),
    DisconnectResponse(common_states::login::DisconnectResponse<'x>),
    EncryptionBeginResponse(common_states::login::EncryptionBeginResponse<'x>),
    SuccessResponse(common_states::login::SuccessResponse<'x>),
    CompressResponse(common_states::login::CompressResponse),
    LoginPluginRequest(common_states::login::LoginPluginRequest<'x>),
}
pub fn login<'r>(
    state: ConnectionState,
    direction: PacketDirection,
    id: PacketId,
    reader: &mut &'r [u8],
) -> Result<Login<'r>> {
    use ConnectionState as S;
    use PacketDirection as D;

    let packet = match (state, direction, id) {
        (S::Login, D::C2S, PacketId(0x0)) => {
            let p = MD::deserialize(reader)?;
            Login::LoginStartRequest(p)
        }
        (S::Login, D::C2S, PacketId(0x1)) => {
            let p = MD::deserialize(reader)?;
            Login::EncryptionBeginRequest(p)
        }
        (S::Login, D::C2S, PacketId(0x2)) => {
            let p = MD::deserialize(reader)?;
            Login::LoginPluginResponse(p)
        }
        (S::Login, D::S2C, PacketId(0x0)) => {
            let p = MD::deserialize(reader)?;
            Login::DisconnectResponse(p)
        }
        (S::Login, D::S2C, PacketId(0x1)) => {
            let p = MD::deserialize(reader)?;
            Login::EncryptionBeginResponse(p)
        }
        (S::Login, D::S2C, PacketId(0x2)) => {
            let p = MD::deserialize(reader)?;
            Login::SuccessResponse(p)
        }
        (S::Login, D::S2C, PacketId(0x3)) => {
            let p = MD::deserialize(reader)?;
            Login::CompressResponse(p)
        }
        (S::Login, D::S2C, PacketId(0x4)) => {
            let p = MD::deserialize(reader)?;
            Login::LoginPluginRequest(p)
        }
        _ => {
            return Err(anyhow!(
                "unknown login packet id={},direction={:?}",
                id,
                direction
            ));
        }
    };
    Ok(packet)
}
