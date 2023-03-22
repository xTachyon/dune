#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(clippy::needless_borrow)]
// fix
#![allow(unreachable_code)]
#![allow(unused_variables)]
// fix

use super::de::MemoryExt;
use crate::protocol::de::cautious_size;
use crate::protocol::de::Position;
use crate::protocol::de::MD;
use crate::protocol::varint::read_varint;
use crate::protocol::varint::read_varlong;
use crate::protocol::varint::write_varint;
use crate::protocol::varint::write_varlong;
use crate::protocol::ChunkBlockEntity;
use crate::protocol::ConnectionState;
use crate::protocol::IndexedNbt;
use crate::protocol::IndexedOptionNbt;
use crate::protocol::InventorySlot;
use crate::protocol::PacketDirection;
use crate::protocol::UnalignedSliceI64;
use anyhow::{anyhow, Result};
use std::io::{Result as IoResult, Write};
use std::mem::size_of;

pub mod handshaking {
    use super::*;

    #[derive(Debug)]
    pub struct SetProtocolRequest<'p> {
        pub protocol_version: i32,
        pub server_host: &'p str,
        pub server_port: u16,
        pub next_state: i32,
    }
    impl<'p> MD<'p> for SetProtocolRequest<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<SetProtocolRequest<'p>> {
            let protocol_version: i32 = read_varint(&mut reader)?;
            let server_host: &'p str = MD::deserialize(reader)?;
            let server_port: u16 = MD::deserialize(reader)?;
            let next_state: i32 = read_varint(&mut reader)?;

            let result = SetProtocolRequest {
                protocol_version,
                server_host,
                server_port,
                next_state,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x0)?;
            write_varint(&mut writer, self.protocol_version as u32)?;
            self.server_host.serialize(&mut writer)?;
            self.server_port.serialize(&mut writer)?;
            write_varint(&mut writer, self.next_state as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct LegacyServerListPingRequest {
        pub payload: u8,
    }
    impl<'p> MD<'p> for LegacyServerListPingRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<LegacyServerListPingRequest> {
            let payload: u8 = MD::deserialize(reader)?;

            let result = LegacyServerListPingRequest { payload };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0xfe)?;
            self.payload.serialize(&mut writer)?;
            Ok(())
        }
    }
}
pub mod status {
    use super::*;

    #[derive(Debug)]
    pub struct PingStartRequest {}
    impl<'p> MD<'p> for PingStartRequest {
        fn deserialize(mut _reader: &mut &[u8]) -> Result<PingStartRequest> {
            let result = PingStartRequest {};
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x0)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct PingRequest {
        pub time: i64,
    }
    impl<'p> MD<'p> for PingRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<PingRequest> {
            let time: i64 = MD::deserialize(reader)?;

            let result = PingRequest { time };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x1)?;
            self.time.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ServerInfoResponse<'p> {
        pub response: &'p str,
    }
    impl<'p> MD<'p> for ServerInfoResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<ServerInfoResponse<'p>> {
            let response: &'p str = MD::deserialize(reader)?;

            let result = ServerInfoResponse { response };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x0)?;
            self.response.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct PingResponse {
        pub time: i64,
    }
    impl<'p> MD<'p> for PingResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<PingResponse> {
            let time: i64 = MD::deserialize(reader)?;

            let result = PingResponse { time };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x1)?;
            self.time.serialize(&mut writer)?;
            Ok(())
        }
    }
}
pub mod login {
    use super::*;

    #[derive(Debug)]
    pub struct LoginStartRequest<'p> {
        pub username: &'p str,
    }
    impl<'p> MD<'p> for LoginStartRequest<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<LoginStartRequest<'p>> {
            let username: &'p str = MD::deserialize(reader)?;

            let result = LoginStartRequest { username };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x0)?;
            self.username.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EncryptionBeginRequest<'p> {
        pub shared_secret: &'p [u8],
        pub verify_token: &'p [u8],
    }
    impl<'p> MD<'p> for EncryptionBeginRequest<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<EncryptionBeginRequest<'p>> {
            let shared_secret: &'p [u8] = MD::deserialize(reader)?;
            let verify_token: &'p [u8] = MD::deserialize(reader)?;

            let result = EncryptionBeginRequest {
                shared_secret,
                verify_token,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x1)?;
            self.shared_secret.serialize(&mut writer)?;
            self.verify_token.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct LoginPluginResponse<'p> {
        pub message_id: i32,
        pub data: Option<&'p [u8]>,
    }
    impl<'p> MD<'p> for LoginPluginResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<LoginPluginResponse<'p>> {
            let message_id: i32 = read_varint(&mut reader)?;
            let data: Option<&'p [u8]> = MD::deserialize(reader)?;

            let result = LoginPluginResponse { message_id, data };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x2)?;
            write_varint(&mut writer, self.message_id as u32)?;
            self.data.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct DisconnectResponse<'p> {
        pub reason: &'p str,
    }
    impl<'p> MD<'p> for DisconnectResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<DisconnectResponse<'p>> {
            let reason: &'p str = MD::deserialize(reader)?;

            let result = DisconnectResponse { reason };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x0)?;
            self.reason.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EncryptionBeginResponse<'p> {
        pub server_id: &'p str,
        pub public_key: &'p [u8],
        pub verify_token: &'p [u8],
    }
    impl<'p> MD<'p> for EncryptionBeginResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<EncryptionBeginResponse<'p>> {
            let server_id: &'p str = MD::deserialize(reader)?;
            let public_key: &'p [u8] = MD::deserialize(reader)?;
            let verify_token: &'p [u8] = MD::deserialize(reader)?;

            let result = EncryptionBeginResponse {
                server_id,
                public_key,
                verify_token,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x1)?;
            self.server_id.serialize(&mut writer)?;
            self.public_key.serialize(&mut writer)?;
            self.verify_token.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SuccessResponse<'p> {
        pub uuid: u128,
        pub username: &'p str,
    }
    impl<'p> MD<'p> for SuccessResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<SuccessResponse<'p>> {
            let uuid: u128 = MD::deserialize(reader)?;
            let username: &'p str = MD::deserialize(reader)?;

            let result = SuccessResponse { uuid, username };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x2)?;
            self.uuid.serialize(&mut writer)?;
            self.username.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct CompressResponse {
        pub threshold: i32,
    }
    impl<'p> MD<'p> for CompressResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<CompressResponse> {
            let threshold: i32 = read_varint(&mut reader)?;

            let result = CompressResponse { threshold };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x3)?;
            write_varint(&mut writer, self.threshold as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct LoginPluginRequest<'p> {
        pub message_id: i32,
        pub channel: &'p str,
        pub data: &'p [u8],
    }
    impl<'p> MD<'p> for LoginPluginRequest<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<LoginPluginRequest<'p>> {
            let message_id: i32 = read_varint(&mut reader)?;
            let channel: &'p str = MD::deserialize(reader)?;
            let data: &'p [u8] = &reader[..];
            *reader = &[];

            let result = LoginPluginRequest {
                message_id,
                channel,
                data,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x4)?;
            write_varint(&mut writer, self.message_id as u32)?;
            self.channel.serialize(&mut writer)?;
            writer.write_all(self.data)?;
            Ok(())
        }
    }
}
pub mod play {
    use super::*;

    #[derive(Debug)]
    pub struct TeleportConfirmRequest {
        pub teleport_id: i32,
    }
    impl<'p> MD<'p> for TeleportConfirmRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<TeleportConfirmRequest> {
            let teleport_id: i32 = read_varint(&mut reader)?;

            let result = TeleportConfirmRequest { teleport_id };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x0)?;
            write_varint(&mut writer, self.teleport_id as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct QueryBlockNbtRequest {
        pub transaction_id: i32,
        pub location: Position,
    }
    impl<'p> MD<'p> for QueryBlockNbtRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<QueryBlockNbtRequest> {
            let transaction_id: i32 = read_varint(&mut reader)?;
            let location: Position = MD::deserialize(reader)?;

            let result = QueryBlockNbtRequest {
                transaction_id,
                location,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x1)?;
            write_varint(&mut writer, self.transaction_id as u32)?;
            self.location.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SetDifficultyRequest {
        pub new_difficulty: u8,
    }
    impl<'p> MD<'p> for SetDifficultyRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<SetDifficultyRequest> {
            let new_difficulty: u8 = MD::deserialize(reader)?;

            let result = SetDifficultyRequest { new_difficulty };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x2)?;
            self.new_difficulty.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EditBookRequest<'p> {
        pub hand: i32,
        pub pages: Vec<&'p str>,
        pub title: Option<&'p str>,
    }
    impl<'p> MD<'p> for EditBookRequest<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<EditBookRequest<'p>> {
            let hand: i32 = read_varint(&mut reader)?;
            let array_count: i32 = read_varint(&mut reader)?;
            let mut pages = Vec::with_capacity(cautious_size(array_count as usize));
            for _ in 0..array_count {
                let x: &'p str = MD::deserialize(reader)?;
                pages.push(x);
            }
            let title: Option<&'p str> = MD::deserialize(reader)?;

            let result = EditBookRequest { hand, pages, title };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut _writer: &mut W) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct QueryEntityNbtRequest {
        pub transaction_id: i32,
        pub entity_id: i32,
    }
    impl<'p> MD<'p> for QueryEntityNbtRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<QueryEntityNbtRequest> {
            let transaction_id: i32 = read_varint(&mut reader)?;
            let entity_id: i32 = read_varint(&mut reader)?;

            let result = QueryEntityNbtRequest {
                transaction_id,
                entity_id,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0xc)?;
            write_varint(&mut writer, self.transaction_id as u32)?;
            write_varint(&mut writer, self.entity_id as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct PickItemRequest {
        pub slot: i32,
    }
    impl<'p> MD<'p> for PickItemRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<PickItemRequest> {
            let slot: i32 = read_varint(&mut reader)?;

            let result = PickItemRequest { slot };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x17)?;
            write_varint(&mut writer, self.slot as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct NameItemRequest<'p> {
        pub name: &'p str,
    }
    impl<'p> MD<'p> for NameItemRequest<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<NameItemRequest<'p>> {
            let name: &'p str = MD::deserialize(reader)?;

            let result = NameItemRequest { name };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x20)?;
            self.name.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SelectTradeRequest {
        pub slot: i32,
    }
    impl<'p> MD<'p> for SelectTradeRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<SelectTradeRequest> {
            let slot: i32 = read_varint(&mut reader)?;

            let result = SelectTradeRequest { slot };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x23)?;
            write_varint(&mut writer, self.slot as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SetBeaconEffectRequest {
        pub primary_effect: i32,
        pub secondary_effect: i32,
    }
    impl<'p> MD<'p> for SetBeaconEffectRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<SetBeaconEffectRequest> {
            let primary_effect: i32 = read_varint(&mut reader)?;
            let secondary_effect: i32 = read_varint(&mut reader)?;

            let result = SetBeaconEffectRequest {
                primary_effect,
                secondary_effect,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x24)?;
            write_varint(&mut writer, self.primary_effect as u32)?;
            write_varint(&mut writer, self.secondary_effect as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UpdateCommandBlockRequest<'p> {
        pub location: Position,
        pub command: &'p str,
        pub mode: i32,
        pub flags: u8,
    }
    impl<'p> MD<'p> for UpdateCommandBlockRequest<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<UpdateCommandBlockRequest<'p>> {
            let location: Position = MD::deserialize(reader)?;
            let command: &'p str = MD::deserialize(reader)?;
            let mode: i32 = read_varint(&mut reader)?;
            let flags: u8 = MD::deserialize(reader)?;

            let result = UpdateCommandBlockRequest {
                location,
                command,
                mode,
                flags,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x26)?;
            self.location.serialize(&mut writer)?;
            self.command.serialize(&mut writer)?;
            write_varint(&mut writer, self.mode as u32)?;
            self.flags.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UpdateCommandBlockMinecartRequest<'p> {
        pub entity_id: i32,
        pub command: &'p str,
        pub track_output: bool,
    }
    impl<'p> MD<'p> for UpdateCommandBlockMinecartRequest<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<UpdateCommandBlockMinecartRequest<'p>> {
            let entity_id: i32 = read_varint(&mut reader)?;
            let command: &'p str = MD::deserialize(reader)?;
            let track_output: bool = MD::deserialize(reader)?;

            let result = UpdateCommandBlockMinecartRequest {
                entity_id,
                command,
                track_output,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x27)?;
            write_varint(&mut writer, self.entity_id as u32)?;
            self.command.serialize(&mut writer)?;
            self.track_output.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UpdateStructureBlockRequest<'p> {
        pub location: Position,
        pub action: i32,
        pub mode: i32,
        pub name: &'p str,
        pub offset_x: i8,
        pub offset_y: i8,
        pub offset_z: i8,
        pub size_x: i8,
        pub size_y: i8,
        pub size_z: i8,
        pub mirror: i32,
        pub rotation: i32,
        pub metadata: &'p str,
        pub integrity: f32,
        pub seed: i64,
        pub flags: u8,
    }
    impl<'p> MD<'p> for UpdateStructureBlockRequest<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<UpdateStructureBlockRequest<'p>> {
            let location: Position = MD::deserialize(reader)?;
            let action: i32 = read_varint(&mut reader)?;
            let mode: i32 = read_varint(&mut reader)?;
            let name: &'p str = MD::deserialize(reader)?;
            let offset_x: i8 = MD::deserialize(reader)?;
            let offset_y: i8 = MD::deserialize(reader)?;
            let offset_z: i8 = MD::deserialize(reader)?;
            let size_x: i8 = MD::deserialize(reader)?;
            let size_y: i8 = MD::deserialize(reader)?;
            let size_z: i8 = MD::deserialize(reader)?;
            let mirror: i32 = read_varint(&mut reader)?;
            let rotation: i32 = read_varint(&mut reader)?;
            let metadata: &'p str = MD::deserialize(reader)?;
            let integrity: f32 = MD::deserialize(reader)?;
            let seed: i64 = read_varlong(&mut reader)?;
            let flags: u8 = MD::deserialize(reader)?;

            let result = UpdateStructureBlockRequest {
                location,
                action,
                mode,
                name,
                offset_x,
                offset_y,
                offset_z,
                size_x,
                size_y,
                size_z,
                mirror,
                rotation,
                metadata,
                integrity,
                seed,
                flags,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x2a)?;
            self.location.serialize(&mut writer)?;
            write_varint(&mut writer, self.action as u32)?;
            write_varint(&mut writer, self.mode as u32)?;
            self.name.serialize(&mut writer)?;
            self.offset_x.serialize(&mut writer)?;
            self.offset_y.serialize(&mut writer)?;
            self.offset_z.serialize(&mut writer)?;
            self.size_x.serialize(&mut writer)?;
            self.size_y.serialize(&mut writer)?;
            self.size_z.serialize(&mut writer)?;
            write_varint(&mut writer, self.mirror as u32)?;
            write_varint(&mut writer, self.rotation as u32)?;
            self.metadata.serialize(&mut writer)?;
            self.integrity.serialize(&mut writer)?;
            write_varlong(&mut writer, self.seed as u64)?;
            self.flags.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct TabCompleteRequest<'p> {
        pub transaction_id: i32,
        pub text: &'p str,
    }
    impl<'p> MD<'p> for TabCompleteRequest<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<TabCompleteRequest<'p>> {
            let transaction_id: i32 = read_varint(&mut reader)?;
            let text: &'p str = MD::deserialize(reader)?;

