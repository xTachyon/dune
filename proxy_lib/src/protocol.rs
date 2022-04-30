use crate::de::{MinecraftDeserialize, Reader};
// use crate::game::Gamemode;
use crate::varint::VarInt;
// use crate::PacketDirection;
use anyhow::Result;
use flate2::read::ZlibDecoder;
use num_enum::TryFromPrimitive;
use crate::{PacketDirection, pro};
use crate::pro::Packet;
use std::io::{Cursor, Read};

//
// macro_rules! deserialize_for {
//     ($type:ident $($field:ident)*) => {
//       impl MinecraftDeserialize for $type {
//         fn deserialize<R: Read>(mut _reader: R) -> Result<Self> {
//           $(
//             let $field = MinecraftDeserialize::deserialize(&mut _reader)?;
//           )*
//           let result = $type {
//             $(
//                 $field,
//             )*
//           };
//           Ok(result)
//         }
//       }
//     };
// }
//
#[repr(u8)]
#[derive(Copy, Clone, Debug, TryFromPrimitive)]
pub enum ConnectionState {
    Handshaking = 0,
    Status = 1,
    Login = 2,
    Play = 3,
}

//
// //#[derive(Default)]
// //pub struct PacketHeaderNoCompression {
// //    length: VarInt,
// //    id: VarInt,
// //}
// //
// //deserialize_for!(PacketHeaderNoCompression length id);
//
// #[derive(Debug, Default)]
// pub struct Handshake {
//     pub version: VarInt,
//     pub address: String,
//     pub port: u16,
//     pub next_state: VarInt,
// }
//
// deserialize_for!(Handshake version address port next_state);
//
// #[derive(Debug, Default)]
// pub struct StatusRequest {}
//
// deserialize_for!(StatusRequest);
//
// #[derive(Debug, Default)]
// pub struct StatusResponse {
//     response: String,
// }
//
// deserialize_for!(StatusResponse response);
//
// #[derive(Debug, Default)]
// pub struct SpawnMob {
//     entity_id: VarInt,
//     uuid: u128,
//     entity_type: VarInt,
//     x: f64,
//     y: f64,
//     z: f64,
//     yaw: u8,
//     pitch: u8,
//     velocity_x: u16,
//     velocity_y: u16,
//     velocity_z: u16,
// }
//
// deserialize_for!(SpawnMob entity_id uuid entity_type x y z yaw pitch velocity_x velocity_y velocity_z);
//
// #[derive(Debug, Default)]
// pub struct ChatResponse {
//     pub response: String,
//     pub position: u8,
// }
//
// deserialize_for!(ChatResponse response position);
//
// #[derive(Debug, Default)]
// pub struct SetCompression {
//     pub value: VarInt,
// }
//
// deserialize_for!(SetCompression value);
//
// #[derive(Debug, Default)]
// pub struct LoginSuccess {
//     pub uuid: u128,
//     pub username: String,
// }
//
// deserialize_for!(LoginSuccess uuid username);
//
// #[derive(Debug, Default)]
// pub struct EncRequest {
//     pub server_id: String,
//     pub public_key: Vec<u8>,
//     pub verify_token: Vec<u8>,
// }
//
// deserialize_for!(EncRequest server_id public_key verify_token);
//
// #[derive(Debug, Default)]
// pub struct EncResponse {
//     pub shared_secret: Vec<u8>,
//     pub verify_token: Vec<u8>,
// }
//
// deserialize_for!(EncResponse shared_secret verify_token);
//
// #[derive(Debug, Default)]
// pub struct LoginStart {
//     pub name: String,
// }
//
// deserialize_for!(LoginStart name);
//
// #[derive(Debug)]
// pub enum PlayerInfoTabAction {
//     AddPlayer(
//         String,
//         Vec<(String, String, Option<String>)>,
//         Gamemode,
//         u32,
//         Option<String>,
//     ),
//     Gamemode(u8),
//     Latency(u32),
//     DisplayName(Option<String>),
//     RemovePlayer,
// }
//
// impl PlayerInfoTabAction {
//     fn deserialize<R: Read>(mut reader: R, action: u32) -> Result<(u128, Self)> {
//         let uuid = MinecraftDeserialize::deserialize(&mut reader)?;
//         let result = match action {
//             0 => {
//                 let name = MinecraftDeserialize::deserialize(&mut reader)?;
//                 let number_of_properties =
//                     <VarInt as MinecraftDeserialize>::deserialize(&mut reader)?.get();
//                 let properties: Result<_> = (0..number_of_properties)
//                     .map(|_| MinecraftDeserialize::deserialize(&mut reader))
//                     .collect();
//                 let properties = properties?;
//                 let gamemode = MinecraftDeserialize::deserialize(&mut reader)?;
//                 let ping = <VarInt as MinecraftDeserialize>::deserialize(&mut reader)?.get();
//                 let display_name = MinecraftDeserialize::deserialize(&mut reader)?;
//
//                 PlayerInfoTabAction::AddPlayer(name, properties, gamemode, ping, display_name)
//             }
//             1 => {
//                 let gamemode: VarInt = MinecraftDeserialize::deserialize(&mut reader)?;
//                 PlayerInfoTabAction::Gamemode(gamemode.get() as u8)
//             }
//             2 => {
//                 let ping: VarInt = MinecraftDeserialize::deserialize(&mut reader)?;
//                 PlayerInfoTabAction::Latency(*ping)
//             }
//             3 => {
//                 let name = MinecraftDeserialize::deserialize(&mut reader)?;
//                 PlayerInfoTabAction::DisplayName(name)
//             }
//             4 => PlayerInfoTabAction::RemovePlayer,
//             _ => unreachable!(),
//         };
//         Ok((uuid, result))
//     }
// }
//
// #[derive(Debug)]
// pub struct PlayerInfoTab {
//     actions: Vec<(u128, PlayerInfoTabAction)>,
// }
//
// impl MinecraftDeserialize for PlayerInfoTab {
//     fn deserialize<R: Read>(mut reader: R) -> Result<Self>
//         where
//             Self: Sized,
//     {
//         let action = <VarInt as MinecraftDeserialize>::deserialize(&mut reader)?;
//         assert!(action.get() <= 4);
//         let action = action.get();
//         let count = <VarInt as MinecraftDeserialize>::deserialize(&mut reader)?;
//
//         let actions: Result<_, _> = (0..count.get())
//             .map(|_| PlayerInfoTabAction::deserialize(&mut reader, action))
//             .collect();
//         Ok(PlayerInfoTab { actions: actions? })
//     }
// }
//
// macro_rules! packet_macro {
//     ($($name:ident $state:ident $direction:ident $id:expr)*) => {
//       #[derive(Debug)]
//       pub enum Packet {
//         $(
//           $name($name),
//         )*
//         Unknown(ConnectionState, u32)
//       }
//
//       fn deserialize<R: Read>(direction: PacketDirection, state: ConnectionState, id: u32, mut reader: R) -> Result<Packet> {
//         let result = match (direction, state, id) {
//           $(
//             (PacketDirection::$direction, ConnectionState::$state, $id) => Packet::$name($name::deserialize(&mut reader)?),
//           )*
//           _ => Packet::Unknown(state, id)
//         };
//         Ok(result)
//       }
//     };
// }
//
// packet_macro!(
// Handshake      Handshake ClientToServer 0x00
//
// StatusRequest  Status    ClientToServer 0x00
// StatusResponse Status    ServerToClient 0x00
//
// LoginStart     Login     ClientToServer 0x00
// EncResponse    Login     ClientToServer 0x01
//
// EncRequest     Login     ServerToClient 0x01
// LoginSuccess   Login     ServerToClient 0x02
// SetCompression Login     ServerToClient 0x03
//
// SpawnMob       Play      ServerToClient 0x03
// ChatResponse   Play      ServerToClient 0x0F
// PlayerInfoTab  Play      ServerToClient 0x34
// );
//
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

        let mut decompresser = ZlibDecoder::new(bytes);
        decompresser.read_to_end(&mut buffer)?;
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

    let packet = pro::v1_18_1::de_packets(state, direction, id.get(), reader)?;
    Ok(Some(packet))
}

fn has_enough_bytes(bytes: &[u8]) -> bool {
    if let Some(x) = VarInt::deserialize(bytes) {
        bytes.len() >= x.get() as usize + x.size()
    } else {
        false
    }
}