            let result = TabCompleteRequest {
                transaction_id,
                text,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x6)?;
            write_varint(&mut writer, self.transaction_id as u32)?;
            self.text.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ChatRequest<'p> {
        pub message: &'p str,
    }
    impl<'p> MD<'p> for ChatRequest<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<ChatRequest<'p>> {
            let message: &'p str = MD::deserialize(reader)?;

            let result = ChatRequest { message };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x3)?;
            self.message.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ClientCommandRequest {
        pub action_id: i32,
    }
    impl<'p> MD<'p> for ClientCommandRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<ClientCommandRequest> {
            let action_id: i32 = read_varint(&mut reader)?;

            let result = ClientCommandRequest { action_id };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x4)?;
            write_varint(&mut writer, self.action_id as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SettingsRequest<'p> {
        pub locale: &'p str,
        pub view_distance: i8,
        pub chat_flags: i32,
        pub chat_colors: bool,
        pub skin_parts: u8,
        pub main_hand: i32,
        pub enable_text_filtering: bool,
        pub enable_server_listing: bool,
    }
    impl<'p> MD<'p> for SettingsRequest<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<SettingsRequest<'p>> {
            let locale: &'p str = MD::deserialize(reader)?;
            let view_distance: i8 = MD::deserialize(reader)?;
            let chat_flags: i32 = read_varint(&mut reader)?;
            let chat_colors: bool = MD::deserialize(reader)?;
            let skin_parts: u8 = MD::deserialize(reader)?;
            let main_hand: i32 = read_varint(&mut reader)?;
            let enable_text_filtering: bool = MD::deserialize(reader)?;
            let enable_server_listing: bool = MD::deserialize(reader)?;

            let result = SettingsRequest {
                locale,
                view_distance,
                chat_flags,
                chat_colors,
                skin_parts,
                main_hand,
                enable_text_filtering,
                enable_server_listing,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x5)?;
            self.locale.serialize(&mut writer)?;
            self.view_distance.serialize(&mut writer)?;
            write_varint(&mut writer, self.chat_flags as u32)?;
            self.chat_colors.serialize(&mut writer)?;
            self.skin_parts.serialize(&mut writer)?;
            write_varint(&mut writer, self.main_hand as u32)?;
            self.enable_text_filtering.serialize(&mut writer)?;
            self.enable_server_listing.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EnchantItemRequest {
        pub window_id: i8,
        pub enchantment: i8,
    }
    impl<'p> MD<'p> for EnchantItemRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<EnchantItemRequest> {
            let window_id: i8 = MD::deserialize(reader)?;
            let enchantment: i8 = MD::deserialize(reader)?;

            let result = EnchantItemRequest {
                window_id,
                enchantment,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x7)?;
            self.window_id.serialize(&mut writer)?;
            self.enchantment.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct WindowClickRequest_ChangedSlots<'p> {
        pub location: i16,
        pub item: InventorySlot<'p>,
    }
    impl<'p> MD<'p> for WindowClickRequest_ChangedSlots<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<WindowClickRequest_ChangedSlots<'p>> {
            let location: i16 = MD::deserialize(reader)?;
            let item: InventorySlot<'p> = MD::deserialize(reader)?;

            let result = WindowClickRequest_ChangedSlots { location, item };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            self.location.serialize(&mut writer)?;
            self.item.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct WindowClickRequest<'p> {
        pub window_id: u8,
        pub state_id: i32,
        pub slot: i16,
        pub mouse_button: i8,
        pub mode: i32,
        pub changed_slots: Vec<WindowClickRequest_ChangedSlots<'p>>,
        pub cursor_item: InventorySlot<'p>,
    }
    impl<'p> MD<'p> for WindowClickRequest<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<WindowClickRequest<'p>> {
            let window_id: u8 = MD::deserialize(reader)?;
            let state_id: i32 = read_varint(&mut reader)?;
            let slot: i16 = MD::deserialize(reader)?;
            let mouse_button: i8 = MD::deserialize(reader)?;
            let mode: i32 = read_varint(&mut reader)?;
            let array_count: i32 = read_varint(&mut reader)?;
            let mut changed_slots = Vec::with_capacity(cautious_size(array_count as usize));
            for _ in 0..array_count {
                let x: WindowClickRequest_ChangedSlots =
                    WindowClickRequest_ChangedSlots::deserialize(reader)?;
                changed_slots.push(x);
            }
            let cursor_item: InventorySlot<'p> = MD::deserialize(reader)?;

            let result = WindowClickRequest {
                window_id,
                state_id,
                slot,
                mouse_button,
                mode,
                changed_slots,
                cursor_item,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut _writer: &mut W) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct CloseWindowRequest {
        pub window_id: u8,
    }
    impl<'p> MD<'p> for CloseWindowRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<CloseWindowRequest> {
            let window_id: u8 = MD::deserialize(reader)?;

            let result = CloseWindowRequest { window_id };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x9)?;
            self.window_id.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct CustomPayloadRequest<'p> {
        pub channel: &'p str,
        pub data: &'p [u8],
    }
    impl<'p> MD<'p> for CustomPayloadRequest<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<CustomPayloadRequest<'p>> {
            let channel: &'p str = MD::deserialize(reader)?;
            let data: &'p [u8] = &reader[..];
            *reader = &[];

            let result = CustomPayloadRequest { channel, data };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0xa)?;
            self.channel.serialize(&mut writer)?;
            writer.write_all(self.data)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct Coords {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }
    #[derive(Debug)]
    pub enum UseEntityKind {
        Interact,
        Attack,
        InteractAt(Coords),
    }

    #[derive(Debug)]
    pub struct UseEntityRequest {
        pub entity_id: i32,
        pub kind: UseEntityKind,
        pub sneaking: bool,
    }

    impl<'p> MD<'p> for UseEntityRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<UseEntityRequest> {
            let entity_id = read_varint(&mut reader)?;
            let kind = read_varint(&mut reader)?;
            let kind = match kind {
                0 => {
                    let _ = read_varint(&mut reader)?;
                    UseEntityKind::Interact
                }
                1 => UseEntityKind::Attack,
                2 => {
                    let x = MD::deserialize(&mut reader)?;
                    let y = MD::deserialize(&mut reader)?;
                    let z = MD::deserialize(&mut reader)?;
                    let _ = read_varint(&mut reader)?;

                    UseEntityKind::InteractAt(Coords { x, y, z })
                }
                _ => anyhow::bail!("unknown use entity kind {}", kind),
            };
            let sneaking = MD::deserialize(&mut reader)?;

            Ok(UseEntityRequest {
                entity_id,
                kind,
                sneaking,
            })
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            unimplemented!()
        }
    }
    #[derive(Debug)]
    pub struct GenerateStructureRequest {
        pub location: Position,
        pub levels: i32,
        pub keep_jigsaws: bool,
    }
    impl<'p> MD<'p> for GenerateStructureRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<GenerateStructureRequest> {
            let location: Position = MD::deserialize(reader)?;
            let levels: i32 = read_varint(&mut reader)?;
            let keep_jigsaws: bool = MD::deserialize(reader)?;

            let result = GenerateStructureRequest {
                location,
                levels,
                keep_jigsaws,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0xe)?;
            self.location.serialize(&mut writer)?;
            write_varint(&mut writer, self.levels as u32)?;
            self.keep_jigsaws.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct KeepAliveRequest {
        pub keep_alive_id: i64,
    }
    impl<'p> MD<'p> for KeepAliveRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<KeepAliveRequest> {
            let keep_alive_id: i64 = MD::deserialize(reader)?;

            let result = KeepAliveRequest { keep_alive_id };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0xf)?;
            self.keep_alive_id.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct LockDifficultyRequest {
        pub locked: bool,
    }
    impl<'p> MD<'p> for LockDifficultyRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<LockDifficultyRequest> {
            let locked: bool = MD::deserialize(reader)?;

            let result = LockDifficultyRequest { locked };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x10)?;
            self.locked.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct PositionRequest {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub on_ground: bool,
    }
    impl<'p> MD<'p> for PositionRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<PositionRequest> {
            let x: f64 = MD::deserialize(reader)?;
            let y: f64 = MD::deserialize(reader)?;
            let z: f64 = MD::deserialize(reader)?;
            let on_ground: bool = MD::deserialize(reader)?;

            let result = PositionRequest { x, y, z, on_ground };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x11)?;
            self.x.serialize(&mut writer)?;
            self.y.serialize(&mut writer)?;
            self.z.serialize(&mut writer)?;
            self.on_ground.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct PositionLookRequest {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub yaw: f32,
        pub pitch: f32,
        pub on_ground: bool,
    }
    impl<'p> MD<'p> for PositionLookRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<PositionLookRequest> {
            let x: f64 = MD::deserialize(reader)?;
            let y: f64 = MD::deserialize(reader)?;
            let z: f64 = MD::deserialize(reader)?;
            let yaw: f32 = MD::deserialize(reader)?;
            let pitch: f32 = MD::deserialize(reader)?;
            let on_ground: bool = MD::deserialize(reader)?;

            let result = PositionLookRequest {
                x,
                y,
                z,
                yaw,
                pitch,
                on_ground,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x12)?;
            self.x.serialize(&mut writer)?;
            self.y.serialize(&mut writer)?;
            self.z.serialize(&mut writer)?;
            self.yaw.serialize(&mut writer)?;
            self.pitch.serialize(&mut writer)?;
            self.on_ground.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct LookRequest {
        pub yaw: f32,
        pub pitch: f32,
        pub on_ground: bool,
    }
    impl<'p> MD<'p> for LookRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<LookRequest> {
            let yaw: f32 = MD::deserialize(reader)?;
            let pitch: f32 = MD::deserialize(reader)?;
            let on_ground: bool = MD::deserialize(reader)?;

            let result = LookRequest {
                yaw,
                pitch,
                on_ground,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x13)?;
            self.yaw.serialize(&mut writer)?;
            self.pitch.serialize(&mut writer)?;
            self.on_ground.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct FlyingRequest {
        pub on_ground: bool,
    }
    impl<'p> MD<'p> for FlyingRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<FlyingRequest> {
            let on_ground: bool = MD::deserialize(reader)?;

            let result = FlyingRequest { on_ground };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x14)?;
            self.on_ground.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct VehicleMoveRequest {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub yaw: f32,
        pub pitch: f32,
    }
    impl<'p> MD<'p> for VehicleMoveRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<VehicleMoveRequest> {
            let x: f64 = MD::deserialize(reader)?;
            let y: f64 = MD::deserialize(reader)?;
            let z: f64 = MD::deserialize(reader)?;
            let yaw: f32 = MD::deserialize(reader)?;
            let pitch: f32 = MD::deserialize(reader)?;

            let result = VehicleMoveRequest {
                x,
                y,
                z,
                yaw,
                pitch,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x15)?;
            self.x.serialize(&mut writer)?;
            self.y.serialize(&mut writer)?;
            self.z.serialize(&mut writer)?;
            self.yaw.serialize(&mut writer)?;
            self.pitch.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SteerBoatRequest {
        pub left_paddle: bool,
        pub right_paddle: bool,
    }
    impl<'p> MD<'p> for SteerBoatRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<SteerBoatRequest> {
            let left_paddle: bool = MD::deserialize(reader)?;
            let right_paddle: bool = MD::deserialize(reader)?;

            let result = SteerBoatRequest {
                left_paddle,
                right_paddle,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x16)?;
            self.left_paddle.serialize(&mut writer)?;
            self.right_paddle.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct CraftRecipeRequest<'p> {
        pub window_id: i8,
        pub recipe: &'p str,
        pub make_all: bool,
    }
    impl<'p> MD<'p> for CraftRecipeRequest<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<CraftRecipeRequest<'p>> {
            let window_id: i8 = MD::deserialize(reader)?;
            let recipe: &'p str = MD::deserialize(reader)?;
            let make_all: bool = MD::deserialize(reader)?;

            let result = CraftRecipeRequest {
                window_id,
                recipe,
                make_all,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x18)?;
            self.window_id.serialize(&mut writer)?;
            self.recipe.serialize(&mut writer)?;
            self.make_all.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct AbilitiesRequest {
        pub flags: i8,
    }
    impl<'p> MD<'p> for AbilitiesRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<AbilitiesRequest> {
            let flags: i8 = MD::deserialize(reader)?;

            let result = AbilitiesRequest { flags };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x19)?;
            self.flags.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct BlockDigRequest {
        pub status: i32,
        pub location: Position,
        pub face: i8,
    }
    impl<'p> MD<'p> for BlockDigRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<BlockDigRequest> {
            let status: i32 = read_varint(&mut reader)?;
            let location: Position = MD::deserialize(reader)?;
            let face: i8 = MD::deserialize(reader)?;

            let result = BlockDigRequest {
                status,
                location,
                face,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x1a)?;
            write_varint(&mut writer, self.status as u32)?;
            self.location.serialize(&mut writer)?;
            self.face.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EntityActionRequest {
        pub entity_id: i32,
        pub action_id: i32,
        pub jump_boost: i32,
    }
    impl<'p> MD<'p> for EntityActionRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<EntityActionRequest> {
            let entity_id: i32 = read_varint(&mut reader)?;
            let action_id: i32 = read_varint(&mut reader)?;
            let jump_boost: i32 = read_varint(&mut reader)?;

            let result = EntityActionRequest {
                entity_id,
                action_id,
                jump_boost,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x1b)?;
            write_varint(&mut writer, self.entity_id as u32)?;
            write_varint(&mut writer, self.action_id as u32)?;
            write_varint(&mut writer, self.jump_boost as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SteerVehicleRequest {
        pub sideways: f32,
        pub forward: f32,
        pub jump: u8,
    }
    impl<'p> MD<'p> for SteerVehicleRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<SteerVehicleRequest> {
            let sideways: f32 = MD::deserialize(reader)?;
            let forward: f32 = MD::deserialize(reader)?;
            let jump: u8 = MD::deserialize(reader)?;

            let result = SteerVehicleRequest {
                sideways,
                forward,
                jump,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x1c)?;
            self.sideways.serialize(&mut writer)?;
            self.forward.serialize(&mut writer)?;
            self.jump.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct DisplayedRecipeRequest<'p> {
        pub recipe_id: &'p str,
    }
    impl<'p> MD<'p> for DisplayedRecipeRequest<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<DisplayedRecipeRequest<'p>> {
            let recipe_id: &'p str = MD::deserialize(reader)?;

            let result = DisplayedRecipeRequest { recipe_id };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x1f)?;
            self.recipe_id.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct RecipeBookRequest {
        pub book_id: i32,
        pub book_open: bool,
        pub filter_active: bool,
    }
    impl<'p> MD<'p> for RecipeBookRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<RecipeBookRequest> {
            let book_id: i32 = read_varint(&mut reader)?;
            let book_open: bool = MD::deserialize(reader)?;
            let filter_active: bool = MD::deserialize(reader)?;

            let result = RecipeBookRequest {
                book_id,
                book_open,
                filter_active,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x1e)?;
            write_varint(&mut writer, self.book_id as u32)?;
            self.book_open.serialize(&mut writer)?;
            self.filter_active.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ResourcePackReceiveRequest {
        pub result: i32,
    }
    impl<'p> MD<'p> for ResourcePackReceiveRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<ResourcePackReceiveRequest> {
            let result: i32 = read_varint(&mut reader)?;

            let result = ResourcePackReceiveRequest { result };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x21)?;
            write_varint(&mut writer, self.result as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct HeldItemSlotRequest {
        pub slot_id: i16,
    }
    impl<'p> MD<'p> for HeldItemSlotRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<HeldItemSlotRequest> {
            let slot_id: i16 = MD::deserialize(reader)?;

            let result = HeldItemSlotRequest { slot_id };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x25)?;
            self.slot_id.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SetCreativeSlotRequest<'p> {
        pub slot: i16,
        pub item: InventorySlot<'p>,
    }
    impl<'p> MD<'p> for SetCreativeSlotRequest<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<SetCreativeSlotRequest<'p>> {
            let slot: i16 = MD::deserialize(reader)?;
            let item: InventorySlot<'p> = MD::deserialize(reader)?;

            let result = SetCreativeSlotRequest { slot, item };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x28)?;
            self.slot.serialize(&mut writer)?;
            self.item.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UpdateJigsawBlockRequest<'p> {
        pub location: Position,
        pub name: &'p str,
        pub target: &'p str,
        pub pool: &'p str,
        pub final_state: &'p str,
        pub joint_type: &'p str,
    }
    impl<'p> MD<'p> for UpdateJigsawBlockRequest<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<UpdateJigsawBlockRequest<'p>> {
            let location: Position = MD::deserialize(reader)?;
            let name: &'p str = MD::deserialize(reader)?;
            let target: &'p str = MD::deserialize(reader)?;
            let pool: &'p str = MD::deserialize(reader)?;
            let final_state: &'p str = MD::deserialize(reader)?;
            let joint_type: &'p str = MD::deserialize(reader)?;

            let result = UpdateJigsawBlockRequest {
                location,
                name,
                target,
                pool,
                final_state,
                joint_type,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x29)?;
            self.location.serialize(&mut writer)?;
            self.name.serialize(&mut writer)?;
            self.target.serialize(&mut writer)?;
            self.pool.serialize(&mut writer)?;
            self.final_state.serialize(&mut writer)?;
            self.joint_type.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UpdateSignRequest<'p> {
        pub location: Position,
        pub text_1: &'p str,
        pub text_2: &'p str,
        pub text_3: &'p str,
        pub text_4: &'p str,
    }
    impl<'p> MD<'p> for UpdateSignRequest<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<UpdateSignRequest<'p>> {
            let location: Position = MD::deserialize(reader)?;
            let text_1: &'p str = MD::deserialize(reader)?;
            let text_2: &'p str = MD::deserialize(reader)?;
            let text_3: &'p str = MD::deserialize(reader)?;
            let text_4: &'p str = MD::deserialize(reader)?;

            let result = UpdateSignRequest {
                location,
                text_1,
                text_2,
                text_3,
                text_4,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x2b)?;
            self.location.serialize(&mut writer)?;
            self.text_1.serialize(&mut writer)?;
            self.text_2.serialize(&mut writer)?;
            self.text_3.serialize(&mut writer)?;
            self.text_4.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ArmAnimationRequest {
        pub hand: i32,
    }
    impl<'p> MD<'p> for ArmAnimationRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<ArmAnimationRequest> {
            let hand: i32 = read_varint(&mut reader)?;

            let result = ArmAnimationRequest { hand };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x2c)?;
            write_varint(&mut writer, self.hand as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SpectateRequest {
        pub target: u128,
    }
    impl<'p> MD<'p> for SpectateRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<SpectateRequest> {
            let target: u128 = MD::deserialize(reader)?;

            let result = SpectateRequest { target };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x2d)?;
            self.target.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct BlockPlaceRequest {
        pub hand: i32,
        pub location: Position,
        pub direction: i32,
        pub cursor_x: f32,
        pub cursor_y: f32,
        pub cursor_z: f32,
        pub inside_block: bool,
    }
    impl<'p> MD<'p> for BlockPlaceRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<BlockPlaceRequest> {
            let hand: i32 = read_varint(&mut reader)?;
            let location: Position = MD::deserialize(reader)?;
            let direction: i32 = read_varint(&mut reader)?;
            let cursor_x: f32 = MD::deserialize(reader)?;
            let cursor_y: f32 = MD::deserialize(reader)?;
            let cursor_z: f32 = MD::deserialize(reader)?;
            let inside_block: bool = MD::deserialize(reader)?;

            let result = BlockPlaceRequest {
                hand,
                location,
                direction,
                cursor_x,
                cursor_y,
                cursor_z,
                inside_block,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x2e)?;
            write_varint(&mut writer, self.hand as u32)?;
            self.location.serialize(&mut writer)?;
            write_varint(&mut writer, self.direction as u32)?;
            self.cursor_x.serialize(&mut writer)?;
            self.cursor_y.serialize(&mut writer)?;
            self.cursor_z.serialize(&mut writer)?;
            self.inside_block.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UseItemRequest {
        pub hand: i32,
    }
    impl<'p> MD<'p> for UseItemRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<UseItemRequest> {
            let hand: i32 = read_varint(&mut reader)?;

            let result = UseItemRequest { hand };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x2f)?;
            write_varint(&mut writer, self.hand as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct AdvancementTabRequest {}
    impl<'p> MD<'p> for AdvancementTabRequest {
        fn deserialize(mut _reader: &mut &[u8]) -> Result<AdvancementTabRequest> {
            // failed

            let result = AdvancementTabRequest {};
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x22)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct PongRequest {
        pub id: i32,
    }
    impl<'p> MD<'p> for PongRequest {
        fn deserialize(mut reader: &mut &[u8]) -> Result<PongRequest> {
            let id: i32 = MD::deserialize(reader)?;

            let result = PongRequest { id };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x1d)?;
            self.id.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SpawnEntityResponse {
        pub entity_id: i32,
        pub object_uuid: u128,
        pub type_: i32,
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub pitch: i8,
        pub yaw: i8,
        pub object_data: i32,
        pub velocity_x: i16,
        pub velocity_y: i16,
        pub velocity_z: i16,
    }
    impl<'p> MD<'p> for SpawnEntityResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<SpawnEntityResponse> {
            let entity_id: i32 = read_varint(&mut reader)?;
            let object_uuid: u128 = MD::deserialize(reader)?;
            let type_: i32 = read_varint(&mut reader)?;
            let x: f64 = MD::deserialize(reader)?;
            let y: f64 = MD::deserialize(reader)?;
            let z: f64 = MD::deserialize(reader)?;
            let pitch: i8 = MD::deserialize(reader)?;
            let yaw: i8 = MD::deserialize(reader)?;
            let object_data: i32 = MD::deserialize(reader)?;
            let velocity_x: i16 = MD::deserialize(reader)?;
            let velocity_y: i16 = MD::deserialize(reader)?;
            let velocity_z: i16 = MD::deserialize(reader)?;

            let result = SpawnEntityResponse {
                entity_id,
                object_uuid,
                type_,
                x,
                y,
                z,
                pitch,
                yaw,
                object_data,
                velocity_x,
                velocity_y,
                velocity_z,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x0)?;
            write_varint(&mut writer, self.entity_id as u32)?;
            self.object_uuid.serialize(&mut writer)?;
            write_varint(&mut writer, self.type_ as u32)?;
            self.x.serialize(&mut writer)?;
            self.y.serialize(&mut writer)?;
            self.z.serialize(&mut writer)?;
            self.pitch.serialize(&mut writer)?;
            self.yaw.serialize(&mut writer)?;
            self.object_data.serialize(&mut writer)?;
            self.velocity_x.serialize(&mut writer)?;
            self.velocity_y.serialize(&mut writer)?;
            self.velocity_z.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SpawnEntityExperienceOrbResponse {
        pub entity_id: i32,
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub count: i16,
    }
    impl<'p> MD<'p> for SpawnEntityExperienceOrbResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<SpawnEntityExperienceOrbResponse> {
            let entity_id: i32 = read_varint(&mut reader)?;
            let x: f64 = MD::deserialize(reader)?;
            let y: f64 = MD::deserialize(reader)?;
            let z: f64 = MD::deserialize(reader)?;
            let count: i16 = MD::deserialize(reader)?;

            let result = SpawnEntityExperienceOrbResponse {
                entity_id,
                x,
                y,
                z,
                count,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x1)?;
            write_varint(&mut writer, self.entity_id as u32)?;
            self.x.serialize(&mut writer)?;
            self.y.serialize(&mut writer)?;
            self.z.serialize(&mut writer)?;
            self.count.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SpawnEntityLivingResponse {
        pub entity_id: i32,
        pub entity_uuid: u128,
        pub type_: i32,
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub yaw: i8,
        pub pitch: i8,
        pub head_pitch: i8,
        pub velocity_x: i16,
        pub velocity_y: i16,
        pub velocity_z: i16,
    }
    impl<'p> MD<'p> for SpawnEntityLivingResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<SpawnEntityLivingResponse> {
            let entity_id: i32 = read_varint(&mut reader)?;
            let entity_uuid: u128 = MD::deserialize(reader)?;
            let type_: i32 = read_varint(&mut reader)?;
            let x: f64 = MD::deserialize(reader)?;
            let y: f64 = MD::deserialize(reader)?;
            let z: f64 = MD::deserialize(reader)?;
            let yaw: i8 = MD::deserialize(reader)?;
            let pitch: i8 = MD::deserialize(reader)?;
            let head_pitch: i8 = MD::deserialize(reader)?;
            let velocity_x: i16 = MD::deserialize(reader)?;
            let velocity_y: i16 = MD::deserialize(reader)?;
            let velocity_z: i16 = MD::deserialize(reader)?;

            let result = SpawnEntityLivingResponse {
                entity_id,
                entity_uuid,
                type_,
                x,
                y,
                z,
                yaw,
                pitch,
                head_pitch,
                velocity_x,
                velocity_y,
                velocity_z,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x2)?;
            write_varint(&mut writer, self.entity_id as u32)?;
            self.entity_uuid.serialize(&mut writer)?;
            write_varint(&mut writer, self.type_ as u32)?;
            self.x.serialize(&mut writer)?;
            self.y.serialize(&mut writer)?;
            self.z.serialize(&mut writer)?;
            self.yaw.serialize(&mut writer)?;
            self.pitch.serialize(&mut writer)?;
            self.head_pitch.serialize(&mut writer)?;
            self.velocity_x.serialize(&mut writer)?;
            self.velocity_y.serialize(&mut writer)?;
            self.velocity_z.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SpawnEntityPaintingResponse {
        pub entity_id: i32,
        pub entity_uuid: u128,
        pub title: i32,
        pub location: Position,
        pub direction: u8,
    }
    impl<'p> MD<'p> for SpawnEntityPaintingResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<SpawnEntityPaintingResponse> {
            let entity_id: i32 = read_varint(&mut reader)?;
            let entity_uuid: u128 = MD::deserialize(reader)?;
            let title: i32 = read_varint(&mut reader)?;
            let location: Position = MD::deserialize(reader)?;
            let direction: u8 = MD::deserialize(reader)?;

            let result = SpawnEntityPaintingResponse {
                entity_id,
                entity_uuid,
                title,
                location,
                direction,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x3)?;
            write_varint(&mut writer, self.entity_id as u32)?;
            self.entity_uuid.serialize(&mut writer)?;
            write_varint(&mut writer, self.title as u32)?;
            self.location.serialize(&mut writer)?;
            self.direction.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct NamedEntitySpawnResponse {
        pub entity_id: i32,
        pub player_uuid: u128,
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub yaw: i8,
        pub pitch: i8,
    }
    impl<'p> MD<'p> for NamedEntitySpawnResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<NamedEntitySpawnResponse> {
            let entity_id: i32 = read_varint(&mut reader)?;
            let player_uuid: u128 = MD::deserialize(reader)?;
            let x: f64 = MD::deserialize(reader)?;
            let y: f64 = MD::deserialize(reader)?;
            let z: f64 = MD::deserialize(reader)?;
            let yaw: i8 = MD::deserialize(reader)?;
            let pitch: i8 = MD::deserialize(reader)?;

            let result = NamedEntitySpawnResponse {
                entity_id,
                player_uuid,
                x,
                y,
                z,
                yaw,
                pitch,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x4)?;
            write_varint(&mut writer, self.entity_id as u32)?;
            self.player_uuid.serialize(&mut writer)?;
            self.x.serialize(&mut writer)?;
            self.y.serialize(&mut writer)?;
            self.z.serialize(&mut writer)?;
            self.yaw.serialize(&mut writer)?;
            self.pitch.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct AnimationResponse {
        pub entity_id: i32,
        pub animation: u8,
    }
    impl<'p> MD<'p> for AnimationResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<AnimationResponse> {
            let entity_id: i32 = read_varint(&mut reader)?;
            let animation: u8 = MD::deserialize(reader)?;

            let result = AnimationResponse {
                entity_id,
                animation,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x6)?;
            write_varint(&mut writer, self.entity_id as u32)?;
            self.animation.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct StatisticsResponse_Entries {
        pub category_id: i32,
        pub statistic_id: i32,
        pub value: i32,
    }
    impl<'p> MD<'p> for StatisticsResponse_Entries {
        fn deserialize(mut reader: &mut &[u8]) -> Result<StatisticsResponse_Entries> {
            let category_id: i32 = read_varint(&mut reader)?;
            let statistic_id: i32 = read_varint(&mut reader)?;
            let value: i32 = read_varint(&mut reader)?;

            let result = StatisticsResponse_Entries {
                category_id,
                statistic_id,
                value,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, self.category_id as u32)?;
            write_varint(&mut writer, self.statistic_id as u32)?;
            write_varint(&mut writer, self.value as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct StatisticsResponse {
        pub entries: Vec<StatisticsResponse_Entries>,
    }
    impl<'p> MD<'p> for StatisticsResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<StatisticsResponse> {
            let array_count: i32 = read_varint(&mut reader)?;
            let mut entries = Vec::with_capacity(cautious_size(array_count as usize));
            for _ in 0..array_count {
                let x: StatisticsResponse_Entries =
                    StatisticsResponse_Entries::deserialize(reader)?;
                entries.push(x);
            }

            let result = StatisticsResponse { entries };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut _writer: &mut W) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct AdvancementsResponse {}
    impl<'p> MD<'p> for AdvancementsResponse {
        fn deserialize(mut _reader: &mut &[u8]) -> Result<AdvancementsResponse> {
            // failed

            let result = AdvancementsResponse {};
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x63)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct BlockBreakAnimationResponse {
        pub entity_id: i32,
        pub location: Position,
        pub destroy_stage: i8,
    }
    impl<'p> MD<'p> for BlockBreakAnimationResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<BlockBreakAnimationResponse> {
            let entity_id: i32 = read_varint(&mut reader)?;
            let location: Position = MD::deserialize(reader)?;
            let destroy_stage: i8 = MD::deserialize(reader)?;

            let result = BlockBreakAnimationResponse {
                entity_id,
                location,
                destroy_stage,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x9)?;
            write_varint(&mut writer, self.entity_id as u32)?;
            self.location.serialize(&mut writer)?;
            self.destroy_stage.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct TileEntityDataResponse<'p> {
        pub location: Position,
        pub action: i32,
        pub nbt_data: IndexedOptionNbt<'p>,
    }
    impl<'p> MD<'p> for TileEntityDataResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<TileEntityDataResponse<'p>> {
            let location: Position = MD::deserialize(reader)?;
            let action: i32 = read_varint(&mut reader)?;
            let nbt_data: IndexedOptionNbt<'p> = MD::deserialize(reader)?;

            let result = TileEntityDataResponse {
                location,
                action,
                nbt_data,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0xa)?;
            self.location.serialize(&mut writer)?;
            write_varint(&mut writer, self.action as u32)?;
            self.nbt_data.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct BlockActionResponse {
        pub location: Position,
        pub byte_1: u8,
        pub byte_2: u8,
        pub block_id: i32,
    }
    impl<'p> MD<'p> for BlockActionResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<BlockActionResponse> {
            let location: Position = MD::deserialize(reader)?;
            let byte_1: u8 = MD::deserialize(reader)?;
            let byte_2: u8 = MD::deserialize(reader)?;
            let block_id: i32 = read_varint(&mut reader)?;

            let result = BlockActionResponse {
                location,
                byte_1,
                byte_2,
                block_id,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0xb)?;
            self.location.serialize(&mut writer)?;
            self.byte_1.serialize(&mut writer)?;
            self.byte_2.serialize(&mut writer)?;
            write_varint(&mut writer, self.block_id as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct BlockChangeResponse {
        pub location: Position,
        pub type_: i32,
    }
    impl<'p> MD<'p> for BlockChangeResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<BlockChangeResponse> {
            let location: Position = MD::deserialize(reader)?;
            let type_: i32 = read_varint(&mut reader)?;

            let result = BlockChangeResponse { location, type_ };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0xc)?;
            self.location.serialize(&mut writer)?;
            write_varint(&mut writer, self.type_ as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct BossBarResponse {}
    impl<'p> MD<'p> for BossBarResponse {
        fn deserialize(mut _reader: &mut &[u8]) -> Result<BossBarResponse> {
            // failed

            let result = BossBarResponse {};
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0xd)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct DifficultyResponse {
        pub difficulty: u8,
        pub difficulty_locked: bool,
    }
    impl<'p> MD<'p> for DifficultyResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<DifficultyResponse> {
            let difficulty: u8 = MD::deserialize(reader)?;
            let difficulty_locked: bool = MD::deserialize(reader)?;

            let result = DifficultyResponse {
                difficulty,
                difficulty_locked,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0xe)?;
            self.difficulty.serialize(&mut writer)?;
            self.difficulty_locked.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct TabCompleteResponse_Matches<'p> {
        pub match_: &'p str,
        pub tooltip: Option<&'p str>,
    }
    impl<'p> MD<'p> for TabCompleteResponse_Matches<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<TabCompleteResponse_Matches<'p>> {
            let match_: &'p str = MD::deserialize(reader)?;
            let tooltip: Option<&'p str> = MD::deserialize(reader)?;

            let result = TabCompleteResponse_Matches { match_, tooltip };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            self.match_.serialize(&mut writer)?;
            self.tooltip.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct TabCompleteResponse<'p> {
        pub transaction_id: i32,
        pub start: i32,
        pub length: i32,
        pub matches: Vec<TabCompleteResponse_Matches<'p>>,
    }
    impl<'p> MD<'p> for TabCompleteResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<TabCompleteResponse<'p>> {
            let transaction_id: i32 = read_varint(&mut reader)?;
            let start: i32 = read_varint(&mut reader)?;
            let length: i32 = read_varint(&mut reader)?;
            let array_count: i32 = read_varint(&mut reader)?;
            let mut matches = Vec::with_capacity(cautious_size(array_count as usize));
            for _ in 0..array_count {
                let x: TabCompleteResponse_Matches =
                    TabCompleteResponse_Matches::deserialize(reader)?;
                matches.push(x);
            }

            let result = TabCompleteResponse {
                transaction_id,
                start,
                length,
                matches,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut _writer: &mut W) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct DeclareCommandsResponse {}
    impl<'p> MD<'p> for DeclareCommandsResponse {
        fn deserialize(mut _reader: &mut &[u8]) -> Result<DeclareCommandsResponse> {
            // failed

            let result = DeclareCommandsResponse {};
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x12)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct FacePlayerResponse {}
    impl<'p> MD<'p> for FacePlayerResponse {
        fn deserialize(mut _reader: &mut &[u8]) -> Result<FacePlayerResponse> {
            // failed

            let result = FacePlayerResponse {};
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x37)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct NbtQueryResponse<'p> {
        pub transaction_id: i32,
        pub nbt: IndexedOptionNbt<'p>,
    }
    impl<'p> MD<'p> for NbtQueryResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<NbtQueryResponse<'p>> {
            let transaction_id: i32 = read_varint(&mut reader)?;
            let nbt: IndexedOptionNbt<'p> = MD::deserialize(reader)?;

            let result = NbtQueryResponse {
                transaction_id,
                nbt,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x60)?;
            write_varint(&mut writer, self.transaction_id as u32)?;
            self.nbt.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ChatResponse<'p> {
        pub message: &'p str,
        pub position: i8,
        pub sender: u128,
    }
    impl<'p> MD<'p> for ChatResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<ChatResponse<'p>> {
            let message: &'p str = MD::deserialize(reader)?;
            let position: i8 = MD::deserialize(reader)?;
            let sender: u128 = MD::deserialize(reader)?;

            let result = ChatResponse {
                message,
                position,
                sender,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0xf)?;
            self.message.serialize(&mut writer)?;
            self.position.serialize(&mut writer)?;
            self.sender.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct MultiBlockChangeResponse_ChunkCoordinates {
        pub x: u32,
        pub z: u32,
        pub y: u32,
    }
    impl<'p> MD<'p> for MultiBlockChangeResponse_ChunkCoordinates {
        fn deserialize(
            mut reader: &mut &[u8],
        ) -> Result<MultiBlockChangeResponse_ChunkCoordinates> {
            let value: i64 = MD::deserialize(reader)?;
            let x: u32 = (value << 42 >> 42) as _;
            let z: u32 = (value << 20 >> 42) as _;
            let y: u32 = (value << 0 >> 44) as _;
            let result = MultiBlockChangeResponse_ChunkCoordinates { x, z, y };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut _writer: &mut W) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct MultiBlockChangeResponse {
        pub chunk_coordinates: MultiBlockChangeResponse_ChunkCoordinates,
        pub not_trust_edges: bool,
        pub records: Vec<i64>,
    }
    impl<'p> MD<'p> for MultiBlockChangeResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<MultiBlockChangeResponse> {
            let chunk_coordinates: MultiBlockChangeResponse_ChunkCoordinates =
                MultiBlockChangeResponse_ChunkCoordinates::deserialize(reader)?;
            let not_trust_edges: bool = MD::deserialize(reader)?;
            let array_count: i32 = read_varint(&mut reader)?;
            let mut records = Vec::with_capacity(cautious_size(array_count as usize));
            for _ in 0..array_count {
                let x: i64 = read_varlong(&mut reader)?;
                records.push(x);
            }

            let result = MultiBlockChangeResponse {
                chunk_coordinates,
                not_trust_edges,
                records,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut _writer: &mut W) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct CloseWindowResponse {
        pub window_id: u8,
    }
    impl<'p> MD<'p> for CloseWindowResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<CloseWindowResponse> {
            let window_id: u8 = MD::deserialize(reader)?;

            let result = CloseWindowResponse { window_id };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x13)?;
            self.window_id.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct OpenWindowResponse<'p> {
        pub window_id: i32,
        pub inventory_type: i32,
        pub window_title: &'p str,
    }
    impl<'p> MD<'p> for OpenWindowResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<OpenWindowResponse<'p>> {
            let window_id: i32 = read_varint(&mut reader)?;
            let inventory_type: i32 = read_varint(&mut reader)?;
            let window_title: &'p str = MD::deserialize(reader)?;

            let result = OpenWindowResponse {
                window_id,
                inventory_type,
                window_title,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x2e)?;
            write_varint(&mut writer, self.window_id as u32)?;
            write_varint(&mut writer, self.inventory_type as u32)?;
            self.window_title.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct WindowItemsResponse<'p> {
        pub window_id: u8,
        pub state_id: i32,
        pub items: Vec<InventorySlot<'p>>,
        pub carried_item: InventorySlot<'p>,
    }
    impl<'p> MD<'p> for WindowItemsResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<WindowItemsResponse<'p>> {
            let window_id: u8 = MD::deserialize(reader)?;
            let state_id: i32 = read_varint(&mut reader)?;
            let array_count: i32 = read_varint(&mut reader)?;
            let mut items = Vec::with_capacity(cautious_size(array_count as usize));
            for _ in 0..array_count {
                let x: InventorySlot<'p> = MD::deserialize(reader)?;
                items.push(x);
            }
            let carried_item: InventorySlot<'p> = MD::deserialize(reader)?;

            let result = WindowItemsResponse {
                window_id,
                state_id,
                items,
                carried_item,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut _writer: &mut W) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct CraftProgressBarResponse {
        pub window_id: u8,
        pub property: i16,
        pub value: i16,
    }
    impl<'p> MD<'p> for CraftProgressBarResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<CraftProgressBarResponse> {
            let window_id: u8 = MD::deserialize(reader)?;
            let property: i16 = MD::deserialize(reader)?;
            let value: i16 = MD::deserialize(reader)?;

            let result = CraftProgressBarResponse {
                window_id,
                property,
                value,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x15)?;
            self.window_id.serialize(&mut writer)?;
            self.property.serialize(&mut writer)?;
            self.value.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SetSlotResponse<'p> {
        pub window_id: i8,
        pub state_id: i32,
        pub slot: i16,
        pub item: InventorySlot<'p>,
    }
    impl<'p> MD<'p> for SetSlotResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<SetSlotResponse<'p>> {
            let window_id: i8 = MD::deserialize(reader)?;
            let state_id: i32 = read_varint(&mut reader)?;
            let slot: i16 = MD::deserialize(reader)?;
            let item: InventorySlot<'p> = MD::deserialize(reader)?;

            let result = SetSlotResponse {
                window_id,
                state_id,
                slot,
                item,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x16)?;
            self.window_id.serialize(&mut writer)?;
            write_varint(&mut writer, self.state_id as u32)?;
            self.slot.serialize(&mut writer)?;
            self.item.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SetCooldownResponse {
        pub item_id: i32,
        pub cooldown_ticks: i32,
    }
    impl<'p> MD<'p> for SetCooldownResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<SetCooldownResponse> {
            let item_id: i32 = read_varint(&mut reader)?;
            let cooldown_ticks: i32 = read_varint(&mut reader)?;

            let result = SetCooldownResponse {
                item_id,
                cooldown_ticks,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x17)?;
            write_varint(&mut writer, self.item_id as u32)?;
            write_varint(&mut writer, self.cooldown_ticks as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct CustomPayloadResponse<'p> {
        pub channel: &'p str,
        pub data: &'p [u8],
    }
    impl<'p> MD<'p> for CustomPayloadResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<CustomPayloadResponse<'p>> {
            let channel: &'p str = MD::deserialize(reader)?;
            let data: &'p [u8] = &reader[..];
            *reader = &[];

            let result = CustomPayloadResponse { channel, data };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x18)?;
            self.channel.serialize(&mut writer)?;
            writer.write_all(self.data)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct NamedSoundEffectResponse<'p> {
        pub sound_name: &'p str,
        pub sound_category: i32,
        pub x: i32,
        pub y: i32,
        pub z: i32,
        pub volume: f32,
        pub pitch: f32,
    }
    impl<'p> MD<'p> for NamedSoundEffectResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<NamedSoundEffectResponse<'p>> {
            let sound_name: &'p str = MD::deserialize(reader)?;
            let sound_category: i32 = read_varint(&mut reader)?;
            let x: i32 = MD::deserialize(reader)?;
            let y: i32 = MD::deserialize(reader)?;
            let z: i32 = MD::deserialize(reader)?;
            let volume: f32 = MD::deserialize(reader)?;
            let pitch: f32 = MD::deserialize(reader)?;

            let result = NamedSoundEffectResponse {
                sound_name,
                sound_category,
                x,
                y,
                z,
                volume,
                pitch,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x19)?;
            self.sound_name.serialize(&mut writer)?;
            write_varint(&mut writer, self.sound_category as u32)?;
            self.x.serialize(&mut writer)?;
            self.y.serialize(&mut writer)?;
            self.z.serialize(&mut writer)?;
            self.volume.serialize(&mut writer)?;
            self.pitch.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct KickDisconnectResponse<'p> {
        pub reason: &'p str,
    }
    impl<'p> MD<'p> for KickDisconnectResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<KickDisconnectResponse<'p>> {
            let reason: &'p str = MD::deserialize(reader)?;

            let result = KickDisconnectResponse { reason };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x1a)?;
            self.reason.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EntityStatusResponse {
        pub entity_id: i32,
        pub entity_status: i8,
    }
    impl<'p> MD<'p> for EntityStatusResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<EntityStatusResponse> {
            let entity_id: i32 = MD::deserialize(reader)?;
            let entity_status: i8 = MD::deserialize(reader)?;

            let result = EntityStatusResponse {
                entity_id,
                entity_status,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x1b)?;
            self.entity_id.serialize(&mut writer)?;
            self.entity_status.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ExplosionResponse_AffectedBlockOffsets {
        pub x: i8,
        pub y: i8,
        pub z: i8,
    }
    impl<'p> MD<'p> for ExplosionResponse_AffectedBlockOffsets {
        fn deserialize(mut reader: &mut &[u8]) -> Result<ExplosionResponse_AffectedBlockOffsets> {
            let x: i8 = MD::deserialize(reader)?;
            let y: i8 = MD::deserialize(reader)?;
            let z: i8 = MD::deserialize(reader)?;

            let result = ExplosionResponse_AffectedBlockOffsets { x, y, z };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            self.x.serialize(&mut writer)?;
            self.y.serialize(&mut writer)?;
            self.z.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ExplosionResponse {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub radius: f32,
        pub affected_block_offsets: Vec<ExplosionResponse_AffectedBlockOffsets>,
        pub player_motion_x: f32,
        pub player_motion_y: f32,
        pub player_motion_z: f32,
    }
    impl<'p> MD<'p> for ExplosionResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<ExplosionResponse> {
            let x: f32 = MD::deserialize(reader)?;
            let y: f32 = MD::deserialize(reader)?;
            let z: f32 = MD::deserialize(reader)?;
            let radius: f32 = MD::deserialize(reader)?;
            let array_count: i32 = read_varint(&mut reader)?;
            let mut affected_block_offsets =
                Vec::with_capacity(cautious_size(array_count as usize));
            for _ in 0..array_count {
                let x: ExplosionResponse_AffectedBlockOffsets =
                    ExplosionResponse_AffectedBlockOffsets::deserialize(reader)?;
                affected_block_offsets.push(x);
            }
            let player_motion_x: f32 = MD::deserialize(reader)?;
            let player_motion_y: f32 = MD::deserialize(reader)?;
            let player_motion_z: f32 = MD::deserialize(reader)?;

            let result = ExplosionResponse {
                x,
                y,
                z,
                radius,
                affected_block_offsets,
                player_motion_x,
                player_motion_y,
                player_motion_z,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut _writer: &mut W) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct UnloadChunkResponse {
        pub chunk_x: i32,
        pub chunk_z: i32,
    }
    impl<'p> MD<'p> for UnloadChunkResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<UnloadChunkResponse> {
            let chunk_x: i32 = MD::deserialize(reader)?;
            let chunk_z: i32 = MD::deserialize(reader)?;

            let result = UnloadChunkResponse { chunk_x, chunk_z };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x1d)?;
            self.chunk_x.serialize(&mut writer)?;
            self.chunk_z.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct GameStateChangeResponse {
        pub reason: u8,
        pub game_mode: f32,
    }
    impl<'p> MD<'p> for GameStateChangeResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<GameStateChangeResponse> {
            let reason: u8 = MD::deserialize(reader)?;
            let game_mode: f32 = MD::deserialize(reader)?;

            let result = GameStateChangeResponse { reason, game_mode };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x1e)?;
            self.reason.serialize(&mut writer)?;
            self.game_mode.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct OpenHorseWindowResponse {
        pub window_id: u8,
        pub nb_slots: i32,
        pub entity_id: i32,
    }
    impl<'p> MD<'p> for OpenHorseWindowResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<OpenHorseWindowResponse> {
            let window_id: u8 = MD::deserialize(reader)?;
            let nb_slots: i32 = read_varint(&mut reader)?;
            let entity_id: i32 = MD::deserialize(reader)?;

            let result = OpenHorseWindowResponse {
                window_id,
                nb_slots,
                entity_id,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x1f)?;
            self.window_id.serialize(&mut writer)?;
            write_varint(&mut writer, self.nb_slots as u32)?;
            self.entity_id.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct KeepAliveResponse {
        pub keep_alive_id: i64,
    }
    impl<'p> MD<'p> for KeepAliveResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<KeepAliveResponse> {
            let keep_alive_id: i64 = MD::deserialize(reader)?;

            let result = KeepAliveResponse { keep_alive_id };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x21)?;
            self.keep_alive_id.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct MapChunkResponse<'p> {
        pub x: i32,
        pub z: i32,
        pub heightmaps: IndexedNbt<'p>,
        pub chunk_data: &'p [u8],
        pub block_entities: Vec<ChunkBlockEntity<'p>>,
        pub trust_edges: bool,
        pub sky_light_mask: UnalignedSliceI64<'p>,
        pub block_light_mask: UnalignedSliceI64<'p>,
        pub empty_sky_light_mask: UnalignedSliceI64<'p>,
        pub empty_block_light_mask: UnalignedSliceI64<'p>,
        pub sky_light: Vec<&'p [u8]>,
        pub block_light: Vec<&'p [u8]>,
    }
    impl<'p> MD<'p> for MapChunkResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<MapChunkResponse<'p>> {
            let x: i32 = MD::deserialize(reader)?;
            let z: i32 = MD::deserialize(reader)?;
            let heightmaps: IndexedNbt<'p> = MD::deserialize(reader)?;
            let chunk_data: &'p [u8] = MD::deserialize(reader)?;
            let array_count: i32 = read_varint(&mut reader)?;
            let mut block_entities = Vec::with_capacity(cautious_size(array_count as usize));
            for _ in 0..array_count {
                let x: ChunkBlockEntity = MD::deserialize(reader)?;
                block_entities.push(x);
            }
            let trust_edges: bool = MD::deserialize(reader)?;
            let array_count: i32 = read_varint(&mut reader)?;
            let mem = reader.read_mem(array_count as usize * size_of::<i64>())?;
            let sky_light_mask = UnalignedSliceI64::new(mem);
            let array_count: i32 = read_varint(&mut reader)?;
            let mem = reader.read_mem(array_count as usize * size_of::<i64>())?;
            let block_light_mask = UnalignedSliceI64::new(mem);
            let array_count: i32 = read_varint(&mut reader)?;
            let mem = reader.read_mem(array_count as usize * size_of::<i64>())?;
            let empty_sky_light_mask = UnalignedSliceI64::new(mem);
            let array_count: i32 = read_varint(&mut reader)?;
            let mem = reader.read_mem(array_count as usize * size_of::<i64>())?;
            let empty_block_light_mask = UnalignedSliceI64::new(mem);
            let array_count: i32 = read_varint(&mut reader)?;
            let mut sky_light = Vec::with_capacity(cautious_size(array_count as usize));
            for _ in 0..array_count {
                let x: &'p [u8] = MD::deserialize(reader)?;
                sky_light.push(x);
            }
            let array_count: i32 = read_varint(&mut reader)?;
            let mut block_light = Vec::with_capacity(cautious_size(array_count as usize));
            for _ in 0..array_count {
                let x: &'p [u8] = MD::deserialize(reader)?;
                block_light.push(x);
            }

            let result = MapChunkResponse {
                x,
                z,
                heightmaps,
                chunk_data,
                block_entities,
                trust_edges,
                sky_light_mask,
                block_light_mask,
                empty_sky_light_mask,
                empty_block_light_mask,
                sky_light,
                block_light,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut _writer: &mut W) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct WorldEventResponse {
        pub effect_id: i32,
        pub location: Position,
        pub data: i32,
        pub global: bool,
    }
    impl<'p> MD<'p> for WorldEventResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<WorldEventResponse> {
            let effect_id: i32 = MD::deserialize(reader)?;
            let location: Position = MD::deserialize(reader)?;
            let data: i32 = MD::deserialize(reader)?;
            let global: bool = MD::deserialize(reader)?;

            let result = WorldEventResponse {
                effect_id,
                location,
                data,
                global,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x23)?;
            self.effect_id.serialize(&mut writer)?;
            self.location.serialize(&mut writer)?;
            self.data.serialize(&mut writer)?;
            self.global.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct WorldParticlesResponse {}
    impl<'p> MD<'p> for WorldParticlesResponse {
        fn deserialize(mut _reader: &mut &[u8]) -> Result<WorldParticlesResponse> {
            // failed

            let result = WorldParticlesResponse {};
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x24)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UpdateLightResponse<'p> {
        pub chunk_x: i32,
        pub chunk_z: i32,
        pub trust_edges: bool,
        pub sky_light_mask: UnalignedSliceI64<'p>,
        pub block_light_mask: UnalignedSliceI64<'p>,
        pub empty_sky_light_mask: UnalignedSliceI64<'p>,
        pub empty_block_light_mask: UnalignedSliceI64<'p>,
        pub sky_light: Vec<&'p [u8]>,
        pub block_light: Vec<&'p [u8]>,
    }
    impl<'p> MD<'p> for UpdateLightResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<UpdateLightResponse<'p>> {
            let chunk_x: i32 = read_varint(&mut reader)?;
            let chunk_z: i32 = read_varint(&mut reader)?;
            let trust_edges: bool = MD::deserialize(reader)?;
            let array_count: i32 = read_varint(&mut reader)?;
            let mem = reader.read_mem(array_count as usize * size_of::<i64>())?;
            let sky_light_mask = UnalignedSliceI64::new(mem);
            let array_count: i32 = read_varint(&mut reader)?;
            let mem = reader.read_mem(array_count as usize * size_of::<i64>())?;
            let block_light_mask = UnalignedSliceI64::new(mem);
            let array_count: i32 = read_varint(&mut reader)?;
            let mem = reader.read_mem(array_count as usize * size_of::<i64>())?;
            let empty_sky_light_mask = UnalignedSliceI64::new(mem);
            let array_count: i32 = read_varint(&mut reader)?;
            let mem = reader.read_mem(array_count as usize * size_of::<i64>())?;
            let empty_block_light_mask = UnalignedSliceI64::new(mem);
            let array_count: i32 = read_varint(&mut reader)?;
            let mut sky_light = Vec::with_capacity(cautious_size(array_count as usize));
            for _ in 0..array_count {
                let x: &'p [u8] = MD::deserialize(reader)?;
                sky_light.push(x);
            }
            let array_count: i32 = read_varint(&mut reader)?;
            let mut block_light = Vec::with_capacity(cautious_size(array_count as usize));
            for _ in 0..array_count {
                let x: &'p [u8] = MD::deserialize(reader)?;
                block_light.push(x);
            }

            let result = UpdateLightResponse {
                chunk_x,
                chunk_z,
                trust_edges,
                sky_light_mask,
                block_light_mask,
                empty_sky_light_mask,
                empty_block_light_mask,
                sky_light,
                block_light,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut _writer: &mut W) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct LoginResponse<'p> {
        pub entity_id: i32,
        pub is_hardcore: bool,
        pub game_mode: u8,
        pub previous_game_mode: i8,
        pub world_names: Vec<&'p str>,
        pub dimension_codec: IndexedNbt<'p>,
        pub dimension: IndexedNbt<'p>,
        pub world_name: &'p str,
        pub hashed_seed: i64,
        pub max_players: i32,
        pub view_distance: i32,
        pub simulation_distance: i32,
        pub reduced_debug_info: bool,
        pub enable_respawn_screen: bool,
        pub is_debug: bool,
        pub is_flat: bool,
    }
    impl<'p> MD<'p> for LoginResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<LoginResponse<'p>> {
            let entity_id: i32 = MD::deserialize(reader)?;
            let is_hardcore: bool = MD::deserialize(reader)?;
            let game_mode: u8 = MD::deserialize(reader)?;
            let previous_game_mode: i8 = MD::deserialize(reader)?;
            let array_count: i32 = read_varint(&mut reader)?;
            let mut world_names = Vec::with_capacity(cautious_size(array_count as usize));
            for _ in 0..array_count {
                let x: &'p str = MD::deserialize(reader)?;
                world_names.push(x);
            }
            let dimension_codec: IndexedNbt<'p> = MD::deserialize(reader)?;
            let dimension: IndexedNbt<'p> = MD::deserialize(reader)?;
            let world_name: &'p str = MD::deserialize(reader)?;
            let hashed_seed: i64 = MD::deserialize(reader)?;
            let max_players: i32 = read_varint(&mut reader)?;
            let view_distance: i32 = read_varint(&mut reader)?;
            let simulation_distance: i32 = read_varint(&mut reader)?;
            let reduced_debug_info: bool = MD::deserialize(reader)?;
            let enable_respawn_screen: bool = MD::deserialize(reader)?;
            let is_debug: bool = MD::deserialize(reader)?;
            let is_flat: bool = MD::deserialize(reader)?;

            let result = LoginResponse {
                entity_id,
                is_hardcore,
                game_mode,
                previous_game_mode,
                world_names,
                dimension_codec,
                dimension,
                world_name,
                hashed_seed,
                max_players,
                view_distance,
                simulation_distance,
                reduced_debug_info,
                enable_respawn_screen,
                is_debug,
                is_flat,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut _writer: &mut W) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct MapResponse {}
    impl<'p> MD<'p> for MapResponse {
        fn deserialize(mut _reader: &mut &[u8]) -> Result<MapResponse> {
            // failed

            let result = MapResponse {};
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x27)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct TradeListResponse_Trades<'p> {
        pub input_item_1: InventorySlot<'p>,
        pub output_item: InventorySlot<'p>,
        pub input_item_2: Option<InventorySlot<'p>>,
        pub trade_disabled: bool,
        pub nb_trade_uses: i32,
        pub maximum_nb_trade_uses: i32,
        pub xp: i32,
        pub special_price: i32,
        pub price_multiplier: f32,
        pub demand: i32,
    }
    impl<'p> MD<'p> for TradeListResponse_Trades<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<TradeListResponse_Trades<'p>> {
            let input_item_1: InventorySlot<'p> = MD::deserialize(reader)?;
            let output_item: InventorySlot<'p> = MD::deserialize(reader)?;
            let input_item_2: Option<InventorySlot<'p>> = MD::deserialize(reader)?;
            let trade_disabled: bool = MD::deserialize(reader)?;
            let nb_trade_uses: i32 = MD::deserialize(reader)?;
            let maximum_nb_trade_uses: i32 = MD::deserialize(reader)?;
            let xp: i32 = MD::deserialize(reader)?;
            let special_price: i32 = MD::deserialize(reader)?;
            let price_multiplier: f32 = MD::deserialize(reader)?;
            let demand: i32 = MD::deserialize(reader)?;

            let result = TradeListResponse_Trades {
                input_item_1,
                output_item,
                input_item_2,
                trade_disabled,
                nb_trade_uses,
                maximum_nb_trade_uses,
                xp,
                special_price,
                price_multiplier,
                demand,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            self.input_item_1.serialize(&mut writer)?;
            self.output_item.serialize(&mut writer)?;
            self.input_item_2.serialize(&mut writer)?;
            self.trade_disabled.serialize(&mut writer)?;
            self.nb_trade_uses.serialize(&mut writer)?;
            self.maximum_nb_trade_uses.serialize(&mut writer)?;
            self.xp.serialize(&mut writer)?;
            self.special_price.serialize(&mut writer)?;
            self.price_multiplier.serialize(&mut writer)?;
            self.demand.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct TradeListResponse<'p> {
        pub window_id: i32,
        pub trades: Vec<TradeListResponse_Trades<'p>>,
        pub villager_level: i32,
        pub experience: i32,
        pub is_regular_villager: bool,
        pub can_restock: bool,
    }
    impl<'p> MD<'p> for TradeListResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<TradeListResponse<'p>> {
            let window_id: i32 = read_varint(&mut reader)?;
            let array_count: u8 = MD::deserialize(reader)?;
            let mut trades = Vec::with_capacity(cautious_size(array_count as usize));
            for _ in 0..array_count {
                let x: TradeListResponse_Trades = TradeListResponse_Trades::deserialize(reader)?;
                trades.push(x);
            }
            let villager_level: i32 = read_varint(&mut reader)?;
            let experience: i32 = read_varint(&mut reader)?;
            let is_regular_villager: bool = MD::deserialize(reader)?;
            let can_restock: bool = MD::deserialize(reader)?;

            let result = TradeListResponse {
                window_id,
                trades,
                villager_level,
                experience,
                is_regular_villager,
                can_restock,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut _writer: &mut W) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct RelEntityMoveResponse {
        pub entity_id: i32,
        pub d_x: i16,
        pub d_y: i16,
        pub d_z: i16,
        pub on_ground: bool,
    }
    impl<'p> MD<'p> for RelEntityMoveResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<RelEntityMoveResponse> {
            let entity_id: i32 = read_varint(&mut reader)?;
            let d_x: i16 = MD::deserialize(reader)?;
            let d_y: i16 = MD::deserialize(reader)?;
            let d_z: i16 = MD::deserialize(reader)?;
            let on_ground: bool = MD::deserialize(reader)?;

            let result = RelEntityMoveResponse {
                entity_id,
                d_x,
                d_y,
                d_z,
                on_ground,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x29)?;
            write_varint(&mut writer, self.entity_id as u32)?;
            self.d_x.serialize(&mut writer)?;
            self.d_y.serialize(&mut writer)?;
            self.d_z.serialize(&mut writer)?;
            self.on_ground.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EntityMoveLookResponse {
        pub entity_id: i32,
        pub d_x: i16,
        pub d_y: i16,
        pub d_z: i16,
        pub yaw: i8,
        pub pitch: i8,
        pub on_ground: bool,
    }
    impl<'p> MD<'p> for EntityMoveLookResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<EntityMoveLookResponse> {
            let entity_id: i32 = read_varint(&mut reader)?;
            let d_x: i16 = MD::deserialize(reader)?;
            let d_y: i16 = MD::deserialize(reader)?;
            let d_z: i16 = MD::deserialize(reader)?;
            let yaw: i8 = MD::deserialize(reader)?;
            let pitch: i8 = MD::deserialize(reader)?;
            let on_ground: bool = MD::deserialize(reader)?;

            let result = EntityMoveLookResponse {
                entity_id,
                d_x,
                d_y,
                d_z,
                yaw,
                pitch,
                on_ground,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x2a)?;
            write_varint(&mut writer, self.entity_id as u32)?;
            self.d_x.serialize(&mut writer)?;
            self.d_y.serialize(&mut writer)?;
            self.d_z.serialize(&mut writer)?;
            self.yaw.serialize(&mut writer)?;
            self.pitch.serialize(&mut writer)?;
            self.on_ground.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EntityLookResponse {
        pub entity_id: i32,
        pub yaw: i8,
        pub pitch: i8,
        pub on_ground: bool,
    }
    impl<'p> MD<'p> for EntityLookResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<EntityLookResponse> {
            let entity_id: i32 = read_varint(&mut reader)?;
            let yaw: i8 = MD::deserialize(reader)?;
            let pitch: i8 = MD::deserialize(reader)?;
            let on_ground: bool = MD::deserialize(reader)?;

            let result = EntityLookResponse {
                entity_id,
                yaw,
                pitch,
                on_ground,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x2b)?;
            write_varint(&mut writer, self.entity_id as u32)?;
            self.yaw.serialize(&mut writer)?;
            self.pitch.serialize(&mut writer)?;
            self.on_ground.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct VehicleMoveResponse {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub yaw: f32,
        pub pitch: f32,
    }
    impl<'p> MD<'p> for VehicleMoveResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<VehicleMoveResponse> {
            let x: f64 = MD::deserialize(reader)?;
            let y: f64 = MD::deserialize(reader)?;
            let z: f64 = MD::deserialize(reader)?;
            let yaw: f32 = MD::deserialize(reader)?;
            let pitch: f32 = MD::deserialize(reader)?;

            let result = VehicleMoveResponse {
                x,
                y,
                z,
                yaw,
                pitch,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x2c)?;
            self.x.serialize(&mut writer)?;
            self.y.serialize(&mut writer)?;
            self.z.serialize(&mut writer)?;
            self.yaw.serialize(&mut writer)?;
            self.pitch.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct OpenBookResponse {
        pub hand: i32,
    }
    impl<'p> MD<'p> for OpenBookResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<OpenBookResponse> {
            let hand: i32 = read_varint(&mut reader)?;

            let result = OpenBookResponse { hand };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x2d)?;
            write_varint(&mut writer, self.hand as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct OpenSignEntityResponse {
        pub location: Position,
    }
    impl<'p> MD<'p> for OpenSignEntityResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<OpenSignEntityResponse> {
            let location: Position = MD::deserialize(reader)?;

            let result = OpenSignEntityResponse { location };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x2f)?;
            self.location.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct CraftRecipeResponse<'p> {
        pub window_id: i8,
        pub recipe: &'p str,
    }
    impl<'p> MD<'p> for CraftRecipeResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<CraftRecipeResponse<'p>> {
            let window_id: i8 = MD::deserialize(reader)?;
            let recipe: &'p str = MD::deserialize(reader)?;

            let result = CraftRecipeResponse { window_id, recipe };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x31)?;
            self.window_id.serialize(&mut writer)?;
            self.recipe.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct AbilitiesResponse {
        pub flags: i8,
        pub flying_speed: f32,
        pub walking_speed: f32,
    }
    impl<'p> MD<'p> for AbilitiesResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<AbilitiesResponse> {
            let flags: i8 = MD::deserialize(reader)?;
            let flying_speed: f32 = MD::deserialize(reader)?;
            let walking_speed: f32 = MD::deserialize(reader)?;

            let result = AbilitiesResponse {
                flags,
                flying_speed,
                walking_speed,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x32)?;
            self.flags.serialize(&mut writer)?;
            self.flying_speed.serialize(&mut writer)?;
            self.walking_speed.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EndCombatEventResponse {
        pub duration: i32,
        pub entity_id: i32,
    }
    impl<'p> MD<'p> for EndCombatEventResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<EndCombatEventResponse> {
            let duration: i32 = read_varint(&mut reader)?;
            let entity_id: i32 = MD::deserialize(reader)?;

            let result = EndCombatEventResponse {
                duration,
                entity_id,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x33)?;
            write_varint(&mut writer, self.duration as u32)?;
            self.entity_id.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EnterCombatEventResponse {}
    impl<'p> MD<'p> for EnterCombatEventResponse {
        fn deserialize(mut _reader: &mut &[u8]) -> Result<EnterCombatEventResponse> {
            let result = EnterCombatEventResponse {};
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x34)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct DeathCombatEventResponse<'p> {
        pub player_id: i32,
        pub entity_id: i32,
        pub message: &'p str,
    }
    impl<'p> MD<'p> for DeathCombatEventResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<DeathCombatEventResponse<'p>> {
            let player_id: i32 = read_varint(&mut reader)?;
            let entity_id: i32 = MD::deserialize(reader)?;
            let message: &'p str = MD::deserialize(reader)?;

            let result = DeathCombatEventResponse {
                player_id,
                entity_id,
                message,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x35)?;
            write_varint(&mut writer, self.player_id as u32)?;
            self.entity_id.serialize(&mut writer)?;
            self.message.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct PlayerInfoResponse_Data {}
    impl<'p> MD<'p> for PlayerInfoResponse_Data {
        fn deserialize(mut _reader: &mut &[u8]) -> Result<PlayerInfoResponse_Data> {
            // failed

            let result = PlayerInfoResponse_Data {};
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct PlayerInfoResponse {
        pub action: i32,
        pub data: Vec<PlayerInfoResponse_Data>,
    }
    impl<'p> MD<'p> for PlayerInfoResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<PlayerInfoResponse> {
            let action: i32 = read_varint(&mut reader)?;
            let array_count: i32 = read_varint(&mut reader)?;
            let mut data = Vec::with_capacity(cautious_size(array_count as usize));
            for _ in 0..array_count {
                let x: PlayerInfoResponse_Data = PlayerInfoResponse_Data::deserialize(reader)?;
                data.push(x);
            }

            let result = PlayerInfoResponse { action, data };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut _writer: &mut W) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct PositionResponse {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub yaw: f32,
        pub pitch: f32,
        pub flags: i8,
        pub teleport_id: i32,
        pub dismount_vehicle: bool,
    }
    impl<'p> MD<'p> for PositionResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<PositionResponse> {
            let x: f64 = MD::deserialize(reader)?;
            let y: f64 = MD::deserialize(reader)?;
            let z: f64 = MD::deserialize(reader)?;
            let yaw: f32 = MD::deserialize(reader)?;
            let pitch: f32 = MD::deserialize(reader)?;
            let flags: i8 = MD::deserialize(reader)?;
            let teleport_id: i32 = read_varint(&mut reader)?;
            let dismount_vehicle: bool = MD::deserialize(reader)?;

            let result = PositionResponse {
                x,
                y,
                z,
                yaw,
                pitch,
                flags,
                teleport_id,
                dismount_vehicle,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x38)?;
            self.x.serialize(&mut writer)?;
            self.y.serialize(&mut writer)?;
            self.z.serialize(&mut writer)?;
            self.yaw.serialize(&mut writer)?;
            self.pitch.serialize(&mut writer)?;
            self.flags.serialize(&mut writer)?;
            write_varint(&mut writer, self.teleport_id as u32)?;
            self.dismount_vehicle.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UnlockRecipesResponse {}
    impl<'p> MD<'p> for UnlockRecipesResponse {
        fn deserialize(mut _reader: &mut &[u8]) -> Result<UnlockRecipesResponse> {
            // failed

            let result = UnlockRecipesResponse {};
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x39)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EntityDestroyResponse {
        pub entity_ids: Vec<i32>,
    }
    impl<'p> MD<'p> for EntityDestroyResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<EntityDestroyResponse> {
            let array_count: i32 = read_varint(&mut reader)?;
            let mut entity_ids = Vec::with_capacity(cautious_size(array_count as usize));
            for _ in 0..array_count {
                let x: i32 = read_varint(&mut reader)?;
                entity_ids.push(x);
            }

            let result = EntityDestroyResponse { entity_ids };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut _writer: &mut W) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct RemoveEntityEffectResponse {
        pub entity_id: i32,
        pub effect_id: i32,
    }
    impl<'p> MD<'p> for RemoveEntityEffectResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<RemoveEntityEffectResponse> {
            let entity_id: i32 = read_varint(&mut reader)?;
            let effect_id: i32 = read_varint(&mut reader)?;

            let result = RemoveEntityEffectResponse {
                entity_id,
                effect_id,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x3b)?;
            write_varint(&mut writer, self.entity_id as u32)?;
            write_varint(&mut writer, self.effect_id as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ResourcePackSendResponse<'p> {
        pub url: &'p str,
        pub hash: &'p str,
        pub forced: bool,
        pub prompt_message: Option<&'p str>,
    }
    impl<'p> MD<'p> for ResourcePackSendResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<ResourcePackSendResponse<'p>> {
            let url: &'p str = MD::deserialize(reader)?;
            let hash: &'p str = MD::deserialize(reader)?;
            let forced: bool = MD::deserialize(reader)?;
            let prompt_message: Option<&'p str> = MD::deserialize(reader)?;

            let result = ResourcePackSendResponse {
                url,
                hash,
                forced,
                prompt_message,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x3c)?;
            self.url.serialize(&mut writer)?;
            self.hash.serialize(&mut writer)?;
            self.forced.serialize(&mut writer)?;
            self.prompt_message.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct RespawnResponse<'p> {
        pub dimension: IndexedNbt<'p>,
        pub world_name: &'p str,
        pub hashed_seed: i64,
        pub gamemode: u8,
        pub previous_gamemode: u8,
        pub is_debug: bool,
        pub is_flat: bool,
        pub copy_metadata: bool,
    }
    impl<'p> MD<'p> for RespawnResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<RespawnResponse<'p>> {
            let dimension: IndexedNbt<'p> = MD::deserialize(reader)?;
            let world_name: &'p str = MD::deserialize(reader)?;
            let hashed_seed: i64 = MD::deserialize(reader)?;
            let gamemode: u8 = MD::deserialize(reader)?;
            let previous_gamemode: u8 = MD::deserialize(reader)?;
            let is_debug: bool = MD::deserialize(reader)?;
            let is_flat: bool = MD::deserialize(reader)?;
            let copy_metadata: bool = MD::deserialize(reader)?;

            let result = RespawnResponse {
                dimension,
                world_name,
                hashed_seed,
                gamemode,
                previous_gamemode,
                is_debug,
                is_flat,
                copy_metadata,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x3d)?;
            self.dimension.serialize(&mut writer)?;
            self.world_name.serialize(&mut writer)?;
            self.hashed_seed.serialize(&mut writer)?;
            self.gamemode.serialize(&mut writer)?;
            self.previous_gamemode.serialize(&mut writer)?;
            self.is_debug.serialize(&mut writer)?;
            self.is_flat.serialize(&mut writer)?;
            self.copy_metadata.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EntityHeadRotationResponse {
        pub entity_id: i32,
        pub head_yaw: i8,
    }
    impl<'p> MD<'p> for EntityHeadRotationResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<EntityHeadRotationResponse> {
            let entity_id: i32 = read_varint(&mut reader)?;
            let head_yaw: i8 = MD::deserialize(reader)?;

            let result = EntityHeadRotationResponse {
                entity_id,
                head_yaw,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x3e)?;
            write_varint(&mut writer, self.entity_id as u32)?;
            self.head_yaw.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct CameraResponse {
        pub camera_id: i32,
    }
    impl<'p> MD<'p> for CameraResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<CameraResponse> {
            let camera_id: i32 = read_varint(&mut reader)?;

            let result = CameraResponse { camera_id };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x47)?;
            write_varint(&mut writer, self.camera_id as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct HeldItemSlotResponse {
        pub slot: i8,
    }
    impl<'p> MD<'p> for HeldItemSlotResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<HeldItemSlotResponse> {
            let slot: i8 = MD::deserialize(reader)?;

            let result = HeldItemSlotResponse { slot };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x48)?;
            self.slot.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UpdateViewPositionResponse {
        pub chunk_x: i32,
        pub chunk_z: i32,
    }
    impl<'p> MD<'p> for UpdateViewPositionResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<UpdateViewPositionResponse> {
            let chunk_x: i32 = read_varint(&mut reader)?;
            let chunk_z: i32 = read_varint(&mut reader)?;

            let result = UpdateViewPositionResponse { chunk_x, chunk_z };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x49)?;
            write_varint(&mut writer, self.chunk_x as u32)?;
            write_varint(&mut writer, self.chunk_z as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UpdateViewDistanceResponse {
        pub view_distance: i32,
    }
    impl<'p> MD<'p> for UpdateViewDistanceResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<UpdateViewDistanceResponse> {
            let view_distance: i32 = read_varint(&mut reader)?;

            let result = UpdateViewDistanceResponse { view_distance };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x4a)?;
            write_varint(&mut writer, self.view_distance as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ScoreboardDisplayObjectiveResponse<'p> {
        pub position: i8,
        pub name: &'p str,
    }
    impl<'p> MD<'p> for ScoreboardDisplayObjectiveResponse<'p> {
        fn deserialize(
            mut reader: &mut &'p [u8],
        ) -> Result<ScoreboardDisplayObjectiveResponse<'p>> {
            let position: i8 = MD::deserialize(reader)?;
            let name: &'p str = MD::deserialize(reader)?;

            let result = ScoreboardDisplayObjectiveResponse { position, name };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x4c)?;
            self.position.serialize(&mut writer)?;
            self.name.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EntityMetadataResponse {}
    impl<'p> MD<'p> for EntityMetadataResponse {
        fn deserialize(mut _reader: &mut &[u8]) -> Result<EntityMetadataResponse> {
            // failed

            let result = EntityMetadataResponse {};
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x4d)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct AttachEntityResponse {
        pub entity_id: i32,
        pub vehicle_id: i32,
    }
    impl<'p> MD<'p> for AttachEntityResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<AttachEntityResponse> {
            let entity_id: i32 = MD::deserialize(reader)?;
            let vehicle_id: i32 = MD::deserialize(reader)?;

            let result = AttachEntityResponse {
                entity_id,
                vehicle_id,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x4e)?;
            self.entity_id.serialize(&mut writer)?;
            self.vehicle_id.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EntityVelocityResponse {
        pub entity_id: i32,
        pub velocity_x: i16,
        pub velocity_y: i16,
        pub velocity_z: i16,
    }
    impl<'p> MD<'p> for EntityVelocityResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<EntityVelocityResponse> {
            let entity_id: i32 = read_varint(&mut reader)?;
            let velocity_x: i16 = MD::deserialize(reader)?;
            let velocity_y: i16 = MD::deserialize(reader)?;
            let velocity_z: i16 = MD::deserialize(reader)?;

            let result = EntityVelocityResponse {
                entity_id,
                velocity_x,
                velocity_y,
                velocity_z,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x4f)?;
            write_varint(&mut writer, self.entity_id as u32)?;
            self.velocity_x.serialize(&mut writer)?;
            self.velocity_y.serialize(&mut writer)?;
            self.velocity_z.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EntityEquipmentResponse {}
    impl<'p> MD<'p> for EntityEquipmentResponse {
        fn deserialize(mut _reader: &mut &[u8]) -> Result<EntityEquipmentResponse> {
            // failed

            let result = EntityEquipmentResponse {};
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x50)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ExperienceResponse {
        pub experience_bar: f32,
        pub level: i32,
        pub total_experience: i32,
    }
    impl<'p> MD<'p> for ExperienceResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<ExperienceResponse> {
            let experience_bar: f32 = MD::deserialize(reader)?;
            let level: i32 = read_varint(&mut reader)?;
            let total_experience: i32 = read_varint(&mut reader)?;

            let result = ExperienceResponse {
                experience_bar,
                level,
                total_experience,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x51)?;
            self.experience_bar.serialize(&mut writer)?;
            write_varint(&mut writer, self.level as u32)?;
            write_varint(&mut writer, self.total_experience as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UpdateHealthResponse {
        pub health: f32,
        pub food: i32,
        pub food_saturation: f32,
    }
    impl<'p> MD<'p> for UpdateHealthResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<UpdateHealthResponse> {
            let health: f32 = MD::deserialize(reader)?;
            let food: i32 = read_varint(&mut reader)?;
            let food_saturation: f32 = MD::deserialize(reader)?;

            let result = UpdateHealthResponse {
                health,
                food,
                food_saturation,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x52)?;
            self.health.serialize(&mut writer)?;
            write_varint(&mut writer, self.food as u32)?;
            self.food_saturation.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ScoreboardObjectiveResponse {}
    impl<'p> MD<'p> for ScoreboardObjectiveResponse {
        fn deserialize(mut _reader: &mut &[u8]) -> Result<ScoreboardObjectiveResponse> {
            // failed

            let result = ScoreboardObjectiveResponse {};
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x53)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SetPassengersResponse {
        pub entity_id: i32,
        pub passengers: Vec<i32>,
    }
    impl<'p> MD<'p> for SetPassengersResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<SetPassengersResponse> {
            let entity_id: i32 = read_varint(&mut reader)?;
            let array_count: i32 = read_varint(&mut reader)?;
            let mut passengers = Vec::with_capacity(cautious_size(array_count as usize));
            for _ in 0..array_count {
                let x: i32 = read_varint(&mut reader)?;
                passengers.push(x);
            }

            let result = SetPassengersResponse {
                entity_id,
                passengers,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut _writer: &mut W) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct TeamsResponse {}
    impl<'p> MD<'p> for TeamsResponse {
        fn deserialize(mut _reader: &mut &[u8]) -> Result<TeamsResponse> {
            // failed

            let result = TeamsResponse {};
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x55)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ScoreboardScoreResponse {}
    impl<'p> MD<'p> for ScoreboardScoreResponse {
        fn deserialize(mut _reader: &mut &[u8]) -> Result<ScoreboardScoreResponse> {
            // failed

            let result = ScoreboardScoreResponse {};
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x56)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SpawnPositionResponse {
        pub location: Position,
        pub angle: f32,
    }
    impl<'p> MD<'p> for SpawnPositionResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<SpawnPositionResponse> {
            let location: Position = MD::deserialize(reader)?;
            let angle: f32 = MD::deserialize(reader)?;

            let result = SpawnPositionResponse { location, angle };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x4b)?;
            self.location.serialize(&mut writer)?;
            self.angle.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UpdateTimeResponse {
        pub age: i64,
        pub time: i64,
    }
    impl<'p> MD<'p> for UpdateTimeResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<UpdateTimeResponse> {
            let age: i64 = MD::deserialize(reader)?;
            let time: i64 = MD::deserialize(reader)?;

            let result = UpdateTimeResponse { age, time };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x59)?;
            self.age.serialize(&mut writer)?;
            self.time.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EntitySoundEffectResponse {
        pub sound_id: i32,
        pub sound_category: i32,
        pub entity_id: i32,
        pub volume: f32,
        pub pitch: f32,
    }
    impl<'p> MD<'p> for EntitySoundEffectResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<EntitySoundEffectResponse> {
            let sound_id: i32 = read_varint(&mut reader)?;
            let sound_category: i32 = read_varint(&mut reader)?;
            let entity_id: i32 = read_varint(&mut reader)?;
            let volume: f32 = MD::deserialize(reader)?;
            let pitch: f32 = MD::deserialize(reader)?;

            let result = EntitySoundEffectResponse {
                sound_id,
                sound_category,
                entity_id,
                volume,
                pitch,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x5c)?;
            write_varint(&mut writer, self.sound_id as u32)?;
            write_varint(&mut writer, self.sound_category as u32)?;
            write_varint(&mut writer, self.entity_id as u32)?;
            self.volume.serialize(&mut writer)?;
            self.pitch.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct StopSoundResponse {}
    impl<'p> MD<'p> for StopSoundResponse {
        fn deserialize(mut _reader: &mut &[u8]) -> Result<StopSoundResponse> {
            // failed

            let result = StopSoundResponse {};
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x5e)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SoundEffectResponse {
        pub sound_id: i32,
        pub sound_category: i32,
        pub x: i32,
        pub y: i32,
        pub z: i32,
        pub volume: f32,
        pub pitch: f32,
    }
    impl<'p> MD<'p> for SoundEffectResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<SoundEffectResponse> {
            let sound_id: i32 = read_varint(&mut reader)?;
            let sound_category: i32 = read_varint(&mut reader)?;
            let x: i32 = MD::deserialize(reader)?;
            let y: i32 = MD::deserialize(reader)?;
            let z: i32 = MD::deserialize(reader)?;
            let volume: f32 = MD::deserialize(reader)?;
            let pitch: f32 = MD::deserialize(reader)?;

            let result = SoundEffectResponse {
                sound_id,
                sound_category,
                x,
                y,
                z,
                volume,
                pitch,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x5d)?;
            write_varint(&mut writer, self.sound_id as u32)?;
            write_varint(&mut writer, self.sound_category as u32)?;
            self.x.serialize(&mut writer)?;
            self.y.serialize(&mut writer)?;
            self.z.serialize(&mut writer)?;
            self.volume.serialize(&mut writer)?;
            self.pitch.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct PlayerlistHeaderResponse<'p> {
        pub header: &'p str,
        pub footer: &'p str,
    }
    impl<'p> MD<'p> for PlayerlistHeaderResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<PlayerlistHeaderResponse<'p>> {
            let header: &'p str = MD::deserialize(reader)?;
            let footer: &'p str = MD::deserialize(reader)?;

            let result = PlayerlistHeaderResponse { header, footer };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x5f)?;
            self.header.serialize(&mut writer)?;
            self.footer.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct CollectResponse {
        pub collected_entity_id: i32,
        pub collector_entity_id: i32,
        pub pickup_item_count: i32,
    }
    impl<'p> MD<'p> for CollectResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<CollectResponse> {
            let collected_entity_id: i32 = read_varint(&mut reader)?;
            let collector_entity_id: i32 = read_varint(&mut reader)?;
            let pickup_item_count: i32 = read_varint(&mut reader)?;

            let result = CollectResponse {
                collected_entity_id,
                collector_entity_id,
                pickup_item_count,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x61)?;
            write_varint(&mut writer, self.collected_entity_id as u32)?;
            write_varint(&mut writer, self.collector_entity_id as u32)?;
            write_varint(&mut writer, self.pickup_item_count as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EntityTeleportResponse {
        pub entity_id: i32,
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub yaw: i8,
        pub pitch: i8,
        pub on_ground: bool,
    }
    impl<'p> MD<'p> for EntityTeleportResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<EntityTeleportResponse> {
            let entity_id: i32 = read_varint(&mut reader)?;
            let x: f64 = MD::deserialize(reader)?;
            let y: f64 = MD::deserialize(reader)?;
            let z: f64 = MD::deserialize(reader)?;
            let yaw: i8 = MD::deserialize(reader)?;
            let pitch: i8 = MD::deserialize(reader)?;
            let on_ground: bool = MD::deserialize(reader)?;

            let result = EntityTeleportResponse {
                entity_id,
                x,
                y,
                z,
                yaw,
                pitch,
                on_ground,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x62)?;
            write_varint(&mut writer, self.entity_id as u32)?;
            self.x.serialize(&mut writer)?;
            self.y.serialize(&mut writer)?;
            self.z.serialize(&mut writer)?;
            self.yaw.serialize(&mut writer)?;
            self.pitch.serialize(&mut writer)?;
            self.on_ground.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EntityUpdateAttributesResponse_Modifiers {
        pub uuid: u128,
        pub amount: f64,
        pub operation: i8,
    }
    impl<'p> MD<'p> for EntityUpdateAttributesResponse_Modifiers {
        fn deserialize(mut reader: &mut &[u8]) -> Result<EntityUpdateAttributesResponse_Modifiers> {
            let uuid: u128 = MD::deserialize(reader)?;
            let amount: f64 = MD::deserialize(reader)?;
            let operation: i8 = MD::deserialize(reader)?;

            let result = EntityUpdateAttributesResponse_Modifiers {
                uuid,
                amount,
                operation,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            self.uuid.serialize(&mut writer)?;
            self.amount.serialize(&mut writer)?;
            self.operation.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EntityUpdateAttributesResponse_Properties<'p> {
        pub key: &'p str,
        pub value: f64,
        pub modifiers: Vec<EntityUpdateAttributesResponse_Modifiers>,
    }
    impl<'p> MD<'p> for EntityUpdateAttributesResponse_Properties<'p> {
        fn deserialize(
            mut reader: &mut &'p [u8],
        ) -> Result<EntityUpdateAttributesResponse_Properties<'p>> {
            let key: &'p str = MD::deserialize(reader)?;
            let value: f64 = MD::deserialize(reader)?;
            let array_count: i32 = read_varint(&mut reader)?;
            let mut modifiers = Vec::with_capacity(cautious_size(array_count as usize));
            for _ in 0..array_count {
                let x: EntityUpdateAttributesResponse_Modifiers =
                    EntityUpdateAttributesResponse_Modifiers::deserialize(reader)?;
                modifiers.push(x);
            }

            let result = EntityUpdateAttributesResponse_Properties {
                key,
                value,
                modifiers,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut _writer: &mut W) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct EntityUpdateAttributesResponse<'p> {
        pub entity_id: i32,
        pub properties: Vec<EntityUpdateAttributesResponse_Properties<'p>>,
    }
    impl<'p> MD<'p> for EntityUpdateAttributesResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<EntityUpdateAttributesResponse<'p>> {
            let entity_id: i32 = read_varint(&mut reader)?;
            let array_count: i32 = read_varint(&mut reader)?;
            let mut properties = Vec::with_capacity(cautious_size(array_count as usize));
            for _ in 0..array_count {
                let x: EntityUpdateAttributesResponse_Properties =
                    EntityUpdateAttributesResponse_Properties::deserialize(reader)?;
                properties.push(x);
            }

            let result = EntityUpdateAttributesResponse {
                entity_id,
                properties,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut _writer: &mut W) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct EntityEffectResponse {
        pub entity_id: i32,
        pub effect_id: i32,
        pub amplifier: i8,
        pub duration: i32,
        pub hide_particles: i8,
    }
    impl<'p> MD<'p> for EntityEffectResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<EntityEffectResponse> {
            let entity_id: i32 = read_varint(&mut reader)?;
            let effect_id: i32 = read_varint(&mut reader)?;
            let amplifier: i8 = MD::deserialize(reader)?;
            let duration: i32 = read_varint(&mut reader)?;
            let hide_particles: i8 = MD::deserialize(reader)?;

            let result = EntityEffectResponse {
                entity_id,
                effect_id,
                amplifier,
                duration,
                hide_particles,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x65)?;
            write_varint(&mut writer, self.entity_id as u32)?;
            write_varint(&mut writer, self.effect_id as u32)?;
            self.amplifier.serialize(&mut writer)?;
            write_varint(&mut writer, self.duration as u32)?;
            self.hide_particles.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SelectAdvancementTabResponse<'p> {
        pub id: Option<&'p str>,
    }
    impl<'p> MD<'p> for SelectAdvancementTabResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<SelectAdvancementTabResponse<'p>> {
            let id: Option<&'p str> = MD::deserialize(reader)?;

            let result = SelectAdvancementTabResponse { id };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x40)?;
            self.id.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct DeclareRecipesResponse_Recipes {}
    impl<'p> MD<'p> for DeclareRecipesResponse_Recipes {
        fn deserialize(mut _reader: &mut &[u8]) -> Result<DeclareRecipesResponse_Recipes> {
            // failed

            let result = DeclareRecipesResponse_Recipes {};
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct DeclareRecipesResponse {
        pub recipes: Vec<DeclareRecipesResponse_Recipes>,
    }
    impl<'p> MD<'p> for DeclareRecipesResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<DeclareRecipesResponse> {
            let array_count: i32 = read_varint(&mut reader)?;
            let mut recipes = Vec::with_capacity(cautious_size(array_count as usize));
            for _ in 0..array_count {
                let x: DeclareRecipesResponse_Recipes =
                    DeclareRecipesResponse_Recipes::deserialize(reader)?;
                recipes.push(x);
            }

            let result = DeclareRecipesResponse { recipes };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut _writer: &mut W) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct TagsResponse_Tags {}
    impl<'p> MD<'p> for TagsResponse_Tags {
        fn deserialize(mut _reader: &mut &[u8]) -> Result<TagsResponse_Tags> {
            // failed

            let result = TagsResponse_Tags {};
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct TagsResponse {
        pub tags: Vec<TagsResponse_Tags>,
    }
    impl<'p> MD<'p> for TagsResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<TagsResponse> {
            let array_count: i32 = read_varint(&mut reader)?;
            let mut tags = Vec::with_capacity(cautious_size(array_count as usize));
            for _ in 0..array_count {
                let x: TagsResponse_Tags = TagsResponse_Tags::deserialize(reader)?;
                tags.push(x);
            }

            let result = TagsResponse { tags };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut _writer: &mut W) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct AcknowledgePlayerDiggingResponse {
        pub location: Position,
        pub block: i32,
        pub status: i32,
        pub successful: bool,
    }
    impl<'p> MD<'p> for AcknowledgePlayerDiggingResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<AcknowledgePlayerDiggingResponse> {
            let location: Position = MD::deserialize(reader)?;
            let block: i32 = read_varint(&mut reader)?;
            let status: i32 = read_varint(&mut reader)?;
            let successful: bool = MD::deserialize(reader)?;

            let result = AcknowledgePlayerDiggingResponse {
                location,
                block,
                status,
                successful,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x8)?;
            self.location.serialize(&mut writer)?;
            write_varint(&mut writer, self.block as u32)?;
            write_varint(&mut writer, self.status as u32)?;
            self.successful.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SculkVibrationSignalResponse {}
    impl<'p> MD<'p> for SculkVibrationSignalResponse {
        fn deserialize(mut _reader: &mut &[u8]) -> Result<SculkVibrationSignalResponse> {
            // failed

            let result = SculkVibrationSignalResponse {};
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x5)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ClearTitlesResponse {
        pub reset: bool,
    }
    impl<'p> MD<'p> for ClearTitlesResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<ClearTitlesResponse> {
            let reset: bool = MD::deserialize(reader)?;

            let result = ClearTitlesResponse { reset };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x10)?;
            self.reset.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct InitializeWorldBorderResponse {
        pub x: f64,
        pub z: f64,
        pub old_diameter: f64,
        pub new_diameter: f64,
        pub speed: i64,
        pub portal_teleport_boundary: i32,
        pub warning_blocks: i32,
        pub warning_time: i32,
    }
    impl<'p> MD<'p> for InitializeWorldBorderResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<InitializeWorldBorderResponse> {
            let x: f64 = MD::deserialize(reader)?;
            let z: f64 = MD::deserialize(reader)?;
            let old_diameter: f64 = MD::deserialize(reader)?;
            let new_diameter: f64 = MD::deserialize(reader)?;
            let speed: i64 = read_varlong(&mut reader)?;
            let portal_teleport_boundary: i32 = read_varint(&mut reader)?;
            let warning_blocks: i32 = read_varint(&mut reader)?;
            let warning_time: i32 = read_varint(&mut reader)?;

            let result = InitializeWorldBorderResponse {
                x,
                z,
                old_diameter,
                new_diameter,
                speed,
                portal_teleport_boundary,
                warning_blocks,
                warning_time,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x20)?;
            self.x.serialize(&mut writer)?;
            self.z.serialize(&mut writer)?;
            self.old_diameter.serialize(&mut writer)?;
            self.new_diameter.serialize(&mut writer)?;
            write_varlong(&mut writer, self.speed as u64)?;
            write_varint(&mut writer, self.portal_teleport_boundary as u32)?;
            write_varint(&mut writer, self.warning_blocks as u32)?;
            write_varint(&mut writer, self.warning_time as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ActionBarResponse<'p> {
        pub text: &'p str,
    }
    impl<'p> MD<'p> for ActionBarResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<ActionBarResponse<'p>> {
            let text: &'p str = MD::deserialize(reader)?;

            let result = ActionBarResponse { text };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x41)?;
            self.text.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct WorldBorderCenterResponse {
        pub x: f64,
        pub z: f64,
    }
    impl<'p> MD<'p> for WorldBorderCenterResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<WorldBorderCenterResponse> {
            let x: f64 = MD::deserialize(reader)?;
            let z: f64 = MD::deserialize(reader)?;

            let result = WorldBorderCenterResponse { x, z };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x42)?;
            self.x.serialize(&mut writer)?;
            self.z.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct WorldBorderLerpSizeResponse {
        pub old_diameter: f64,
        pub new_diameter: f64,
        pub speed: i64,
    }
    impl<'p> MD<'p> for WorldBorderLerpSizeResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<WorldBorderLerpSizeResponse> {
            let old_diameter: f64 = MD::deserialize(reader)?;
            let new_diameter: f64 = MD::deserialize(reader)?;
            let speed: i64 = read_varlong(&mut reader)?;

            let result = WorldBorderLerpSizeResponse {
                old_diameter,
                new_diameter,
                speed,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x43)?;
            self.old_diameter.serialize(&mut writer)?;
            self.new_diameter.serialize(&mut writer)?;
            write_varlong(&mut writer, self.speed as u64)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct WorldBorderSizeResponse {
        pub diameter: f64,
    }
    impl<'p> MD<'p> for WorldBorderSizeResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<WorldBorderSizeResponse> {
            let diameter: f64 = MD::deserialize(reader)?;

            let result = WorldBorderSizeResponse { diameter };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x44)?;
            self.diameter.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct WorldBorderWarningDelayResponse {
        pub warning_time: i32,
    }
    impl<'p> MD<'p> for WorldBorderWarningDelayResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<WorldBorderWarningDelayResponse> {
            let warning_time: i32 = read_varint(&mut reader)?;

            let result = WorldBorderWarningDelayResponse { warning_time };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x45)?;
            write_varint(&mut writer, self.warning_time as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct WorldBorderWarningReachResponse {
        pub warning_blocks: i32,
    }
    impl<'p> MD<'p> for WorldBorderWarningReachResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<WorldBorderWarningReachResponse> {
            let warning_blocks: i32 = read_varint(&mut reader)?;

            let result = WorldBorderWarningReachResponse { warning_blocks };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x46)?;
            write_varint(&mut writer, self.warning_blocks as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct PlayPingResponse {
        pub id: i32,
    }
    impl<'p> MD<'p> for PlayPingResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<PlayPingResponse> {
            let id: i32 = MD::deserialize(reader)?;

            let result = PlayPingResponse { id };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x30)?;
            self.id.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SetTitleSubtitleResponse<'p> {
        pub text: &'p str,
    }
    impl<'p> MD<'p> for SetTitleSubtitleResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<SetTitleSubtitleResponse<'p>> {
            let text: &'p str = MD::deserialize(reader)?;

            let result = SetTitleSubtitleResponse { text };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x58)?;
            self.text.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SetTitleTextResponse<'p> {
        pub text: &'p str,
    }
    impl<'p> MD<'p> for SetTitleTextResponse<'p> {
        fn deserialize(mut reader: &mut &'p [u8]) -> Result<SetTitleTextResponse<'p>> {
            let text: &'p str = MD::deserialize(reader)?;

            let result = SetTitleTextResponse { text };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x5a)?;
            self.text.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SetTitleTimeResponse {
        pub fade_in: i32,
        pub stay: i32,
        pub fade_out: i32,
    }
    impl<'p> MD<'p> for SetTitleTimeResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<SetTitleTimeResponse> {
            let fade_in: i32 = MD::deserialize(reader)?;
            let stay: i32 = MD::deserialize(reader)?;
            let fade_out: i32 = MD::deserialize(reader)?;

            let result = SetTitleTimeResponse {
                fade_in,
                stay,
                fade_out,
            };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x5b)?;
            self.fade_in.serialize(&mut writer)?;
            self.stay.serialize(&mut writer)?;
            self.fade_out.serialize(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SimulationDistanceResponse {
        pub distance: i32,
    }
    impl<'p> MD<'p> for SimulationDistanceResponse {
        fn deserialize(mut reader: &mut &[u8]) -> Result<SimulationDistanceResponse> {
            let distance: i32 = read_varint(&mut reader)?;

            let result = SimulationDistanceResponse { distance };
            Ok(result)
        }
        fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
            write_varint(&mut writer, 0x57)?;
            write_varint(&mut writer, self.distance as u32)?;
            Ok(())
        }
    }
}
#[derive(Debug)]
pub enum Packet<'p> {
    SetProtocolRequest(handshaking::SetProtocolRequest<'p>),
    LegacyServerListPingRequest(handshaking::LegacyServerListPingRequest),
    PingStartRequest(status::PingStartRequest),
    PingRequest(status::PingRequest),
    ServerInfoResponse(status::ServerInfoResponse<'p>),
    PingResponse(status::PingResponse),
    LoginStartRequest(login::LoginStartRequest<'p>),
    EncryptionBeginRequest(login::EncryptionBeginRequest<'p>),
    LoginPluginResponse(login::LoginPluginResponse<'p>),
    DisconnectResponse(login::DisconnectResponse<'p>),
    EncryptionBeginResponse(login::EncryptionBeginResponse<'p>),
    SuccessResponse(login::SuccessResponse<'p>),
    CompressResponse(login::CompressResponse),
    LoginPluginRequest(login::LoginPluginRequest<'p>),
    TeleportConfirmRequest(play::TeleportConfirmRequest),
    QueryBlockNbtRequest(play::QueryBlockNbtRequest),
    SetDifficultyRequest(play::SetDifficultyRequest),
    EditBookRequest(play::EditBookRequest<'p>),
    QueryEntityNbtRequest(play::QueryEntityNbtRequest),
    PickItemRequest(play::PickItemRequest),
    NameItemRequest(play::NameItemRequest<'p>),
    SelectTradeRequest(play::SelectTradeRequest),
    SetBeaconEffectRequest(play::SetBeaconEffectRequest),
    UpdateCommandBlockRequest(play::UpdateCommandBlockRequest<'p>),
    UpdateCommandBlockMinecartRequest(play::UpdateCommandBlockMinecartRequest<'p>),
    UpdateStructureBlockRequest(play::UpdateStructureBlockRequest<'p>),
    TabCompleteRequest(play::TabCompleteRequest<'p>),
    ChatRequest(play::ChatRequest<'p>),
    ClientCommandRequest(play::ClientCommandRequest),
    SettingsRequest(play::SettingsRequest<'p>),
    EnchantItemRequest(play::EnchantItemRequest),
    WindowClickRequest(play::WindowClickRequest<'p>),
    CloseWindowRequest(play::CloseWindowRequest),
    CustomPayloadRequest(play::CustomPayloadRequest<'p>),
    UseEntityRequest(play::UseEntityRequest),
    GenerateStructureRequest(play::GenerateStructureRequest),
    KeepAliveRequest(play::KeepAliveRequest),
    LockDifficultyRequest(play::LockDifficultyRequest),
    PositionRequest(play::PositionRequest),
    PositionLookRequest(play::PositionLookRequest),
    LookRequest(play::LookRequest),
    FlyingRequest(play::FlyingRequest),
    VehicleMoveRequest(play::VehicleMoveRequest),
    SteerBoatRequest(play::SteerBoatRequest),
    CraftRecipeRequest(play::CraftRecipeRequest<'p>),
    AbilitiesRequest(play::AbilitiesRequest),
    BlockDigRequest(play::BlockDigRequest),
    EntityActionRequest(play::EntityActionRequest),
    SteerVehicleRequest(play::SteerVehicleRequest),
    DisplayedRecipeRequest(play::DisplayedRecipeRequest<'p>),
    RecipeBookRequest(play::RecipeBookRequest),
    ResourcePackReceiveRequest(play::ResourcePackReceiveRequest),
    HeldItemSlotRequest(play::HeldItemSlotRequest),
    SetCreativeSlotRequest(play::SetCreativeSlotRequest<'p>),
    UpdateJigsawBlockRequest(play::UpdateJigsawBlockRequest<'p>),
    UpdateSignRequest(play::UpdateSignRequest<'p>),
    ArmAnimationRequest(play::ArmAnimationRequest),
    SpectateRequest(play::SpectateRequest),
    BlockPlaceRequest(play::BlockPlaceRequest),
    UseItemRequest(play::UseItemRequest),
    AdvancementTabRequest(play::AdvancementTabRequest),
    PongRequest(play::PongRequest),
    SpawnEntityResponse(play::SpawnEntityResponse),
    SpawnEntityExperienceOrbResponse(play::SpawnEntityExperienceOrbResponse),
    SpawnEntityLivingResponse(play::SpawnEntityLivingResponse),
    SpawnEntityPaintingResponse(play::SpawnEntityPaintingResponse),
    NamedEntitySpawnResponse(play::NamedEntitySpawnResponse),
    AnimationResponse(play::AnimationResponse),
    StatisticsResponse(play::StatisticsResponse),
    AdvancementsResponse(play::AdvancementsResponse),
    BlockBreakAnimationResponse(play::BlockBreakAnimationResponse),
    TileEntityDataResponse(play::TileEntityDataResponse<'p>),
    BlockActionResponse(play::BlockActionResponse),
    BlockChangeResponse(play::BlockChangeResponse),
    BossBarResponse(play::BossBarResponse),
    DifficultyResponse(play::DifficultyResponse),
    TabCompleteResponse(play::TabCompleteResponse<'p>),
    DeclareCommandsResponse(play::DeclareCommandsResponse),
    FacePlayerResponse(play::FacePlayerResponse),
    NbtQueryResponse(play::NbtQueryResponse<'p>),
    ChatResponse(play::ChatResponse<'p>),
    MultiBlockChangeResponse(play::MultiBlockChangeResponse),
    CloseWindowResponse(play::CloseWindowResponse),
    OpenWindowResponse(play::OpenWindowResponse<'p>),
    WindowItemsResponse(play::WindowItemsResponse<'p>),
    CraftProgressBarResponse(play::CraftProgressBarResponse),
    SetSlotResponse(play::SetSlotResponse<'p>),
    SetCooldownResponse(play::SetCooldownResponse),
    CustomPayloadResponse(play::CustomPayloadResponse<'p>),
    NamedSoundEffectResponse(play::NamedSoundEffectResponse<'p>),
    KickDisconnectResponse(play::KickDisconnectResponse<'p>),
    EntityStatusResponse(play::EntityStatusResponse),
    ExplosionResponse(play::ExplosionResponse),
    UnloadChunkResponse(play::UnloadChunkResponse),
    GameStateChangeResponse(play::GameStateChangeResponse),
    OpenHorseWindowResponse(play::OpenHorseWindowResponse),
    KeepAliveResponse(play::KeepAliveResponse),
    MapChunkResponse(play::MapChunkResponse<'p>),
    WorldEventResponse(play::WorldEventResponse),
    WorldParticlesResponse(play::WorldParticlesResponse),
    UpdateLightResponse(play::UpdateLightResponse<'p>),
    LoginResponse(play::LoginResponse<'p>),
    MapResponse(play::MapResponse),
    TradeListResponse(play::TradeListResponse<'p>),
    RelEntityMoveResponse(play::RelEntityMoveResponse),
    EntityMoveLookResponse(play::EntityMoveLookResponse),
    EntityLookResponse(play::EntityLookResponse),
    VehicleMoveResponse(play::VehicleMoveResponse),
    OpenBookResponse(play::OpenBookResponse),
    OpenSignEntityResponse(play::OpenSignEntityResponse),
    CraftRecipeResponse(play::CraftRecipeResponse<'p>),
    AbilitiesResponse(play::AbilitiesResponse),
    EndCombatEventResponse(play::EndCombatEventResponse),
    EnterCombatEventResponse(play::EnterCombatEventResponse),
    DeathCombatEventResponse(play::DeathCombatEventResponse<'p>),
    PlayerInfoResponse(play::PlayerInfoResponse),
    PositionResponse(play::PositionResponse),
    UnlockRecipesResponse(play::UnlockRecipesResponse),
    EntityDestroyResponse(play::EntityDestroyResponse),
    RemoveEntityEffectResponse(play::RemoveEntityEffectResponse),
    ResourcePackSendResponse(play::ResourcePackSendResponse<'p>),
    RespawnResponse(play::RespawnResponse<'p>),
    EntityHeadRotationResponse(play::EntityHeadRotationResponse),
    CameraResponse(play::CameraResponse),
    HeldItemSlotResponse(play::HeldItemSlotResponse),
    UpdateViewPositionResponse(play::UpdateViewPositionResponse),
    UpdateViewDistanceResponse(play::UpdateViewDistanceResponse),
    ScoreboardDisplayObjectiveResponse(play::ScoreboardDisplayObjectiveResponse<'p>),
    EntityMetadataResponse(play::EntityMetadataResponse),
    AttachEntityResponse(play::AttachEntityResponse),
    EntityVelocityResponse(play::EntityVelocityResponse),
    EntityEquipmentResponse(play::EntityEquipmentResponse),
    ExperienceResponse(play::ExperienceResponse),
    UpdateHealthResponse(play::UpdateHealthResponse),
    ScoreboardObjectiveResponse(play::ScoreboardObjectiveResponse),
    SetPassengersResponse(play::SetPassengersResponse),
    TeamsResponse(play::TeamsResponse),
    ScoreboardScoreResponse(play::ScoreboardScoreResponse),
    SpawnPositionResponse(play::SpawnPositionResponse),
    UpdateTimeResponse(play::UpdateTimeResponse),
    EntitySoundEffectResponse(play::EntitySoundEffectResponse),
    StopSoundResponse(play::StopSoundResponse),
    SoundEffectResponse(play::SoundEffectResponse),
    PlayerlistHeaderResponse(play::PlayerlistHeaderResponse<'p>),
    CollectResponse(play::CollectResponse),
    EntityTeleportResponse(play::EntityTeleportResponse),
    EntityUpdateAttributesResponse(play::EntityUpdateAttributesResponse<'p>),
    EntityEffectResponse(play::EntityEffectResponse),
    SelectAdvancementTabResponse(play::SelectAdvancementTabResponse<'p>),
    DeclareRecipesResponse(play::DeclareRecipesResponse),
    TagsResponse(play::TagsResponse),
    AcknowledgePlayerDiggingResponse(play::AcknowledgePlayerDiggingResponse),
    SculkVibrationSignalResponse(play::SculkVibrationSignalResponse),
    ClearTitlesResponse(play::ClearTitlesResponse),
    InitializeWorldBorderResponse(play::InitializeWorldBorderResponse),
    ActionBarResponse(play::ActionBarResponse<'p>),
    WorldBorderCenterResponse(play::WorldBorderCenterResponse),
    WorldBorderLerpSizeResponse(play::WorldBorderLerpSizeResponse),
    WorldBorderSizeResponse(play::WorldBorderSizeResponse),
    WorldBorderWarningDelayResponse(play::WorldBorderWarningDelayResponse),
    WorldBorderWarningReachResponse(play::WorldBorderWarningReachResponse),
    PlayPingResponse(play::PlayPingResponse),
    SetTitleSubtitleResponse(play::SetTitleSubtitleResponse<'p>),
    SetTitleTextResponse(play::SetTitleTextResponse<'p>),
    SetTitleTimeResponse(play::SetTitleTimeResponse),
    SimulationDistanceResponse(play::SimulationDistanceResponse),
}

pub fn de_packets<'r>(
    state: ConnectionState,
    direction: PacketDirection,
    id: u32,
    reader: &mut &'r [u8],
) -> Result<Packet<'r>> {
    use ConnectionState as S;
    use PacketDirection as D;

    let packet = match (state, direction, id) {
        (S::Handshaking, D::C2S, 0x0) => {
            let p = handshaking::SetProtocolRequest::deserialize(reader)?;
            Packet::SetProtocolRequest(p)
        }
        (S::Handshaking, D::C2S, 0xfe) => {
            let p = handshaking::LegacyServerListPingRequest::deserialize(reader)?;
            Packet::LegacyServerListPingRequest(p)
        }
        (S::Status, D::C2S, 0x0) => {
            let p = status::PingStartRequest::deserialize(reader)?;
            Packet::PingStartRequest(p)
        }
        (S::Status, D::C2S, 0x1) => {
            let p = status::PingRequest::deserialize(reader)?;
            Packet::PingRequest(p)
        }
        (S::Status, D::S2C, 0x0) => {
            let p = status::ServerInfoResponse::deserialize(reader)?;
            Packet::ServerInfoResponse(p)
        }
        (S::Status, D::S2C, 0x1) => {
            let p = status::PingResponse::deserialize(reader)?;
            Packet::PingResponse(p)
        }
        (S::Login, D::C2S, 0x0) => {
            let p = login::LoginStartRequest::deserialize(reader)?;
            Packet::LoginStartRequest(p)
        }
        (S::Login, D::C2S, 0x1) => {
            let p = login::EncryptionBeginRequest::deserialize(reader)?;
            Packet::EncryptionBeginRequest(p)
        }
        (S::Login, D::C2S, 0x2) => {
            let p = login::LoginPluginResponse::deserialize(reader)?;
            Packet::LoginPluginResponse(p)
        }
        (S::Login, D::S2C, 0x0) => {
            let p = login::DisconnectResponse::deserialize(reader)?;
            Packet::DisconnectResponse(p)
        }
        (S::Login, D::S2C, 0x1) => {
            let p = login::EncryptionBeginResponse::deserialize(reader)?;
            Packet::EncryptionBeginResponse(p)
        }
        (S::Login, D::S2C, 0x2) => {
            let p = login::SuccessResponse::deserialize(reader)?;
            Packet::SuccessResponse(p)
        }
        (S::Login, D::S2C, 0x3) => {
            let p = login::CompressResponse::deserialize(reader)?;
            Packet::CompressResponse(p)
        }
        (S::Login, D::S2C, 0x4) => {
            let p = login::LoginPluginRequest::deserialize(reader)?;
            Packet::LoginPluginRequest(p)
        }
        (S::Play, D::C2S, 0x0) => {
            let p = play::TeleportConfirmRequest::deserialize(reader)?;
            Packet::TeleportConfirmRequest(p)
        }
        (S::Play, D::C2S, 0x1) => {
            let p = play::QueryBlockNbtRequest::deserialize(reader)?;
            Packet::QueryBlockNbtRequest(p)
        }
        (S::Play, D::C2S, 0x2) => {
            let p = play::SetDifficultyRequest::deserialize(reader)?;
            Packet::SetDifficultyRequest(p)
        }
        (S::Play, D::C2S, 0x3) => {
            let p = play::ChatRequest::deserialize(reader)?;
            Packet::ChatRequest(p)
        }
        (S::Play, D::C2S, 0x4) => {
            let p = play::ClientCommandRequest::deserialize(reader)?;
            Packet::ClientCommandRequest(p)
        }
        (S::Play, D::C2S, 0x5) => {
            let p = play::SettingsRequest::deserialize(reader)?;
            Packet::SettingsRequest(p)
        }
        (S::Play, D::C2S, 0x6) => {
            let p = play::TabCompleteRequest::deserialize(reader)?;
            Packet::TabCompleteRequest(p)
        }
        (S::Play, D::C2S, 0x7) => {
            let p = play::EnchantItemRequest::deserialize(reader)?;
            Packet::EnchantItemRequest(p)
        }
        (S::Play, D::C2S, 0x8) => {
            let p = play::WindowClickRequest::deserialize(reader)?;
            Packet::WindowClickRequest(p)
        }
        (S::Play, D::C2S, 0x9) => {
            let p = play::CloseWindowRequest::deserialize(reader)?;
            Packet::CloseWindowRequest(p)
        }
        (S::Play, D::C2S, 0xa) => {
            let p = play::CustomPayloadRequest::deserialize(reader)?;
            Packet::CustomPayloadRequest(p)
        }
        (S::Play, D::C2S, 0xb) => {
            let p = play::EditBookRequest::deserialize(reader)?;
            Packet::EditBookRequest(p)
        }
        (S::Play, D::C2S, 0xc) => {
            let p = play::QueryEntityNbtRequest::deserialize(reader)?;
            Packet::QueryEntityNbtRequest(p)
        }
        (S::Play, D::C2S, 0xd) => {
            let p = play::UseEntityRequest::deserialize(reader)?;
            Packet::UseEntityRequest(p)
        }
        (S::Play, D::C2S, 0xe) => {
            let p = play::GenerateStructureRequest::deserialize(reader)?;
            Packet::GenerateStructureRequest(p)
        }
        (S::Play, D::C2S, 0xf) => {
            let p = play::KeepAliveRequest::deserialize(reader)?;
            Packet::KeepAliveRequest(p)
        }
        (S::Play, D::C2S, 0x10) => {
            let p = play::LockDifficultyRequest::deserialize(reader)?;
            Packet::LockDifficultyRequest(p)
        }
        (S::Play, D::C2S, 0x11) => {
            let p = play::PositionRequest::deserialize(reader)?;
            Packet::PositionRequest(p)
        }
        (S::Play, D::C2S, 0x12) => {
            let p = play::PositionLookRequest::deserialize(reader)?;
            Packet::PositionLookRequest(p)
        }
        (S::Play, D::C2S, 0x13) => {
            let p = play::LookRequest::deserialize(reader)?;
            Packet::LookRequest(p)
        }
        (S::Play, D::C2S, 0x14) => {
            let p = play::FlyingRequest::deserialize(reader)?;
            Packet::FlyingRequest(p)
        }
        (S::Play, D::C2S, 0x15) => {
            let p = play::VehicleMoveRequest::deserialize(reader)?;
            Packet::VehicleMoveRequest(p)
        }
        (S::Play, D::C2S, 0x16) => {
            let p = play::SteerBoatRequest::deserialize(reader)?;
            Packet::SteerBoatRequest(p)
        }
        (S::Play, D::C2S, 0x17) => {
            let p = play::PickItemRequest::deserialize(reader)?;
            Packet::PickItemRequest(p)
        }
        (S::Play, D::C2S, 0x18) => {
            let p = play::CraftRecipeRequest::deserialize(reader)?;
            Packet::CraftRecipeRequest(p)
        }
        (S::Play, D::C2S, 0x19) => {
            let p = play::AbilitiesRequest::deserialize(reader)?;
            Packet::AbilitiesRequest(p)
        }
        (S::Play, D::C2S, 0x1a) => {
            let p = play::BlockDigRequest::deserialize(reader)?;
            Packet::BlockDigRequest(p)
        }
        (S::Play, D::C2S, 0x1b) => {
            let p = play::EntityActionRequest::deserialize(reader)?;
            Packet::EntityActionRequest(p)
        }
        (S::Play, D::C2S, 0x1c) => {
            let p = play::SteerVehicleRequest::deserialize(reader)?;
            Packet::SteerVehicleRequest(p)
        }
        (S::Play, D::C2S, 0x1d) => {
            let p = play::PongRequest::deserialize(reader)?;
            Packet::PongRequest(p)
        }
        (S::Play, D::C2S, 0x1e) => {
            let p = play::RecipeBookRequest::deserialize(reader)?;
            Packet::RecipeBookRequest(p)
        }
        (S::Play, D::C2S, 0x1f) => {
            let p = play::DisplayedRecipeRequest::deserialize(reader)?;
            Packet::DisplayedRecipeRequest(p)
        }
        (S::Play, D::C2S, 0x20) => {
            let p = play::NameItemRequest::deserialize(reader)?;
            Packet::NameItemRequest(p)
        }
        (S::Play, D::C2S, 0x21) => {
            let p = play::ResourcePackReceiveRequest::deserialize(reader)?;
            Packet::ResourcePackReceiveRequest(p)
        }
        (S::Play, D::C2S, 0x22) => {
            let p = play::AdvancementTabRequest::deserialize(reader)?;
            Packet::AdvancementTabRequest(p)
        }
        (S::Play, D::C2S, 0x23) => {
            let p = play::SelectTradeRequest::deserialize(reader)?;
            Packet::SelectTradeRequest(p)
        }
        (S::Play, D::C2S, 0x24) => {
            let p = play::SetBeaconEffectRequest::deserialize(reader)?;
            Packet::SetBeaconEffectRequest(p)
        }
        (S::Play, D::C2S, 0x25) => {
            let p = play::HeldItemSlotRequest::deserialize(reader)?;
            Packet::HeldItemSlotRequest(p)
        }
        (S::Play, D::C2S, 0x26) => {
            let p = play::UpdateCommandBlockRequest::deserialize(reader)?;
            Packet::UpdateCommandBlockRequest(p)
        }
        (S::Play, D::C2S, 0x27) => {
            let p = play::UpdateCommandBlockMinecartRequest::deserialize(reader)?;
            Packet::UpdateCommandBlockMinecartRequest(p)
        }
        (S::Play, D::C2S, 0x28) => {
            let p = play::SetCreativeSlotRequest::deserialize(reader)?;
            Packet::SetCreativeSlotRequest(p)
        }
        (S::Play, D::C2S, 0x29) => {
            let p = play::UpdateJigsawBlockRequest::deserialize(reader)?;
            Packet::UpdateJigsawBlockRequest(p)
        }
        (S::Play, D::C2S, 0x2a) => {
            let p = play::UpdateStructureBlockRequest::deserialize(reader)?;
            Packet::UpdateStructureBlockRequest(p)
        }
        (S::Play, D::C2S, 0x2b) => {
            let p = play::UpdateSignRequest::deserialize(reader)?;
            Packet::UpdateSignRequest(p)
        }
        (S::Play, D::C2S, 0x2c) => {
            let p = play::ArmAnimationRequest::deserialize(reader)?;
            Packet::ArmAnimationRequest(p)
        }
        (S::Play, D::C2S, 0x2d) => {
            let p = play::SpectateRequest::deserialize(reader)?;
            Packet::SpectateRequest(p)
        }
        (S::Play, D::C2S, 0x2e) => {
            let p = play::BlockPlaceRequest::deserialize(reader)?;
            Packet::BlockPlaceRequest(p)
        }
        (S::Play, D::C2S, 0x2f) => {
            let p = play::UseItemRequest::deserialize(reader)?;
            Packet::UseItemRequest(p)
        }
        (S::Play, D::S2C, 0x0) => {
            let p = play::SpawnEntityResponse::deserialize(reader)?;
            Packet::SpawnEntityResponse(p)
        }
        (S::Play, D::S2C, 0x1) => {
            let p = play::SpawnEntityExperienceOrbResponse::deserialize(reader)?;
            Packet::SpawnEntityExperienceOrbResponse(p)
        }
        (S::Play, D::S2C, 0x2) => {
            let p = play::SpawnEntityLivingResponse::deserialize(reader)?;
            Packet::SpawnEntityLivingResponse(p)
        }
        (S::Play, D::S2C, 0x3) => {
            let p = play::SpawnEntityPaintingResponse::deserialize(reader)?;
            Packet::SpawnEntityPaintingResponse(p)
        }
        (S::Play, D::S2C, 0x4) => {
            let p = play::NamedEntitySpawnResponse::deserialize(reader)?;
            Packet::NamedEntitySpawnResponse(p)
        }
        (S::Play, D::S2C, 0x5) => {
            let p = play::SculkVibrationSignalResponse::deserialize(reader)?;
            Packet::SculkVibrationSignalResponse(p)
        }
        (S::Play, D::S2C, 0x6) => {
            let p = play::AnimationResponse::deserialize(reader)?;
            Packet::AnimationResponse(p)
        }
        (S::Play, D::S2C, 0x7) => {
            let p = play::StatisticsResponse::deserialize(reader)?;
            Packet::StatisticsResponse(p)
        }
        (S::Play, D::S2C, 0x8) => {
            let p = play::AcknowledgePlayerDiggingResponse::deserialize(reader)?;
            Packet::AcknowledgePlayerDiggingResponse(p)
        }
        (S::Play, D::S2C, 0x9) => {
            let p = play::BlockBreakAnimationResponse::deserialize(reader)?;
            Packet::BlockBreakAnimationResponse(p)
        }
        (S::Play, D::S2C, 0xa) => {
            let p = play::TileEntityDataResponse::deserialize(reader)?;
            Packet::TileEntityDataResponse(p)
        }
        (S::Play, D::S2C, 0xb) => {
            let p = play::BlockActionResponse::deserialize(reader)?;
            Packet::BlockActionResponse(p)
        }
        (S::Play, D::S2C, 0xc) => {
            let p = play::BlockChangeResponse::deserialize(reader)?;
            Packet::BlockChangeResponse(p)
        }
        (S::Play, D::S2C, 0xd) => {
            let p = play::BossBarResponse::deserialize(reader)?;
            Packet::BossBarResponse(p)
        }
        (S::Play, D::S2C, 0xe) => {
            let p = play::DifficultyResponse::deserialize(reader)?;
            Packet::DifficultyResponse(p)
        }
        (S::Play, D::S2C, 0xf) => {
            let p = play::ChatResponse::deserialize(reader)?;
            Packet::ChatResponse(p)
        }
        (S::Play, D::S2C, 0x10) => {
            let p = play::ClearTitlesResponse::deserialize(reader)?;
            Packet::ClearTitlesResponse(p)
        }
        (S::Play, D::S2C, 0x11) => {
            let p = play::TabCompleteResponse::deserialize(reader)?;
            Packet::TabCompleteResponse(p)
        }
        (S::Play, D::S2C, 0x12) => {
            let p = play::DeclareCommandsResponse::deserialize(reader)?;
            Packet::DeclareCommandsResponse(p)
        }
        (S::Play, D::S2C, 0x13) => {
            let p = play::CloseWindowResponse::deserialize(reader)?;
            Packet::CloseWindowResponse(p)
        }
        (S::Play, D::S2C, 0x14) => {
            let p = play::WindowItemsResponse::deserialize(reader)?;
            Packet::WindowItemsResponse(p)
        }
        (S::Play, D::S2C, 0x15) => {
            let p = play::CraftProgressBarResponse::deserialize(reader)?;
            Packet::CraftProgressBarResponse(p)
        }
        (S::Play, D::S2C, 0x16) => {
            let p = play::SetSlotResponse::deserialize(reader)?;
            Packet::SetSlotResponse(p)
        }
        (S::Play, D::S2C, 0x17) => {
            let p = play::SetCooldownResponse::deserialize(reader)?;
            Packet::SetCooldownResponse(p)
        }
        (S::Play, D::S2C, 0x18) => {
            let p = play::CustomPayloadResponse::deserialize(reader)?;
            Packet::CustomPayloadResponse(p)
        }
        (S::Play, D::S2C, 0x19) => {
            let p = play::NamedSoundEffectResponse::deserialize(reader)?;
            Packet::NamedSoundEffectResponse(p)
        }
        (S::Play, D::S2C, 0x1a) => {
            let p = play::KickDisconnectResponse::deserialize(reader)?;
            Packet::KickDisconnectResponse(p)
        }
        (S::Play, D::S2C, 0x1b) => {
            let p = play::EntityStatusResponse::deserialize(reader)?;
            Packet::EntityStatusResponse(p)
        }
        (S::Play, D::S2C, 0x1c) => {
            let p = play::ExplosionResponse::deserialize(reader)?;
            Packet::ExplosionResponse(p)
        }
        (S::Play, D::S2C, 0x1d) => {
            let p = play::UnloadChunkResponse::deserialize(reader)?;
            Packet::UnloadChunkResponse(p)
        }
        (S::Play, D::S2C, 0x1e) => {
            let p = play::GameStateChangeResponse::deserialize(reader)?;
            Packet::GameStateChangeResponse(p)
        }
        (S::Play, D::S2C, 0x1f) => {
            let p = play::OpenHorseWindowResponse::deserialize(reader)?;
            Packet::OpenHorseWindowResponse(p)
        }
        (S::Play, D::S2C, 0x20) => {
            let p = play::InitializeWorldBorderResponse::deserialize(reader)?;
            Packet::InitializeWorldBorderResponse(p)
        }
        (S::Play, D::S2C, 0x21) => {
            let p = play::KeepAliveResponse::deserialize(reader)?;
            Packet::KeepAliveResponse(p)
        }
        (S::Play, D::S2C, 0x22) => {
            let p = play::MapChunkResponse::deserialize(reader)?;
            Packet::MapChunkResponse(p)
        }
        (S::Play, D::S2C, 0x23) => {
            let p = play::WorldEventResponse::deserialize(reader)?;
            Packet::WorldEventResponse(p)
        }
        (S::Play, D::S2C, 0x24) => {
            let p = play::WorldParticlesResponse::deserialize(reader)?;
            Packet::WorldParticlesResponse(p)
        }
        (S::Play, D::S2C, 0x25) => {
            let p = play::UpdateLightResponse::deserialize(reader)?;
            Packet::UpdateLightResponse(p)
        }
        (S::Play, D::S2C, 0x26) => {
            let p = play::LoginResponse::deserialize(reader)?;
            Packet::LoginResponse(p)
        }
        (S::Play, D::S2C, 0x27) => {
            let p = play::MapResponse::deserialize(reader)?;
            Packet::MapResponse(p)
        }
        (S::Play, D::S2C, 0x28) => {
            let p = play::TradeListResponse::deserialize(reader)?;
            Packet::TradeListResponse(p)
        }
        (S::Play, D::S2C, 0x29) => {
            let p = play::RelEntityMoveResponse::deserialize(reader)?;
            Packet::RelEntityMoveResponse(p)
        }
        (S::Play, D::S2C, 0x2a) => {
            let p = play::EntityMoveLookResponse::deserialize(reader)?;
            Packet::EntityMoveLookResponse(p)
        }
        (S::Play, D::S2C, 0x2b) => {
            let p = play::EntityLookResponse::deserialize(reader)?;
            Packet::EntityLookResponse(p)
        }
        (S::Play, D::S2C, 0x2c) => {
            let p = play::VehicleMoveResponse::deserialize(reader)?;
            Packet::VehicleMoveResponse(p)
        }
        (S::Play, D::S2C, 0x2d) => {
            let p = play::OpenBookResponse::deserialize(reader)?;
            Packet::OpenBookResponse(p)
        }
        (S::Play, D::S2C, 0x2e) => {
            let p = play::OpenWindowResponse::deserialize(reader)?;
            Packet::OpenWindowResponse(p)
        }
        (S::Play, D::S2C, 0x2f) => {
            let p = play::OpenSignEntityResponse::deserialize(reader)?;
            Packet::OpenSignEntityResponse(p)
        }
        (S::Play, D::S2C, 0x30) => {
            let p = play::PlayPingResponse::deserialize(reader)?;
            Packet::PlayPingResponse(p)
        }
        (S::Play, D::S2C, 0x31) => {
            let p = play::CraftRecipeResponse::deserialize(reader)?;
            Packet::CraftRecipeResponse(p)
        }
        (S::Play, D::S2C, 0x32) => {
            let p = play::AbilitiesResponse::deserialize(reader)?;
            Packet::AbilitiesResponse(p)
        }
        (S::Play, D::S2C, 0x33) => {
            let p = play::EndCombatEventResponse::deserialize(reader)?;
            Packet::EndCombatEventResponse(p)
        }
        (S::Play, D::S2C, 0x34) => {
            let p = play::EnterCombatEventResponse::deserialize(reader)?;
            Packet::EnterCombatEventResponse(p)
        }
        (S::Play, D::S2C, 0x35) => {
            let p = play::DeathCombatEventResponse::deserialize(reader)?;
            Packet::DeathCombatEventResponse(p)
        }
        (S::Play, D::S2C, 0x36) => {
            let p = play::PlayerInfoResponse::deserialize(reader)?;
            Packet::PlayerInfoResponse(p)
        }
        (S::Play, D::S2C, 0x37) => {
            let p = play::FacePlayerResponse::deserialize(reader)?;
            Packet::FacePlayerResponse(p)
        }
        (S::Play, D::S2C, 0x38) => {
            let p = play::PositionResponse::deserialize(reader)?;
            Packet::PositionResponse(p)
        }
        (S::Play, D::S2C, 0x39) => {
            let p = play::UnlockRecipesResponse::deserialize(reader)?;
            Packet::UnlockRecipesResponse(p)
        }
        (S::Play, D::S2C, 0x3a) => {
            let p = play::EntityDestroyResponse::deserialize(reader)?;
            Packet::EntityDestroyResponse(p)
        }
        (S::Play, D::S2C, 0x3b) => {
            let p = play::RemoveEntityEffectResponse::deserialize(reader)?;
            Packet::RemoveEntityEffectResponse(p)
        }
        (S::Play, D::S2C, 0x3c) => {
            let p = play::ResourcePackSendResponse::deserialize(reader)?;
            Packet::ResourcePackSendResponse(p)
        }
        (S::Play, D::S2C, 0x3d) => {
            let p = play::RespawnResponse::deserialize(reader)?;
            Packet::RespawnResponse(p)
        }
        (S::Play, D::S2C, 0x3e) => {
            let p = play::EntityHeadRotationResponse::deserialize(reader)?;
            Packet::EntityHeadRotationResponse(p)
        }
        (S::Play, D::S2C, 0x3f) => {
            let p = play::MultiBlockChangeResponse::deserialize(reader)?;
            Packet::MultiBlockChangeResponse(p)
        }
        (S::Play, D::S2C, 0x40) => {
            let p = play::SelectAdvancementTabResponse::deserialize(reader)?;
            Packet::SelectAdvancementTabResponse(p)
        }
        (S::Play, D::S2C, 0x41) => {
            let p = play::ActionBarResponse::deserialize(reader)?;
            Packet::ActionBarResponse(p)
        }
        (S::Play, D::S2C, 0x42) => {
            let p = play::WorldBorderCenterResponse::deserialize(reader)?;
            Packet::WorldBorderCenterResponse(p)
        }
        (S::Play, D::S2C, 0x43) => {
            let p = play::WorldBorderLerpSizeResponse::deserialize(reader)?;
            Packet::WorldBorderLerpSizeResponse(p)
        }
        (S::Play, D::S2C, 0x44) => {
            let p = play::WorldBorderSizeResponse::deserialize(reader)?;
            Packet::WorldBorderSizeResponse(p)
        }
        (S::Play, D::S2C, 0x45) => {
            let p = play::WorldBorderWarningDelayResponse::deserialize(reader)?;
            Packet::WorldBorderWarningDelayResponse(p)
        }
        (S::Play, D::S2C, 0x46) => {
            let p = play::WorldBorderWarningReachResponse::deserialize(reader)?;
            Packet::WorldBorderWarningReachResponse(p)
        }
        (S::Play, D::S2C, 0x47) => {
            let p = play::CameraResponse::deserialize(reader)?;
            Packet::CameraResponse(p)
        }
        (S::Play, D::S2C, 0x48) => {
            let p = play::HeldItemSlotResponse::deserialize(reader)?;
            Packet::HeldItemSlotResponse(p)
        }
        (S::Play, D::S2C, 0x49) => {
            let p = play::UpdateViewPositionResponse::deserialize(reader)?;
            Packet::UpdateViewPositionResponse(p)
        }
        (S::Play, D::S2C, 0x4a) => {
            let p = play::UpdateViewDistanceResponse::deserialize(reader)?;
            Packet::UpdateViewDistanceResponse(p)
        }
        (S::Play, D::S2C, 0x4b) => {
            let p = play::SpawnPositionResponse::deserialize(reader)?;
            Packet::SpawnPositionResponse(p)
        }
        (S::Play, D::S2C, 0x4c) => {
            let p = play::ScoreboardDisplayObjectiveResponse::deserialize(reader)?;
            Packet::ScoreboardDisplayObjectiveResponse(p)
        }
        (S::Play, D::S2C, 0x4d) => {
            let p = play::EntityMetadataResponse::deserialize(reader)?;
            Packet::EntityMetadataResponse(p)
        }
        (S::Play, D::S2C, 0x4e) => {
            let p = play::AttachEntityResponse::deserialize(reader)?;
            Packet::AttachEntityResponse(p)
        }
        (S::Play, D::S2C, 0x4f) => {
            let p = play::EntityVelocityResponse::deserialize(reader)?;
            Packet::EntityVelocityResponse(p)
        }
        (S::Play, D::S2C, 0x50) => {
            let p = play::EntityEquipmentResponse::deserialize(reader)?;
            Packet::EntityEquipmentResponse(p)
        }
        (S::Play, D::S2C, 0x51) => {
            let p = play::ExperienceResponse::deserialize(reader)?;
            Packet::ExperienceResponse(p)
        }
        (S::Play, D::S2C, 0x52) => {
            let p = play::UpdateHealthResponse::deserialize(reader)?;
            Packet::UpdateHealthResponse(p)
        }
        (S::Play, D::S2C, 0x53) => {
            let p = play::ScoreboardObjectiveResponse::deserialize(reader)?;
            Packet::ScoreboardObjectiveResponse(p)
        }
        (S::Play, D::S2C, 0x54) => {
            let p = play::SetPassengersResponse::deserialize(reader)?;
            Packet::SetPassengersResponse(p)
        }
        (S::Play, D::S2C, 0x55) => {
            let p = play::TeamsResponse::deserialize(reader)?;
            Packet::TeamsResponse(p)
        }
        (S::Play, D::S2C, 0x56) => {
            let p = play::ScoreboardScoreResponse::deserialize(reader)?;
            Packet::ScoreboardScoreResponse(p)
        }
        (S::Play, D::S2C, 0x57) => {
            let p = play::SimulationDistanceResponse::deserialize(reader)?;
            Packet::SimulationDistanceResponse(p)
        }
        (S::Play, D::S2C, 0x58) => {
            let p = play::SetTitleSubtitleResponse::deserialize(reader)?;
            Packet::SetTitleSubtitleResponse(p)
        }
        (S::Play, D::S2C, 0x59) => {
            let p = play::UpdateTimeResponse::deserialize(reader)?;
            Packet::UpdateTimeResponse(p)
        }
        (S::Play, D::S2C, 0x5a) => {
            let p = play::SetTitleTextResponse::deserialize(reader)?;
            Packet::SetTitleTextResponse(p)
        }
        (S::Play, D::S2C, 0x5b) => {
            let p = play::SetTitleTimeResponse::deserialize(reader)?;
            Packet::SetTitleTimeResponse(p)
        }
        (S::Play, D::S2C, 0x5c) => {
            let p = play::EntitySoundEffectResponse::deserialize(reader)?;
            Packet::EntitySoundEffectResponse(p)
        }
        (S::Play, D::S2C, 0x5d) => {
            let p = play::SoundEffectResponse::deserialize(reader)?;
            Packet::SoundEffectResponse(p)
        }
        (S::Play, D::S2C, 0x5e) => {
            let p = play::StopSoundResponse::deserialize(reader)?;
            Packet::StopSoundResponse(p)
        }
        (S::Play, D::S2C, 0x5f) => {
            let p = play::PlayerlistHeaderResponse::deserialize(reader)?;
            Packet::PlayerlistHeaderResponse(p)
        }
        (S::Play, D::S2C, 0x60) => {
            let p = play::NbtQueryResponse::deserialize(reader)?;
            Packet::NbtQueryResponse(p)
        }
        (S::Play, D::S2C, 0x61) => {
            let p = play::CollectResponse::deserialize(reader)?;
            Packet::CollectResponse(p)
        }
        (S::Play, D::S2C, 0x62) => {
            let p = play::EntityTeleportResponse::deserialize(reader)?;
            Packet::EntityTeleportResponse(p)
        }
        (S::Play, D::S2C, 0x63) => {
            let p = play::AdvancementsResponse::deserialize(reader)?;
            Packet::AdvancementsResponse(p)
        }
        (S::Play, D::S2C, 0x64) => {
            let p = play::EntityUpdateAttributesResponse::deserialize(reader)?;
            Packet::EntityUpdateAttributesResponse(p)
        }
        (S::Play, D::S2C, 0x65) => {
            let p = play::EntityEffectResponse::deserialize(reader)?;
            Packet::EntityEffectResponse(p)
        }
        (S::Play, D::S2C, 0x66) => {
            let p = play::DeclareRecipesResponse::deserialize(reader)?;
            Packet::DeclareRecipesResponse(p)
        }
        (S::Play, D::S2C, 0x67) => {
            let p = play::TagsResponse::deserialize(reader)?;
            Packet::TagsResponse(p)
        }
        _ => {
            return Err(anyhow!("unknown packet id={}", id));
        }
    };
    Ok(packet)
}
