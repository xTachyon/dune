#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(clippy::needless_borrow)]
// fix
#![allow(unreachable_code)]
#![allow(unused_variables)]
// fix

use crate::protocol::de::Position;
use crate::protocol::de::Reader;
use crate::protocol::de::MD;
use crate::protocol::varint::read_varint;
use crate::protocol::varint::read_varlong;
use crate::protocol::varint::write_varint;
use crate::protocol::varint::write_varlong;
use crate::protocol::ConnectionState;
use crate::protocol::IndexedBuffer;
use crate::protocol::IndexedNbt;
use crate::protocol::IndexedOptionNbt;
use crate::protocol::IndexedString;
use crate::protocol::InventorySlot;
use crate::protocol::PacketDirection;
use anyhow::{anyhow, Result};
use byteorder::WriteBytesExt;
use byteorder::BE;
use std::io::{Result as IoResult, Write};

pub mod handshaking {
    use super::*;

    #[derive(Debug)]
    pub struct SetProtocolRequest {
        pub protocol_version: i32,
        pub server_host: IndexedString,
        pub server_port: u16,
        pub next_state: i32,
    }
    pub(super) fn read_set_protocol_request(mut reader: &mut Reader) -> Result<SetProtocolRequest> {
        let protocol_version: i32 = read_varint(&mut reader)?;
        let server_host: IndexedString = MD::deserialize(reader)?;
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
    impl SetProtocolRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            protocol_version: i32,
            server_host: &str,
            server_port: u16,
            next_state: i32,
        ) -> IoResult<()> {
            write_varint(&mut writer, protocol_version as u32)?;
            write_varint(&mut writer, server_host.len() as u32)?;
            writer.write_all(server_host.as_bytes())?;
            writer.write_u16::<BE>(server_port)?;
            write_varint(&mut writer, next_state as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct LegacyServerListPingRequest {
        pub payload: u8,
    }
    pub(super) fn read_legacy_server_list_ping_request(
        mut reader: &mut Reader,
    ) -> Result<LegacyServerListPingRequest> {
        let payload: u8 = MD::deserialize(reader)?;

        let result = LegacyServerListPingRequest { payload };
        Ok(result)
    }
    impl LegacyServerListPingRequest {
        pub(crate) fn write<W: Write>(mut writer: &mut W, payload: u8) -> IoResult<()> {
            writer.write_u8(payload)?;
            Ok(())
        }
    }
}
pub mod status {
    use super::*;

    #[derive(Debug)]
    pub struct PingStartRequest {}
    pub(super) fn read_ping_start_request(mut _reader: &mut Reader) -> Result<PingStartRequest> {
        let result = PingStartRequest {};
        Ok(result)
    }
    impl PingStartRequest {
        pub(crate) fn write<W: Write>(mut _writer: &mut W) -> IoResult<()> {
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct PingRequest {
        pub time: i64,
    }
    pub(super) fn read_ping_request(mut reader: &mut Reader) -> Result<PingRequest> {
        let time: i64 = MD::deserialize(reader)?;

        let result = PingRequest { time };
        Ok(result)
    }
    impl PingRequest {
        pub(crate) fn write<W: Write>(mut writer: &mut W, time: i64) -> IoResult<()> {
            writer.write_i64::<BE>(time)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ServerInfoResponse {
        pub response: IndexedString,
    }
    pub(super) fn read_server_info_response(mut reader: &mut Reader) -> Result<ServerInfoResponse> {
        let response: IndexedString = MD::deserialize(reader)?;

        let result = ServerInfoResponse { response };
        Ok(result)
    }
    impl ServerInfoResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, response: &str) -> IoResult<()> {
            write_varint(&mut writer, response.len() as u32)?;
            writer.write_all(response.as_bytes())?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct PingResponse {
        pub time: i64,
    }
    pub(super) fn read_ping_response(mut reader: &mut Reader) -> Result<PingResponse> {
        let time: i64 = MD::deserialize(reader)?;

        let result = PingResponse { time };
        Ok(result)
    }
    impl PingResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, time: i64) -> IoResult<()> {
            writer.write_i64::<BE>(time)?;
            Ok(())
        }
    }
}
pub mod login {
    use super::*;

    #[derive(Debug)]
    pub struct LoginStartRequest {
        pub username: IndexedString,
    }
    pub(super) fn read_login_start_request(mut reader: &mut Reader) -> Result<LoginStartRequest> {
        let username: IndexedString = MD::deserialize(reader)?;

        let result = LoginStartRequest { username };
        Ok(result)
    }
    impl LoginStartRequest {
        pub(crate) fn write<W: Write>(mut writer: &mut W, username: &str) -> IoResult<()> {
            write_varint(&mut writer, username.len() as u32)?;
            writer.write_all(username.as_bytes())?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EncryptionBeginRequest {
        pub shared_secret: IndexedBuffer,
        pub verify_token: IndexedBuffer,
    }
    pub(super) fn read_encryption_begin_request(
        mut reader: &mut Reader,
    ) -> Result<EncryptionBeginRequest> {
        let shared_secret: IndexedBuffer = MD::deserialize(reader)?;
        let verify_token: IndexedBuffer = MD::deserialize(reader)?;

        let result = EncryptionBeginRequest {
            shared_secret,
            verify_token,
        };
        Ok(result)
    }
    impl EncryptionBeginRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            shared_secret: &[u8],
            verify_token: &[u8],
        ) -> IoResult<()> {
            write_varint(&mut writer, shared_secret.len() as u32)?;
            writer.write_all(shared_secret)?;
            write_varint(&mut writer, verify_token.len() as u32)?;
            writer.write_all(verify_token)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct LoginPluginResponse {
        pub message_id: i32,
        pub data: Option<IndexedBuffer>,
    }
    pub(super) fn read_login_plugin_response(
        mut reader: &mut Reader,
    ) -> Result<LoginPluginResponse> {
        let message_id: i32 = read_varint(&mut reader)?;
        let data: Option<IndexedBuffer> = MD::deserialize(reader)?;

        let result = LoginPluginResponse { message_id, data };
        Ok(result)
    }
    impl LoginPluginResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            message_id: i32,
            data: Option<&[u8]>,
        ) -> IoResult<()> {
            write_varint(&mut writer, message_id as u32)?;
            match data {
                Some(data_1) => {
                    writer.write_all(&[1])?;
                    writer.write_all(data_1)?;
                }
                None => writer.write_all(&[0])?,
            }
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct DisconnectResponse {
        pub reason: IndexedString,
    }
    pub(super) fn read_disconnect_response(mut reader: &mut Reader) -> Result<DisconnectResponse> {
        let reason: IndexedString = MD::deserialize(reader)?;

        let result = DisconnectResponse { reason };
        Ok(result)
    }
    impl DisconnectResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, reason: &str) -> IoResult<()> {
            write_varint(&mut writer, reason.len() as u32)?;
            writer.write_all(reason.as_bytes())?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EncryptionBeginResponse {
        pub server_id: IndexedString,
        pub public_key: IndexedBuffer,
        pub verify_token: IndexedBuffer,
    }
    pub(super) fn read_encryption_begin_response(
        mut reader: &mut Reader,
    ) -> Result<EncryptionBeginResponse> {
        let server_id: IndexedString = MD::deserialize(reader)?;
        let public_key: IndexedBuffer = MD::deserialize(reader)?;
        let verify_token: IndexedBuffer = MD::deserialize(reader)?;

        let result = EncryptionBeginResponse {
            server_id,
            public_key,
            verify_token,
        };
        Ok(result)
    }
    impl EncryptionBeginResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            server_id: &str,
            public_key: &[u8],
            verify_token: &[u8],
        ) -> IoResult<()> {
            write_varint(&mut writer, server_id.len() as u32)?;
            writer.write_all(server_id.as_bytes())?;
            write_varint(&mut writer, public_key.len() as u32)?;
            writer.write_all(public_key)?;
            write_varint(&mut writer, verify_token.len() as u32)?;
            writer.write_all(verify_token)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SuccessResponse {
        pub uuid: u128,
        pub username: IndexedString,
    }
    pub(super) fn read_success_response(mut reader: &mut Reader) -> Result<SuccessResponse> {
        let uuid: u128 = MD::deserialize(reader)?;
        let username: IndexedString = MD::deserialize(reader)?;

        let result = SuccessResponse { uuid, username };
        Ok(result)
    }
    impl SuccessResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            uuid: u128,
            username: &str,
        ) -> IoResult<()> {
            writer.write_u128::<BE>(uuid)?;
            write_varint(&mut writer, username.len() as u32)?;
            writer.write_all(username.as_bytes())?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct CompressResponse {
        pub threshold: i32,
    }
    pub(super) fn read_compress_response(mut reader: &mut Reader) -> Result<CompressResponse> {
        let threshold: i32 = read_varint(&mut reader)?;

        let result = CompressResponse { threshold };
        Ok(result)
    }
    impl CompressResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, threshold: i32) -> IoResult<()> {
            write_varint(&mut writer, threshold as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct LoginPluginRequest {
        pub message_id: i32,
        pub channel: IndexedString,
        pub data: IndexedBuffer,
    }
    pub(super) fn read_login_plugin_request(mut reader: &mut Reader) -> Result<LoginPluginRequest> {
        let message_id: i32 = read_varint(&mut reader)?;
        let channel: IndexedString = MD::deserialize(reader)?;
        let data: IndexedBuffer = reader.read_rest_buffer();

        let result = LoginPluginRequest {
            message_id,
            channel,
            data,
        };
        Ok(result)
    }
    impl LoginPluginRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            message_id: i32,
            channel: &str,
            data: &[u8],
        ) -> IoResult<()> {
            write_varint(&mut writer, message_id as u32)?;
            write_varint(&mut writer, channel.len() as u32)?;
            writer.write_all(channel.as_bytes())?;
            writer.write_all(data)?;
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
    pub(super) fn read_teleport_confirm_request(
        mut reader: &mut Reader,
    ) -> Result<TeleportConfirmRequest> {
        let teleport_id: i32 = read_varint(&mut reader)?;

        let result = TeleportConfirmRequest { teleport_id };
        Ok(result)
    }
    impl TeleportConfirmRequest {
        pub(crate) fn write<W: Write>(mut writer: &mut W, teleport_id: i32) -> IoResult<()> {
            write_varint(&mut writer, teleport_id as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct QueryBlockNbtRequest {
        pub transaction_id: i32,
        pub location: Position,
    }
    pub(super) fn read_query_block_nbt_request(
        mut reader: &mut Reader,
    ) -> Result<QueryBlockNbtRequest> {
        let transaction_id: i32 = read_varint(&mut reader)?;
        let location: Position = MD::deserialize(reader)?;

        let result = QueryBlockNbtRequest {
            transaction_id,
            location,
        };
        Ok(result)
    }
    impl QueryBlockNbtRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            transaction_id: i32,
            location: Position,
        ) -> IoResult<()> {
            write_varint(&mut writer, transaction_id as u32)?;
            location.write(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SetDifficultyRequest {
        pub new_difficulty: u8,
    }
    pub(super) fn read_set_difficulty_request(
        mut reader: &mut Reader,
    ) -> Result<SetDifficultyRequest> {
        let new_difficulty: u8 = MD::deserialize(reader)?;

        let result = SetDifficultyRequest { new_difficulty };
        Ok(result)
    }
    impl SetDifficultyRequest {
        pub(crate) fn write<W: Write>(mut writer: &mut W, new_difficulty: u8) -> IoResult<()> {
            writer.write_u8(new_difficulty)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EditBookRequest {
        pub hand: i32,
        pub pages: Vec<IndexedString>,
        pub title: Option<IndexedString>,
    }
    pub(super) fn read_edit_book_request(mut reader: &mut Reader) -> Result<EditBookRequest> {
        let hand: i32 = read_varint(&mut reader)?;
        let count_array: i32 = read_varint(&mut reader)?;
        let mut pages = Vec::with_capacity(count_array as usize);
        for _ in 0..count_array {
            let x: IndexedString = MD::deserialize(reader)?;
            pages.push(x);
        }
        let title: Option<IndexedString> = MD::deserialize(reader)?;

        let result = EditBookRequest { hand, pages, title };
        Ok(result)
    }
    impl EditBookRequest {
        pub(crate) fn write<W: Write>(
            mut _writer: &mut W,
            _hand: i32,
            _pages: &[&str],
            _title: Option<&str>,
        ) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct QueryEntityNbtRequest {
        pub transaction_id: i32,
        pub entity_id: i32,
    }
    pub(super) fn read_query_entity_nbt_request(
        mut reader: &mut Reader,
    ) -> Result<QueryEntityNbtRequest> {
        let transaction_id: i32 = read_varint(&mut reader)?;
        let entity_id: i32 = read_varint(&mut reader)?;

        let result = QueryEntityNbtRequest {
            transaction_id,
            entity_id,
        };
        Ok(result)
    }
    impl QueryEntityNbtRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            transaction_id: i32,
            entity_id: i32,
        ) -> IoResult<()> {
            write_varint(&mut writer, transaction_id as u32)?;
            write_varint(&mut writer, entity_id as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct PickItemRequest {
        pub slot: i32,
    }
    pub(super) fn read_pick_item_request(mut reader: &mut Reader) -> Result<PickItemRequest> {
        let slot: i32 = read_varint(&mut reader)?;

        let result = PickItemRequest { slot };
        Ok(result)
    }
    impl PickItemRequest {
        pub(crate) fn write<W: Write>(mut writer: &mut W, slot: i32) -> IoResult<()> {
            write_varint(&mut writer, slot as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct NameItemRequest {
        pub name: IndexedString,
    }
    pub(super) fn read_name_item_request(mut reader: &mut Reader) -> Result<NameItemRequest> {
        let name: IndexedString = MD::deserialize(reader)?;

        let result = NameItemRequest { name };
        Ok(result)
    }
    impl NameItemRequest {
        pub(crate) fn write<W: Write>(mut writer: &mut W, name: &str) -> IoResult<()> {
            write_varint(&mut writer, name.len() as u32)?;
            writer.write_all(name.as_bytes())?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SelectTradeRequest {
        pub slot: i32,
    }
    pub(super) fn read_select_trade_request(mut reader: &mut Reader) -> Result<SelectTradeRequest> {
        let slot: i32 = read_varint(&mut reader)?;

        let result = SelectTradeRequest { slot };
        Ok(result)
    }
    impl SelectTradeRequest {
        pub(crate) fn write<W: Write>(mut writer: &mut W, slot: i32) -> IoResult<()> {
            write_varint(&mut writer, slot as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SetBeaconEffectRequest {
        pub primary_effect: i32,
        pub secondary_effect: i32,
    }
    pub(super) fn read_set_beacon_effect_request(
        mut reader: &mut Reader,
    ) -> Result<SetBeaconEffectRequest> {
        let primary_effect: i32 = read_varint(&mut reader)?;
        let secondary_effect: i32 = read_varint(&mut reader)?;

        let result = SetBeaconEffectRequest {
            primary_effect,
            secondary_effect,
        };
        Ok(result)
    }
    impl SetBeaconEffectRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            primary_effect: i32,
            secondary_effect: i32,
        ) -> IoResult<()> {
            write_varint(&mut writer, primary_effect as u32)?;
            write_varint(&mut writer, secondary_effect as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UpdateCommandBlockRequest {
        pub location: Position,
        pub command: IndexedString,
        pub mode: i32,
        pub flags: u8,
    }
    pub(super) fn read_update_command_block_request(
        mut reader: &mut Reader,
    ) -> Result<UpdateCommandBlockRequest> {
        let location: Position = MD::deserialize(reader)?;
        let command: IndexedString = MD::deserialize(reader)?;
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
    impl UpdateCommandBlockRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            location: Position,
            command: &str,
            mode: i32,
            flags: u8,
        ) -> IoResult<()> {
            location.write(&mut writer)?;
            write_varint(&mut writer, command.len() as u32)?;
            writer.write_all(command.as_bytes())?;
            write_varint(&mut writer, mode as u32)?;
            writer.write_u8(flags)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UpdateCommandBlockMinecartRequest {
        pub entity_id: i32,
        pub command: IndexedString,
        pub track_output: bool,
    }
    pub(super) fn read_update_command_block_minecart_request(
        mut reader: &mut Reader,
    ) -> Result<UpdateCommandBlockMinecartRequest> {
        let entity_id: i32 = read_varint(&mut reader)?;
        let command: IndexedString = MD::deserialize(reader)?;
        let track_output: bool = MD::deserialize(reader)?;

        let result = UpdateCommandBlockMinecartRequest {
            entity_id,
            command,
            track_output,
        };
        Ok(result)
    }
    impl UpdateCommandBlockMinecartRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            entity_id: i32,
            command: &str,
            track_output: bool,
        ) -> IoResult<()> {
            write_varint(&mut writer, entity_id as u32)?;
            write_varint(&mut writer, command.len() as u32)?;
            writer.write_all(command.as_bytes())?;
            writer.write_all(&[track_output as u8])?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UpdateStructureBlockRequest {
        pub location: Position,
        pub action: i32,
        pub mode: i32,
        pub name: IndexedString,
        pub offset_x: i8,
        pub offset_y: i8,
        pub offset_z: i8,
        pub size_x: i8,
        pub size_y: i8,
        pub size_z: i8,
        pub mirror: i32,
        pub rotation: i32,
        pub metadata: IndexedString,
        pub integrity: f32,
        pub seed: i64,
        pub flags: u8,
    }
    pub(super) fn read_update_structure_block_request(
        mut reader: &mut Reader,
    ) -> Result<UpdateStructureBlockRequest> {
        let location: Position = MD::deserialize(reader)?;
        let action: i32 = read_varint(&mut reader)?;
        let mode: i32 = read_varint(&mut reader)?;
        let name: IndexedString = MD::deserialize(reader)?;
        let offset_x: i8 = MD::deserialize(reader)?;
        let offset_y: i8 = MD::deserialize(reader)?;
        let offset_z: i8 = MD::deserialize(reader)?;
        let size_x: i8 = MD::deserialize(reader)?;
        let size_y: i8 = MD::deserialize(reader)?;
        let size_z: i8 = MD::deserialize(reader)?;
        let mirror: i32 = read_varint(&mut reader)?;
        let rotation: i32 = read_varint(&mut reader)?;
        let metadata: IndexedString = MD::deserialize(reader)?;
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
    impl UpdateStructureBlockRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            location: Position,
            action: i32,
            mode: i32,
            name: &str,
            offset_x: i8,
            offset_y: i8,
            offset_z: i8,
            size_x: i8,
            size_y: i8,
            size_z: i8,
            mirror: i32,
            rotation: i32,
            metadata: &str,
            integrity: f32,
            seed: i64,
            flags: u8,
        ) -> IoResult<()> {
            location.write(&mut writer)?;
            write_varint(&mut writer, action as u32)?;
            write_varint(&mut writer, mode as u32)?;
            write_varint(&mut writer, name.len() as u32)?;
            writer.write_all(name.as_bytes())?;
            writer.write_i8(offset_x)?;
            writer.write_i8(offset_y)?;
            writer.write_i8(offset_z)?;
            writer.write_i8(size_x)?;
            writer.write_i8(size_y)?;
            writer.write_i8(size_z)?;
            write_varint(&mut writer, mirror as u32)?;
            write_varint(&mut writer, rotation as u32)?;
            write_varint(&mut writer, metadata.len() as u32)?;
            writer.write_all(metadata.as_bytes())?;
            writer.write_f32::<BE>(integrity)?;
            write_varlong(&mut writer, seed as u64)?;
            writer.write_u8(flags)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct TabCompleteRequest {
        pub transaction_id: i32,
        pub text: IndexedString,
    }
    pub(super) fn read_tab_complete_request(mut reader: &mut Reader) -> Result<TabCompleteRequest> {
        let transaction_id: i32 = read_varint(&mut reader)?;
        let text: IndexedString = MD::deserialize(reader)?;

        let result = TabCompleteRequest {
            transaction_id,
            text,
        };
        Ok(result)
    }
    impl TabCompleteRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            transaction_id: i32,
            text: &str,
        ) -> IoResult<()> {
            write_varint(&mut writer, transaction_id as u32)?;
            write_varint(&mut writer, text.len() as u32)?;
            writer.write_all(text.as_bytes())?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ChatRequest {
        pub message: IndexedString,
    }
    pub(super) fn read_chat_request(mut reader: &mut Reader) -> Result<ChatRequest> {
        let message: IndexedString = MD::deserialize(reader)?;

        let result = ChatRequest { message };
        Ok(result)
    }
    impl ChatRequest {
        pub(crate) fn write<W: Write>(mut writer: &mut W, message: &str) -> IoResult<()> {
            write_varint(&mut writer, message.len() as u32)?;
            writer.write_all(message.as_bytes())?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ClientCommandRequest {
        pub action_id: i32,
    }
    pub(super) fn read_client_command_request(
        mut reader: &mut Reader,
    ) -> Result<ClientCommandRequest> {
        let action_id: i32 = read_varint(&mut reader)?;

        let result = ClientCommandRequest { action_id };
        Ok(result)
    }
    impl ClientCommandRequest {
        pub(crate) fn write<W: Write>(mut writer: &mut W, action_id: i32) -> IoResult<()> {
            write_varint(&mut writer, action_id as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SettingsRequest {
        pub locale: IndexedString,
        pub view_distance: i8,
        pub chat_flags: i32,
        pub chat_colors: bool,
        pub skin_parts: u8,
        pub main_hand: i32,
        pub enable_text_filtering: bool,
        pub enable_server_listing: bool,
    }
    pub(super) fn read_settings_request(mut reader: &mut Reader) -> Result<SettingsRequest> {
        let locale: IndexedString = MD::deserialize(reader)?;
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
    impl SettingsRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            locale: &str,
            view_distance: i8,
            chat_flags: i32,
            chat_colors: bool,
            skin_parts: u8,
            main_hand: i32,
            enable_text_filtering: bool,
            enable_server_listing: bool,
        ) -> IoResult<()> {
            write_varint(&mut writer, locale.len() as u32)?;
            writer.write_all(locale.as_bytes())?;
            writer.write_i8(view_distance)?;
            write_varint(&mut writer, chat_flags as u32)?;
            writer.write_all(&[chat_colors as u8])?;
            writer.write_u8(skin_parts)?;
            write_varint(&mut writer, main_hand as u32)?;
            writer.write_all(&[enable_text_filtering as u8])?;
            writer.write_all(&[enable_server_listing as u8])?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EnchantItemRequest {
        pub window_id: i8,
        pub enchantment: i8,
    }
    pub(super) fn read_enchant_item_request(mut reader: &mut Reader) -> Result<EnchantItemRequest> {
        let window_id: i8 = MD::deserialize(reader)?;
        let enchantment: i8 = MD::deserialize(reader)?;

        let result = EnchantItemRequest {
            window_id,
            enchantment,
        };
        Ok(result)
    }
    impl EnchantItemRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            window_id: i8,
            enchantment: i8,
        ) -> IoResult<()> {
            writer.write_i8(window_id)?;
            writer.write_i8(enchantment)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct WindowClickRequest_ChangedSlots {
        pub location: i16,
        pub item: InventorySlot,
    }
    pub(super) fn read_window_click_request_changed_slots(
        mut reader: &mut Reader,
    ) -> Result<WindowClickRequest_ChangedSlots> {
        let location: i16 = MD::deserialize(reader)?;
        let item: InventorySlot = MD::deserialize(reader)?;

        let result = WindowClickRequest_ChangedSlots { location, item };
        Ok(result)
    }
    impl WindowClickRequest_ChangedSlots {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            location: i16,
            item: InventorySlot,
        ) -> IoResult<()> {
            writer.write_i16::<BE>(location)?;
            unimplemented!();
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct WindowClickRequest {
        pub window_id: u8,
        pub state_id: i32,
        pub slot: i16,
        pub mouse_button: i8,
        pub mode: i32,
        pub changed_slots: Vec<WindowClickRequest_ChangedSlots>,
        pub cursor_item: InventorySlot,
    }
    pub(super) fn read_window_click_request(mut reader: &mut Reader) -> Result<WindowClickRequest> {
        let window_id: u8 = MD::deserialize(reader)?;
        let state_id: i32 = read_varint(&mut reader)?;
        let slot: i16 = MD::deserialize(reader)?;
        let mouse_button: i8 = MD::deserialize(reader)?;
        let mode: i32 = read_varint(&mut reader)?;
        let count_array: i32 = read_varint(&mut reader)?;
        let mut changed_slots = Vec::with_capacity(count_array as usize);
        for _ in 0..count_array {
            let x: WindowClickRequest_ChangedSlots =
                read_window_click_request_changed_slots(reader)?;
            changed_slots.push(x);
        }
        let cursor_item: InventorySlot = MD::deserialize(reader)?;

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
    impl WindowClickRequest {
        pub(crate) fn write<W: Write>(
            mut _writer: &mut W,
            _window_id: u8,
            _state_id: i32,
            _slot: i16,
            _mouse_button: i8,
            _mode: i32,
            _changed_slots: &[WindowClickRequest_ChangedSlots],
            _cursor_item: InventorySlot,
        ) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct CloseWindowRequest {
        pub window_id: u8,
    }
    pub(super) fn read_close_window_request(mut reader: &mut Reader) -> Result<CloseWindowRequest> {
        let window_id: u8 = MD::deserialize(reader)?;

        let result = CloseWindowRequest { window_id };
        Ok(result)
    }
    impl CloseWindowRequest {
        pub(crate) fn write<W: Write>(mut writer: &mut W, window_id: u8) -> IoResult<()> {
            writer.write_u8(window_id)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct CustomPayloadRequest {
        pub channel: IndexedString,
        pub data: IndexedBuffer,
    }
    pub(super) fn read_custom_payload_request(
        mut reader: &mut Reader,
    ) -> Result<CustomPayloadRequest> {
        let channel: IndexedString = MD::deserialize(reader)?;
        let data: IndexedBuffer = reader.read_rest_buffer();

        let result = CustomPayloadRequest { channel, data };
        Ok(result)
    }
    impl CustomPayloadRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            channel: &str,
            data: &[u8],
        ) -> IoResult<()> {
            write_varint(&mut writer, channel.len() as u32)?;
            writer.write_all(channel.as_bytes())?;
            writer.write_all(data)?;
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

    pub(super) fn read_use_entity_request(mut reader: &mut Reader) -> Result<UseEntityRequest> {
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
    #[derive(Debug)]
    pub struct GenerateStructureRequest {
        pub location: Position,
        pub levels: i32,
        pub keep_jigsaws: bool,
    }
    pub(super) fn read_generate_structure_request(
        mut reader: &mut Reader,
    ) -> Result<GenerateStructureRequest> {
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
    impl GenerateStructureRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            location: Position,
            levels: i32,
            keep_jigsaws: bool,
        ) -> IoResult<()> {
            location.write(&mut writer)?;
            write_varint(&mut writer, levels as u32)?;
            writer.write_all(&[keep_jigsaws as u8])?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct KeepAliveRequest {
        pub keep_alive_id: i64,
    }
    pub(super) fn read_keep_alive_request(mut reader: &mut Reader) -> Result<KeepAliveRequest> {
        let keep_alive_id: i64 = MD::deserialize(reader)?;

        let result = KeepAliveRequest { keep_alive_id };
        Ok(result)
    }
    impl KeepAliveRequest {
        pub(crate) fn write<W: Write>(mut writer: &mut W, keep_alive_id: i64) -> IoResult<()> {
            writer.write_i64::<BE>(keep_alive_id)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct LockDifficultyRequest {
        pub locked: bool,
    }
    pub(super) fn read_lock_difficulty_request(
        mut reader: &mut Reader,
    ) -> Result<LockDifficultyRequest> {
        let locked: bool = MD::deserialize(reader)?;

        let result = LockDifficultyRequest { locked };
        Ok(result)
    }
    impl LockDifficultyRequest {
        pub(crate) fn write<W: Write>(mut writer: &mut W, locked: bool) -> IoResult<()> {
            writer.write_all(&[locked as u8])?;
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
    pub(super) fn read_position_request(mut reader: &mut Reader) -> Result<PositionRequest> {
        let x: f64 = MD::deserialize(reader)?;
        let y: f64 = MD::deserialize(reader)?;
        let z: f64 = MD::deserialize(reader)?;
        let on_ground: bool = MD::deserialize(reader)?;

        let result = PositionRequest { x, y, z, on_ground };
        Ok(result)
    }
    impl PositionRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            x: f64,
            y: f64,
            z: f64,
            on_ground: bool,
        ) -> IoResult<()> {
            writer.write_f64::<BE>(x)?;
            writer.write_f64::<BE>(y)?;
            writer.write_f64::<BE>(z)?;
            writer.write_all(&[on_ground as u8])?;
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
    pub(super) fn read_position_look_request(
        mut reader: &mut Reader,
    ) -> Result<PositionLookRequest> {
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
    impl PositionLookRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            x: f64,
            y: f64,
            z: f64,
            yaw: f32,
            pitch: f32,
            on_ground: bool,
        ) -> IoResult<()> {
            writer.write_f64::<BE>(x)?;
            writer.write_f64::<BE>(y)?;
            writer.write_f64::<BE>(z)?;
            writer.write_f32::<BE>(yaw)?;
            writer.write_f32::<BE>(pitch)?;
            writer.write_all(&[on_ground as u8])?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct LookRequest {
        pub yaw: f32,
        pub pitch: f32,
        pub on_ground: bool,
    }
    pub(super) fn read_look_request(mut reader: &mut Reader) -> Result<LookRequest> {
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
    impl LookRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            yaw: f32,
            pitch: f32,
            on_ground: bool,
        ) -> IoResult<()> {
            writer.write_f32::<BE>(yaw)?;
            writer.write_f32::<BE>(pitch)?;
            writer.write_all(&[on_ground as u8])?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct FlyingRequest {
        pub on_ground: bool,
    }
    pub(super) fn read_flying_request(mut reader: &mut Reader) -> Result<FlyingRequest> {
        let on_ground: bool = MD::deserialize(reader)?;

        let result = FlyingRequest { on_ground };
        Ok(result)
    }
    impl FlyingRequest {
        pub(crate) fn write<W: Write>(mut writer: &mut W, on_ground: bool) -> IoResult<()> {
            writer.write_all(&[on_ground as u8])?;
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
    pub(super) fn read_vehicle_move_request(mut reader: &mut Reader) -> Result<VehicleMoveRequest> {
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
    impl VehicleMoveRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            x: f64,
            y: f64,
            z: f64,
            yaw: f32,
            pitch: f32,
        ) -> IoResult<()> {
            writer.write_f64::<BE>(x)?;
            writer.write_f64::<BE>(y)?;
            writer.write_f64::<BE>(z)?;
            writer.write_f32::<BE>(yaw)?;
            writer.write_f32::<BE>(pitch)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SteerBoatRequest {
        pub left_paddle: bool,
        pub right_paddle: bool,
    }
    pub(super) fn read_steer_boat_request(mut reader: &mut Reader) -> Result<SteerBoatRequest> {
        let left_paddle: bool = MD::deserialize(reader)?;
        let right_paddle: bool = MD::deserialize(reader)?;

        let result = SteerBoatRequest {
            left_paddle,
            right_paddle,
        };
        Ok(result)
    }
    impl SteerBoatRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            left_paddle: bool,
            right_paddle: bool,
        ) -> IoResult<()> {
            writer.write_all(&[left_paddle as u8])?;
            writer.write_all(&[right_paddle as u8])?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct CraftRecipeRequest {
        pub window_id: i8,
        pub recipe: IndexedString,
        pub make_all: bool,
    }
    pub(super) fn read_craft_recipe_request(mut reader: &mut Reader) -> Result<CraftRecipeRequest> {
        let window_id: i8 = MD::deserialize(reader)?;
        let recipe: IndexedString = MD::deserialize(reader)?;
        let make_all: bool = MD::deserialize(reader)?;

        let result = CraftRecipeRequest {
            window_id,
            recipe,
            make_all,
        };
        Ok(result)
    }
    impl CraftRecipeRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            window_id: i8,
            recipe: &str,
            make_all: bool,
        ) -> IoResult<()> {
            writer.write_i8(window_id)?;
            write_varint(&mut writer, recipe.len() as u32)?;
            writer.write_all(recipe.as_bytes())?;
            writer.write_all(&[make_all as u8])?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct AbilitiesRequest {
        pub flags: i8,
    }
    pub(super) fn read_abilities_request(mut reader: &mut Reader) -> Result<AbilitiesRequest> {
        let flags: i8 = MD::deserialize(reader)?;

        let result = AbilitiesRequest { flags };
        Ok(result)
    }
    impl AbilitiesRequest {
        pub(crate) fn write<W: Write>(mut writer: &mut W, flags: i8) -> IoResult<()> {
            writer.write_i8(flags)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct BlockDigRequest {
        pub status: i32,
        pub location: Position,
        pub face: i8,
    }
    pub(super) fn read_block_dig_request(mut reader: &mut Reader) -> Result<BlockDigRequest> {
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
    impl BlockDigRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            status: i32,
            location: Position,
            face: i8,
        ) -> IoResult<()> {
            write_varint(&mut writer, status as u32)?;
            location.write(&mut writer)?;
            writer.write_i8(face)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EntityActionRequest {
        pub entity_id: i32,
        pub action_id: i32,
        pub jump_boost: i32,
    }
    pub(super) fn read_entity_action_request(
        mut reader: &mut Reader,
    ) -> Result<EntityActionRequest> {
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
    impl EntityActionRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            entity_id: i32,
            action_id: i32,
            jump_boost: i32,
        ) -> IoResult<()> {
            write_varint(&mut writer, entity_id as u32)?;
            write_varint(&mut writer, action_id as u32)?;
            write_varint(&mut writer, jump_boost as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SteerVehicleRequest {
        pub sideways: f32,
        pub forward: f32,
        pub jump: u8,
    }
    pub(super) fn read_steer_vehicle_request(
        mut reader: &mut Reader,
    ) -> Result<SteerVehicleRequest> {
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
    impl SteerVehicleRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            sideways: f32,
            forward: f32,
            jump: u8,
        ) -> IoResult<()> {
            writer.write_f32::<BE>(sideways)?;
            writer.write_f32::<BE>(forward)?;
            writer.write_u8(jump)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct DisplayedRecipeRequest {
        pub recipe_id: IndexedString,
    }
    pub(super) fn read_displayed_recipe_request(
        mut reader: &mut Reader,
    ) -> Result<DisplayedRecipeRequest> {
        let recipe_id: IndexedString = MD::deserialize(reader)?;

        let result = DisplayedRecipeRequest { recipe_id };
        Ok(result)
    }
    impl DisplayedRecipeRequest {
        pub(crate) fn write<W: Write>(mut writer: &mut W, recipe_id: &str) -> IoResult<()> {
            write_varint(&mut writer, recipe_id.len() as u32)?;
            writer.write_all(recipe_id.as_bytes())?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct RecipeBookRequest {
        pub book_id: i32,
        pub book_open: bool,
        pub filter_active: bool,
    }
    pub(super) fn read_recipe_book_request(mut reader: &mut Reader) -> Result<RecipeBookRequest> {
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
    impl RecipeBookRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            book_id: i32,
            book_open: bool,
            filter_active: bool,
        ) -> IoResult<()> {
            write_varint(&mut writer, book_id as u32)?;
            writer.write_all(&[book_open as u8])?;
            writer.write_all(&[filter_active as u8])?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ResourcePackReceiveRequest {
        pub result: i32,
    }
    pub(super) fn read_resource_pack_receive_request(
        mut reader: &mut Reader,
    ) -> Result<ResourcePackReceiveRequest> {
        let result: i32 = read_varint(&mut reader)?;

        let result = ResourcePackReceiveRequest { result };
        Ok(result)
    }
    impl ResourcePackReceiveRequest {
        pub(crate) fn write<W: Write>(mut writer: &mut W, result: i32) -> IoResult<()> {
            write_varint(&mut writer, result as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct HeldItemSlotRequest {
        pub slot_id: i16,
    }
    pub(super) fn read_held_item_slot_request(
        mut reader: &mut Reader,
    ) -> Result<HeldItemSlotRequest> {
        let slot_id: i16 = MD::deserialize(reader)?;

        let result = HeldItemSlotRequest { slot_id };
        Ok(result)
    }
    impl HeldItemSlotRequest {
        pub(crate) fn write<W: Write>(mut writer: &mut W, slot_id: i16) -> IoResult<()> {
            writer.write_i16::<BE>(slot_id)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SetCreativeSlotRequest {
        pub slot: i16,
        pub item: InventorySlot,
    }
    pub(super) fn read_set_creative_slot_request(
        mut reader: &mut Reader,
    ) -> Result<SetCreativeSlotRequest> {
        let slot: i16 = MD::deserialize(reader)?;
        let item: InventorySlot = MD::deserialize(reader)?;

        let result = SetCreativeSlotRequest { slot, item };
        Ok(result)
    }
    impl SetCreativeSlotRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            slot: i16,
            item: InventorySlot,
        ) -> IoResult<()> {
            writer.write_i16::<BE>(slot)?;
            unimplemented!();
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UpdateJigsawBlockRequest {
        pub location: Position,
        pub name: IndexedString,
        pub target: IndexedString,
        pub pool: IndexedString,
        pub final_state: IndexedString,
        pub joint_type: IndexedString,
    }
    pub(super) fn read_update_jigsaw_block_request(
        mut reader: &mut Reader,
    ) -> Result<UpdateJigsawBlockRequest> {
        let location: Position = MD::deserialize(reader)?;
        let name: IndexedString = MD::deserialize(reader)?;
        let target: IndexedString = MD::deserialize(reader)?;
        let pool: IndexedString = MD::deserialize(reader)?;
        let final_state: IndexedString = MD::deserialize(reader)?;
        let joint_type: IndexedString = MD::deserialize(reader)?;

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
    impl UpdateJigsawBlockRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            location: Position,
            name: &str,
            target: &str,
            pool: &str,
            final_state: &str,
            joint_type: &str,
        ) -> IoResult<()> {
            location.write(&mut writer)?;
            write_varint(&mut writer, name.len() as u32)?;
            writer.write_all(name.as_bytes())?;
            write_varint(&mut writer, target.len() as u32)?;
            writer.write_all(target.as_bytes())?;
            write_varint(&mut writer, pool.len() as u32)?;
            writer.write_all(pool.as_bytes())?;
            write_varint(&mut writer, final_state.len() as u32)?;
            writer.write_all(final_state.as_bytes())?;
            write_varint(&mut writer, joint_type.len() as u32)?;
            writer.write_all(joint_type.as_bytes())?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UpdateSignRequest {
        pub location: Position,
        pub text1: IndexedString,
        pub text2: IndexedString,
        pub text3: IndexedString,
        pub text4: IndexedString,
    }
    pub(super) fn read_update_sign_request(mut reader: &mut Reader) -> Result<UpdateSignRequest> {
        let location: Position = MD::deserialize(reader)?;
        let text1: IndexedString = MD::deserialize(reader)?;
        let text2: IndexedString = MD::deserialize(reader)?;
        let text3: IndexedString = MD::deserialize(reader)?;
        let text4: IndexedString = MD::deserialize(reader)?;

        let result = UpdateSignRequest {
            location,
            text1,
            text2,
            text3,
            text4,
        };
        Ok(result)
    }
    impl UpdateSignRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            location: Position,
            text1: &str,
            text2: &str,
            text3: &str,
            text4: &str,
        ) -> IoResult<()> {
            location.write(&mut writer)?;
            write_varint(&mut writer, text1.len() as u32)?;
            writer.write_all(text1.as_bytes())?;
            write_varint(&mut writer, text2.len() as u32)?;
            writer.write_all(text2.as_bytes())?;
            write_varint(&mut writer, text3.len() as u32)?;
            writer.write_all(text3.as_bytes())?;
            write_varint(&mut writer, text4.len() as u32)?;
            writer.write_all(text4.as_bytes())?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ArmAnimationRequest {
        pub hand: i32,
    }
    pub(super) fn read_arm_animation_request(
        mut reader: &mut Reader,
    ) -> Result<ArmAnimationRequest> {
        let hand: i32 = read_varint(&mut reader)?;

        let result = ArmAnimationRequest { hand };
        Ok(result)
    }
    impl ArmAnimationRequest {
        pub(crate) fn write<W: Write>(mut writer: &mut W, hand: i32) -> IoResult<()> {
            write_varint(&mut writer, hand as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SpectateRequest {
        pub target: u128,
    }
    pub(super) fn read_spectate_request(mut reader: &mut Reader) -> Result<SpectateRequest> {
        let target: u128 = MD::deserialize(reader)?;

        let result = SpectateRequest { target };
        Ok(result)
    }
    impl SpectateRequest {
        pub(crate) fn write<W: Write>(mut writer: &mut W, target: u128) -> IoResult<()> {
            writer.write_u128::<BE>(target)?;
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
    pub(super) fn read_block_place_request(mut reader: &mut Reader) -> Result<BlockPlaceRequest> {
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
    impl BlockPlaceRequest {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            hand: i32,
            location: Position,
            direction: i32,
            cursor_x: f32,
            cursor_y: f32,
            cursor_z: f32,
            inside_block: bool,
        ) -> IoResult<()> {
            write_varint(&mut writer, hand as u32)?;
            location.write(&mut writer)?;
            write_varint(&mut writer, direction as u32)?;
            writer.write_f32::<BE>(cursor_x)?;
            writer.write_f32::<BE>(cursor_y)?;
            writer.write_f32::<BE>(cursor_z)?;
            writer.write_all(&[inside_block as u8])?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UseItemRequest {
        pub hand: i32,
    }
    pub(super) fn read_use_item_request(mut reader: &mut Reader) -> Result<UseItemRequest> {
        let hand: i32 = read_varint(&mut reader)?;

        let result = UseItemRequest { hand };
        Ok(result)
    }
    impl UseItemRequest {
        pub(crate) fn write<W: Write>(mut writer: &mut W, hand: i32) -> IoResult<()> {
            write_varint(&mut writer, hand as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct AdvancementTabRequest {}
    pub(super) fn read_advancement_tab_request(
        mut _reader: &mut Reader,
    ) -> Result<AdvancementTabRequest> {
        let result = AdvancementTabRequest {};
        Ok(result)
    }
    impl AdvancementTabRequest {
        pub(crate) fn write<W: Write>(mut _writer: &mut W) -> IoResult<()> {
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct PongRequest {
        pub id: i32,
    }
    pub(super) fn read_pong_request(mut reader: &mut Reader) -> Result<PongRequest> {
        let id: i32 = MD::deserialize(reader)?;

        let result = PongRequest { id };
        Ok(result)
    }
    impl PongRequest {
        pub(crate) fn write<W: Write>(mut writer: &mut W, id: i32) -> IoResult<()> {
            writer.write_i32::<BE>(id)?;
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
    pub(super) fn read_spawn_entity_response(
        mut reader: &mut Reader,
    ) -> Result<SpawnEntityResponse> {
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
    impl SpawnEntityResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            entity_id: i32,
            object_uuid: u128,
            type_: i32,
            x: f64,
            y: f64,
            z: f64,
            pitch: i8,
            yaw: i8,
            object_data: i32,
            velocity_x: i16,
            velocity_y: i16,
            velocity_z: i16,
        ) -> IoResult<()> {
            write_varint(&mut writer, entity_id as u32)?;
            writer.write_u128::<BE>(object_uuid)?;
            write_varint(&mut writer, type_ as u32)?;
            writer.write_f64::<BE>(x)?;
            writer.write_f64::<BE>(y)?;
            writer.write_f64::<BE>(z)?;
            writer.write_i8(pitch)?;
            writer.write_i8(yaw)?;
            writer.write_i32::<BE>(object_data)?;
            writer.write_i16::<BE>(velocity_x)?;
            writer.write_i16::<BE>(velocity_y)?;
            writer.write_i16::<BE>(velocity_z)?;
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
    pub(super) fn read_spawn_entity_experience_orb_response(
        mut reader: &mut Reader,
    ) -> Result<SpawnEntityExperienceOrbResponse> {
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
    impl SpawnEntityExperienceOrbResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            entity_id: i32,
            x: f64,
            y: f64,
            z: f64,
            count: i16,
        ) -> IoResult<()> {
            write_varint(&mut writer, entity_id as u32)?;
            writer.write_f64::<BE>(x)?;
            writer.write_f64::<BE>(y)?;
            writer.write_f64::<BE>(z)?;
            writer.write_i16::<BE>(count)?;
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
    pub(super) fn read_spawn_entity_living_response(
        mut reader: &mut Reader,
    ) -> Result<SpawnEntityLivingResponse> {
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
    impl SpawnEntityLivingResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            entity_id: i32,
            entity_uuid: u128,
            type_: i32,
            x: f64,
            y: f64,
            z: f64,
            yaw: i8,
            pitch: i8,
            head_pitch: i8,
            velocity_x: i16,
            velocity_y: i16,
            velocity_z: i16,
        ) -> IoResult<()> {
            write_varint(&mut writer, entity_id as u32)?;
            writer.write_u128::<BE>(entity_uuid)?;
            write_varint(&mut writer, type_ as u32)?;
            writer.write_f64::<BE>(x)?;
            writer.write_f64::<BE>(y)?;
            writer.write_f64::<BE>(z)?;
            writer.write_i8(yaw)?;
            writer.write_i8(pitch)?;
            writer.write_i8(head_pitch)?;
            writer.write_i16::<BE>(velocity_x)?;
            writer.write_i16::<BE>(velocity_y)?;
            writer.write_i16::<BE>(velocity_z)?;
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
    pub(super) fn read_spawn_entity_painting_response(
        mut reader: &mut Reader,
    ) -> Result<SpawnEntityPaintingResponse> {
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
    impl SpawnEntityPaintingResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            entity_id: i32,
            entity_uuid: u128,
            title: i32,
            location: Position,
            direction: u8,
        ) -> IoResult<()> {
            write_varint(&mut writer, entity_id as u32)?;
            writer.write_u128::<BE>(entity_uuid)?;
            write_varint(&mut writer, title as u32)?;
            location.write(&mut writer)?;
            writer.write_u8(direction)?;
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
    pub(super) fn read_named_entity_spawn_response(
        mut reader: &mut Reader,
    ) -> Result<NamedEntitySpawnResponse> {
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
    impl NamedEntitySpawnResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            entity_id: i32,
            player_uuid: u128,
            x: f64,
            y: f64,
            z: f64,
            yaw: i8,
            pitch: i8,
        ) -> IoResult<()> {
            write_varint(&mut writer, entity_id as u32)?;
            writer.write_u128::<BE>(player_uuid)?;
            writer.write_f64::<BE>(x)?;
            writer.write_f64::<BE>(y)?;
            writer.write_f64::<BE>(z)?;
            writer.write_i8(yaw)?;
            writer.write_i8(pitch)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct AnimationResponse {
        pub entity_id: i32,
        pub animation: u8,
    }
    pub(super) fn read_animation_response(mut reader: &mut Reader) -> Result<AnimationResponse> {
        let entity_id: i32 = read_varint(&mut reader)?;
        let animation: u8 = MD::deserialize(reader)?;

        let result = AnimationResponse {
            entity_id,
            animation,
        };
        Ok(result)
    }
    impl AnimationResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            entity_id: i32,
            animation: u8,
        ) -> IoResult<()> {
            write_varint(&mut writer, entity_id as u32)?;
            writer.write_u8(animation)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct StatisticsResponse_Entries {
        pub category_id: i32,
        pub statistic_id: i32,
        pub value: i32,
    }
    pub(super) fn read_statistics_response_entries(
        mut reader: &mut Reader,
    ) -> Result<StatisticsResponse_Entries> {
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
    impl StatisticsResponse_Entries {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            category_id: i32,
            statistic_id: i32,
            value: i32,
        ) -> IoResult<()> {
            write_varint(&mut writer, category_id as u32)?;
            write_varint(&mut writer, statistic_id as u32)?;
            write_varint(&mut writer, value as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct StatisticsResponse {
        pub entries: Vec<StatisticsResponse_Entries>,
    }
    pub(super) fn read_statistics_response(mut reader: &mut Reader) -> Result<StatisticsResponse> {
        let count_array: i32 = read_varint(&mut reader)?;
        let mut entries = Vec::with_capacity(count_array as usize);
        for _ in 0..count_array {
            let x: StatisticsResponse_Entries = read_statistics_response_entries(reader)?;
            entries.push(x);
        }

        let result = StatisticsResponse { entries };
        Ok(result)
    }
    impl StatisticsResponse {
        pub(crate) fn write<W: Write>(
            mut _writer: &mut W,
            _entries: &[StatisticsResponse_Entries],
        ) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct AdvancementsResponse {}
    pub(super) fn read_advancements_response(
        mut _reader: &mut Reader,
    ) -> Result<AdvancementsResponse> {
        let result = AdvancementsResponse {};
        Ok(result)
    }
    impl AdvancementsResponse {
        pub(crate) fn write<W: Write>(mut _writer: &mut W) -> IoResult<()> {
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct BlockBreakAnimationResponse {
        pub entity_id: i32,
        pub location: Position,
        pub destroy_stage: i8,
    }
    pub(super) fn read_block_break_animation_response(
        mut reader: &mut Reader,
    ) -> Result<BlockBreakAnimationResponse> {
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
    impl BlockBreakAnimationResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            entity_id: i32,
            location: Position,
            destroy_stage: i8,
        ) -> IoResult<()> {
            write_varint(&mut writer, entity_id as u32)?;
            location.write(&mut writer)?;
            writer.write_i8(destroy_stage)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct TileEntityDataResponse {
        pub location: Position,
        pub action: i32,
        pub nbt_data: IndexedOptionNbt,
    }
    pub(super) fn read_tile_entity_data_response(
        mut reader: &mut Reader,
    ) -> Result<TileEntityDataResponse> {
        let location: Position = MD::deserialize(reader)?;
        let action: i32 = read_varint(&mut reader)?;
        let nbt_data: IndexedOptionNbt = MD::deserialize(reader)?;

        let result = TileEntityDataResponse {
            location,
            action,
            nbt_data,
        };
        Ok(result)
    }
    impl TileEntityDataResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            location: Position,
            action: i32,
            nbt_data: IndexedOptionNbt,
        ) -> IoResult<()> {
            location.write(&mut writer)?;
            write_varint(&mut writer, action as u32)?;
            unimplemented!();
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct BlockActionResponse {
        pub location: Position,
        pub byte1: u8,
        pub byte2: u8,
        pub block_id: i32,
    }
    pub(super) fn read_block_action_response(
        mut reader: &mut Reader,
    ) -> Result<BlockActionResponse> {
        let location: Position = MD::deserialize(reader)?;
        let byte1: u8 = MD::deserialize(reader)?;
        let byte2: u8 = MD::deserialize(reader)?;
        let block_id: i32 = read_varint(&mut reader)?;

        let result = BlockActionResponse {
            location,
            byte1,
            byte2,
            block_id,
        };
        Ok(result)
    }
    impl BlockActionResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            location: Position,
            byte1: u8,
            byte2: u8,
            block_id: i32,
        ) -> IoResult<()> {
            location.write(&mut writer)?;
            writer.write_u8(byte1)?;
            writer.write_u8(byte2)?;
            write_varint(&mut writer, block_id as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct BlockChangeResponse {
        pub location: Position,
        pub type_: i32,
    }
    pub(super) fn read_block_change_response(
        mut reader: &mut Reader,
    ) -> Result<BlockChangeResponse> {
        let location: Position = MD::deserialize(reader)?;
        let type_: i32 = read_varint(&mut reader)?;

        let result = BlockChangeResponse { location, type_ };
        Ok(result)
    }
    impl BlockChangeResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            location: Position,
            type_: i32,
        ) -> IoResult<()> {
            location.write(&mut writer)?;
            write_varint(&mut writer, type_ as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct BossBarResponse {}
    pub(super) fn read_boss_bar_response(mut _reader: &mut Reader) -> Result<BossBarResponse> {
        let result = BossBarResponse {};
        Ok(result)
    }
    impl BossBarResponse {
        pub(crate) fn write<W: Write>(mut _writer: &mut W) -> IoResult<()> {
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct DifficultyResponse {
        pub difficulty: u8,
        pub difficulty_locked: bool,
    }
    pub(super) fn read_difficulty_response(mut reader: &mut Reader) -> Result<DifficultyResponse> {
        let difficulty: u8 = MD::deserialize(reader)?;
        let difficulty_locked: bool = MD::deserialize(reader)?;

        let result = DifficultyResponse {
            difficulty,
            difficulty_locked,
        };
        Ok(result)
    }
    impl DifficultyResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            difficulty: u8,
            difficulty_locked: bool,
        ) -> IoResult<()> {
            writer.write_u8(difficulty)?;
            writer.write_all(&[difficulty_locked as u8])?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct TabCompleteResponse_Matches {
        pub match_: IndexedString,
        pub tooltip: Option<IndexedString>,
    }
    pub(super) fn read_tab_complete_response_matches(
        mut reader: &mut Reader,
    ) -> Result<TabCompleteResponse_Matches> {
        let match_: IndexedString = MD::deserialize(reader)?;
        let tooltip: Option<IndexedString> = MD::deserialize(reader)?;

        let result = TabCompleteResponse_Matches { match_, tooltip };
        Ok(result)
    }
    impl TabCompleteResponse_Matches {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            match_: &str,
            tooltip: Option<&str>,
        ) -> IoResult<()> {
            write_varint(&mut writer, match_.len() as u32)?;
            writer.write_all(match_.as_bytes())?;
            match tooltip {
                Some(tooltip_1) => {
                    writer.write_all(&[1])?;

                    write_varint(&mut writer, tooltip_1.len() as u32)?;
                    writer.write_all(tooltip_1.as_bytes())?;
                }
                None => writer.write_all(&[0])?,
            }
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct TabCompleteResponse {
        pub transaction_id: i32,
        pub start: i32,
        pub length: i32,
        pub matches: Vec<TabCompleteResponse_Matches>,
    }
    pub(super) fn read_tab_complete_response(
        mut reader: &mut Reader,
    ) -> Result<TabCompleteResponse> {
        let transaction_id: i32 = read_varint(&mut reader)?;
        let start: i32 = read_varint(&mut reader)?;
        let length: i32 = read_varint(&mut reader)?;
        let count_array: i32 = read_varint(&mut reader)?;
        let mut matches = Vec::with_capacity(count_array as usize);
        for _ in 0..count_array {
            let x: TabCompleteResponse_Matches = read_tab_complete_response_matches(reader)?;
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
    impl TabCompleteResponse {
        pub(crate) fn write<W: Write>(
            mut _writer: &mut W,
            _transaction_id: i32,
            _start: i32,
            _length: i32,
            _matches: &[TabCompleteResponse_Matches],
        ) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct DeclareCommandsResponse {}
    pub(super) fn read_declare_commands_response(
        mut _reader: &mut Reader,
    ) -> Result<DeclareCommandsResponse> {
        let result = DeclareCommandsResponse {};
        Ok(result)
    }
    impl DeclareCommandsResponse {
        pub(crate) fn write<W: Write>(mut _writer: &mut W) -> IoResult<()> {
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct FacePlayerResponse {}
    pub(super) fn read_face_player_response(
        mut _reader: &mut Reader,
    ) -> Result<FacePlayerResponse> {
        let result = FacePlayerResponse {};
        Ok(result)
    }
    impl FacePlayerResponse {
        pub(crate) fn write<W: Write>(mut _writer: &mut W) -> IoResult<()> {
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct NbtQueryResponse {
        pub transaction_id: i32,
        pub nbt: IndexedOptionNbt,
    }
    pub(super) fn read_nbt_query_response(mut reader: &mut Reader) -> Result<NbtQueryResponse> {
        let transaction_id: i32 = read_varint(&mut reader)?;
        let nbt: IndexedOptionNbt = MD::deserialize(reader)?;

        let result = NbtQueryResponse {
            transaction_id,
            nbt,
        };
        Ok(result)
    }
    impl NbtQueryResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            transaction_id: i32,
            nbt: IndexedOptionNbt,
        ) -> IoResult<()> {
            write_varint(&mut writer, transaction_id as u32)?;
            unimplemented!();
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ChatResponse {
        pub message: IndexedString,
        pub position: i8,
        pub sender: u128,
    }
    pub(super) fn read_chat_response(mut reader: &mut Reader) -> Result<ChatResponse> {
        let message: IndexedString = MD::deserialize(reader)?;
        let position: i8 = MD::deserialize(reader)?;
        let sender: u128 = MD::deserialize(reader)?;

        let result = ChatResponse {
            message,
            position,
            sender,
        };
        Ok(result)
    }
    impl ChatResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            message: &str,
            position: i8,
            sender: u128,
        ) -> IoResult<()> {
            write_varint(&mut writer, message.len() as u32)?;
            writer.write_all(message.as_bytes())?;
            writer.write_i8(position)?;
            writer.write_u128::<BE>(sender)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct MultiBlockChangeResponse {}
    pub(super) fn read_multi_block_change_response(
        mut _reader: &mut Reader,
    ) -> Result<MultiBlockChangeResponse> {
        let result = MultiBlockChangeResponse {};
        Ok(result)
    }
    impl MultiBlockChangeResponse {
        pub(crate) fn write<W: Write>(mut _writer: &mut W) -> IoResult<()> {
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct CloseWindowResponse {
        pub window_id: u8,
    }
    pub(super) fn read_close_window_response(
        mut reader: &mut Reader,
    ) -> Result<CloseWindowResponse> {
        let window_id: u8 = MD::deserialize(reader)?;

        let result = CloseWindowResponse { window_id };
        Ok(result)
    }
    impl CloseWindowResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, window_id: u8) -> IoResult<()> {
            writer.write_u8(window_id)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct OpenWindowResponse {
        pub window_id: i32,
        pub inventory_type: i32,
        pub window_title: IndexedString,
    }
    pub(super) fn read_open_window_response(mut reader: &mut Reader) -> Result<OpenWindowResponse> {
        let window_id: i32 = read_varint(&mut reader)?;
        let inventory_type: i32 = read_varint(&mut reader)?;
        let window_title: IndexedString = MD::deserialize(reader)?;

        let result = OpenWindowResponse {
            window_id,
            inventory_type,
            window_title,
        };
        Ok(result)
    }
    impl OpenWindowResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            window_id: i32,
            inventory_type: i32,
            window_title: &str,
        ) -> IoResult<()> {
            write_varint(&mut writer, window_id as u32)?;
            write_varint(&mut writer, inventory_type as u32)?;
            write_varint(&mut writer, window_title.len() as u32)?;
            writer.write_all(window_title.as_bytes())?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct WindowItemsResponse {
        pub window_id: u8,
        pub state_id: i32,
        pub items: Vec<InventorySlot>,
        pub carried_item: InventorySlot,
    }
    pub(super) fn read_window_items_response(
        mut reader: &mut Reader,
    ) -> Result<WindowItemsResponse> {
        let window_id: u8 = MD::deserialize(reader)?;
        let state_id: i32 = read_varint(&mut reader)?;
        let count_array: i32 = read_varint(&mut reader)?;
        let mut items = Vec::with_capacity(count_array as usize);
        for _ in 0..count_array {
            let x: InventorySlot = MD::deserialize(reader)?;
            items.push(x);
        }
        let carried_item: InventorySlot = MD::deserialize(reader)?;

        let result = WindowItemsResponse {
            window_id,
            state_id,
            items,
            carried_item,
        };
        Ok(result)
    }
    impl WindowItemsResponse {
        pub(crate) fn write<W: Write>(
            mut _writer: &mut W,
            _window_id: u8,
            _state_id: i32,
            _items: &[InventorySlot],
            _carried_item: InventorySlot,
        ) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct CraftProgressBarResponse {
        pub window_id: u8,
        pub property: i16,
        pub value: i16,
    }
    pub(super) fn read_craft_progress_bar_response(
        mut reader: &mut Reader,
    ) -> Result<CraftProgressBarResponse> {
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
    impl CraftProgressBarResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            window_id: u8,
            property: i16,
            value: i16,
        ) -> IoResult<()> {
            writer.write_u8(window_id)?;
            writer.write_i16::<BE>(property)?;
            writer.write_i16::<BE>(value)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SetSlotResponse {
        pub window_id: i8,
        pub state_id: i32,
        pub slot: i16,
        pub item: InventorySlot,
    }
    pub(super) fn read_set_slot_response(mut reader: &mut Reader) -> Result<SetSlotResponse> {
        let window_id: i8 = MD::deserialize(reader)?;
        let state_id: i32 = read_varint(&mut reader)?;
        let slot: i16 = MD::deserialize(reader)?;
        let item: InventorySlot = MD::deserialize(reader)?;

        let result = SetSlotResponse {
            window_id,
            state_id,
            slot,
            item,
        };
        Ok(result)
    }
    impl SetSlotResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            window_id: i8,
            state_id: i32,
            slot: i16,
            item: InventorySlot,
        ) -> IoResult<()> {
            writer.write_i8(window_id)?;
            write_varint(&mut writer, state_id as u32)?;
            writer.write_i16::<BE>(slot)?;
            unimplemented!();
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SetCooldownResponse {
        pub item_id: i32,
        pub cooldown_ticks: i32,
    }
    pub(super) fn read_set_cooldown_response(
        mut reader: &mut Reader,
    ) -> Result<SetCooldownResponse> {
        let item_id: i32 = read_varint(&mut reader)?;
        let cooldown_ticks: i32 = read_varint(&mut reader)?;

        let result = SetCooldownResponse {
            item_id,
            cooldown_ticks,
        };
        Ok(result)
    }
    impl SetCooldownResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            item_id: i32,
            cooldown_ticks: i32,
        ) -> IoResult<()> {
            write_varint(&mut writer, item_id as u32)?;
            write_varint(&mut writer, cooldown_ticks as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct CustomPayloadResponse {
        pub channel: IndexedString,
        pub data: IndexedBuffer,
    }
    pub(super) fn read_custom_payload_response(
        mut reader: &mut Reader,
    ) -> Result<CustomPayloadResponse> {
        let channel: IndexedString = MD::deserialize(reader)?;
        let data: IndexedBuffer = reader.read_rest_buffer();

        let result = CustomPayloadResponse { channel, data };
        Ok(result)
    }
    impl CustomPayloadResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            channel: &str,
            data: &[u8],
        ) -> IoResult<()> {
            write_varint(&mut writer, channel.len() as u32)?;
            writer.write_all(channel.as_bytes())?;
            writer.write_all(data)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct NamedSoundEffectResponse {
        pub sound_name: IndexedString,
        pub sound_category: i32,
        pub x: i32,
        pub y: i32,
        pub z: i32,
        pub volume: f32,
        pub pitch: f32,
    }
    pub(super) fn read_named_sound_effect_response(
        mut reader: &mut Reader,
    ) -> Result<NamedSoundEffectResponse> {
        let sound_name: IndexedString = MD::deserialize(reader)?;
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
    impl NamedSoundEffectResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            sound_name: &str,
            sound_category: i32,
            x: i32,
            y: i32,
            z: i32,
            volume: f32,
            pitch: f32,
        ) -> IoResult<()> {
            write_varint(&mut writer, sound_name.len() as u32)?;
            writer.write_all(sound_name.as_bytes())?;
            write_varint(&mut writer, sound_category as u32)?;
            writer.write_i32::<BE>(x)?;
            writer.write_i32::<BE>(y)?;
            writer.write_i32::<BE>(z)?;
            writer.write_f32::<BE>(volume)?;
            writer.write_f32::<BE>(pitch)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct KickDisconnectResponse {
        pub reason: IndexedString,
    }
    pub(super) fn read_kick_disconnect_response(
        mut reader: &mut Reader,
    ) -> Result<KickDisconnectResponse> {
        let reason: IndexedString = MD::deserialize(reader)?;

        let result = KickDisconnectResponse { reason };
        Ok(result)
    }
    impl KickDisconnectResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, reason: &str) -> IoResult<()> {
            write_varint(&mut writer, reason.len() as u32)?;
            writer.write_all(reason.as_bytes())?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EntityStatusResponse {
        pub entity_id: i32,
        pub entity_status: i8,
    }
    pub(super) fn read_entity_status_response(
        mut reader: &mut Reader,
    ) -> Result<EntityStatusResponse> {
        let entity_id: i32 = MD::deserialize(reader)?;
        let entity_status: i8 = MD::deserialize(reader)?;

        let result = EntityStatusResponse {
            entity_id,
            entity_status,
        };
        Ok(result)
    }
    impl EntityStatusResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            entity_id: i32,
            entity_status: i8,
        ) -> IoResult<()> {
            writer.write_i32::<BE>(entity_id)?;
            writer.write_i8(entity_status)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ExplosionResponse_AffectedBlockOffsets {
        pub x: i8,
        pub y: i8,
        pub z: i8,
    }
    pub(super) fn read_explosion_response_affected_block_offsets(
        mut reader: &mut Reader,
    ) -> Result<ExplosionResponse_AffectedBlockOffsets> {
        let x: i8 = MD::deserialize(reader)?;
        let y: i8 = MD::deserialize(reader)?;
        let z: i8 = MD::deserialize(reader)?;

        let result = ExplosionResponse_AffectedBlockOffsets { x, y, z };
        Ok(result)
    }
    impl ExplosionResponse_AffectedBlockOffsets {
        pub(crate) fn write<W: Write>(mut writer: &mut W, x: i8, y: i8, z: i8) -> IoResult<()> {
            writer.write_i8(x)?;
            writer.write_i8(y)?;
            writer.write_i8(z)?;
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
    pub(super) fn read_explosion_response(mut reader: &mut Reader) -> Result<ExplosionResponse> {
        let x: f32 = MD::deserialize(reader)?;
        let y: f32 = MD::deserialize(reader)?;
        let z: f32 = MD::deserialize(reader)?;
        let radius: f32 = MD::deserialize(reader)?;
        let count_array: i32 = read_varint(&mut reader)?;
        let mut affected_block_offsets = Vec::with_capacity(count_array as usize);
        for _ in 0..count_array {
            let x: ExplosionResponse_AffectedBlockOffsets =
                read_explosion_response_affected_block_offsets(reader)?;
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
    impl ExplosionResponse {
        pub(crate) fn write<W: Write>(
            mut _writer: &mut W,
            _x: f32,
            _y: f32,
            _z: f32,
            _radius: f32,
            _affected_block_offsets: &[ExplosionResponse_AffectedBlockOffsets],
            _player_motion_x: f32,
            _player_motion_y: f32,
            _player_motion_z: f32,
        ) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct UnloadChunkResponse {
        pub chunk_x: i32,
        pub chunk_z: i32,
    }
    pub(super) fn read_unload_chunk_response(
        mut reader: &mut Reader,
    ) -> Result<UnloadChunkResponse> {
        let chunk_x: i32 = MD::deserialize(reader)?;
        let chunk_z: i32 = MD::deserialize(reader)?;

        let result = UnloadChunkResponse { chunk_x, chunk_z };
        Ok(result)
    }
    impl UnloadChunkResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            chunk_x: i32,
            chunk_z: i32,
        ) -> IoResult<()> {
            writer.write_i32::<BE>(chunk_x)?;
            writer.write_i32::<BE>(chunk_z)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct GameStateChangeResponse {
        pub reason: u8,
        pub game_mode: f32,
    }
    pub(super) fn read_game_state_change_response(
        mut reader: &mut Reader,
    ) -> Result<GameStateChangeResponse> {
        let reason: u8 = MD::deserialize(reader)?;
        let game_mode: f32 = MD::deserialize(reader)?;

        let result = GameStateChangeResponse { reason, game_mode };
        Ok(result)
    }
    impl GameStateChangeResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            reason: u8,
            game_mode: f32,
        ) -> IoResult<()> {
            writer.write_u8(reason)?;
            writer.write_f32::<BE>(game_mode)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct OpenHorseWindowResponse {
        pub window_id: u8,
        pub nb_slots: i32,
        pub entity_id: i32,
    }
    pub(super) fn read_open_horse_window_response(
        mut reader: &mut Reader,
    ) -> Result<OpenHorseWindowResponse> {
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
    impl OpenHorseWindowResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            window_id: u8,
            nb_slots: i32,
            entity_id: i32,
        ) -> IoResult<()> {
            writer.write_u8(window_id)?;
            write_varint(&mut writer, nb_slots as u32)?;
            writer.write_i32::<BE>(entity_id)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct KeepAliveResponse {
        pub keep_alive_id: i64,
    }
    pub(super) fn read_keep_alive_response(mut reader: &mut Reader) -> Result<KeepAliveResponse> {
        let keep_alive_id: i64 = MD::deserialize(reader)?;

        let result = KeepAliveResponse { keep_alive_id };
        Ok(result)
    }
    impl KeepAliveResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, keep_alive_id: i64) -> IoResult<()> {
            writer.write_i64::<BE>(keep_alive_id)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct MapChunkResponse {}
    pub(super) fn read_map_chunk_response(mut _reader: &mut Reader) -> Result<MapChunkResponse> {
        let result = MapChunkResponse {};
        Ok(result)
    }
    impl MapChunkResponse {
        pub(crate) fn write<W: Write>(mut _writer: &mut W) -> IoResult<()> {
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct WorldEventResponse {
        pub effect_id: i32,
        pub location: Position,
        pub data: i32,
        pub global: bool,
    }
    pub(super) fn read_world_event_response(mut reader: &mut Reader) -> Result<WorldEventResponse> {
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
    impl WorldEventResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            effect_id: i32,
            location: Position,
            data: i32,
            global: bool,
        ) -> IoResult<()> {
            writer.write_i32::<BE>(effect_id)?;
            location.write(&mut writer)?;
            writer.write_i32::<BE>(data)?;
            writer.write_all(&[global as u8])?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct WorldParticlesResponse {}
    pub(super) fn read_world_particles_response(
        mut _reader: &mut Reader,
    ) -> Result<WorldParticlesResponse> {
        let result = WorldParticlesResponse {};
        Ok(result)
    }
    impl WorldParticlesResponse {
        pub(crate) fn write<W: Write>(mut _writer: &mut W) -> IoResult<()> {
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UpdateLightResponse {
        pub chunk_x: i32,
        pub chunk_z: i32,
        pub trust_edges: bool,
        pub sky_light_mask: Vec<i64>,
        pub block_light_mask: Vec<i64>,
        pub empty_sky_light_mask: Vec<i64>,
        pub empty_block_light_mask: Vec<i64>,
        pub sky_light: Vec<Vec<u8>>,
        pub block_light: Vec<Vec<u8>>,
    }
    pub(super) fn read_update_light_response(
        mut reader: &mut Reader,
    ) -> Result<UpdateLightResponse> {
        let chunk_x: i32 = read_varint(&mut reader)?;
        let chunk_z: i32 = read_varint(&mut reader)?;
        let trust_edges: bool = MD::deserialize(reader)?;
        let count_array: i32 = read_varint(&mut reader)?;
        let mut sky_light_mask = Vec::with_capacity(count_array as usize);
        for _ in 0..count_array {
            let x: i64 = MD::deserialize(reader)?;
            sky_light_mask.push(x);
        }
        let count_array: i32 = read_varint(&mut reader)?;
        let mut block_light_mask = Vec::with_capacity(count_array as usize);
        for _ in 0..count_array {
            let x: i64 = MD::deserialize(reader)?;
            block_light_mask.push(x);
        }
        let count_array: i32 = read_varint(&mut reader)?;
        let mut empty_sky_light_mask = Vec::with_capacity(count_array as usize);
        for _ in 0..count_array {
            let x: i64 = MD::deserialize(reader)?;
            empty_sky_light_mask.push(x);
        }
        let count_array: i32 = read_varint(&mut reader)?;
        let mut empty_block_light_mask = Vec::with_capacity(count_array as usize);
        for _ in 0..count_array {
            let x: i64 = MD::deserialize(reader)?;
            empty_block_light_mask.push(x);
        }
        let count_array: i32 = read_varint(&mut reader)?;
        let mut sky_light = Vec::with_capacity(count_array as usize);
        for _ in 0..count_array {
            let count_array: i32 = read_varint(&mut reader)?;
            let mut x = Vec::with_capacity(count_array as usize);
            for _ in 0..count_array {
                let x_2: u8 = MD::deserialize(reader)?;
                x.push(x_2);
            }
            sky_light.push(x);
        }
        let count_array: i32 = read_varint(&mut reader)?;
        let mut block_light = Vec::with_capacity(count_array as usize);
        for _ in 0..count_array {
            let count_array: i32 = read_varint(&mut reader)?;
            let mut x = Vec::with_capacity(count_array as usize);
            for _ in 0..count_array {
                let x_2: u8 = MD::deserialize(reader)?;
                x.push(x_2);
            }
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
    impl UpdateLightResponse {
        pub(crate) fn write<W: Write>(
            mut _writer: &mut W,
            _chunk_x: i32,
            _chunk_z: i32,
            _trust_edges: bool,
            _sky_light_mask: &[i64],
            _block_light_mask: &[i64],
            _empty_sky_light_mask: &[i64],
            _empty_block_light_mask: &[i64],
            _sky_light: &[&[u8]],
            _block_light: &[&[u8]],
        ) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct LoginResponse {
        pub entity_id: i32,
        pub is_hardcore: bool,
        pub game_mode: u8,
        pub previous_game_mode: i8,
        pub world_names: Vec<IndexedString>,
        pub dimension_codec: IndexedNbt,
        pub dimension: IndexedNbt,
        pub world_name: IndexedString,
        pub hashed_seed: i64,
        pub max_players: i32,
        pub view_distance: i32,
        pub simulation_distance: i32,
        pub reduced_debug_info: bool,
        pub enable_respawn_screen: bool,
        pub is_debug: bool,
        pub is_flat: bool,
    }
    pub(super) fn read_login_response(mut reader: &mut Reader) -> Result<LoginResponse> {
        let entity_id: i32 = MD::deserialize(reader)?;
        let is_hardcore: bool = MD::deserialize(reader)?;
        let game_mode: u8 = MD::deserialize(reader)?;
        let previous_game_mode: i8 = MD::deserialize(reader)?;
        let count_array: i32 = read_varint(&mut reader)?;
        let mut world_names = Vec::with_capacity(count_array as usize);
        for _ in 0..count_array {
            let x: IndexedString = MD::deserialize(reader)?;
            world_names.push(x);
        }
        let dimension_codec: IndexedNbt = MD::deserialize(reader)?;
        let dimension: IndexedNbt = MD::deserialize(reader)?;
        let world_name: IndexedString = MD::deserialize(reader)?;
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
    impl LoginResponse {
        pub(crate) fn write<W: Write>(
            mut _writer: &mut W,
            _entity_id: i32,
            _is_hardcore: bool,
            _game_mode: u8,
            _previous_game_mode: i8,
            _world_names: &[&str],
            _dimension_codec: IndexedNbt,
            _dimension: IndexedNbt,
            _world_name: &str,
            _hashed_seed: i64,
            _max_players: i32,
            _view_distance: i32,
            _simulation_distance: i32,
            _reduced_debug_info: bool,
            _enable_respawn_screen: bool,
            _is_debug: bool,
            _is_flat: bool,
        ) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct MapResponse {}
    pub(super) fn read_map_response(mut _reader: &mut Reader) -> Result<MapResponse> {
        let result = MapResponse {};
        Ok(result)
    }
    impl MapResponse {
        pub(crate) fn write<W: Write>(mut _writer: &mut W) -> IoResult<()> {
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct TradeListResponse_Trades {
        pub input_item1: InventorySlot,
        pub output_item: InventorySlot,
        pub input_item2: Option<InventorySlot>,
        pub trade_disabled: bool,
        pub nb_trade_uses: i32,
        pub maximum_nb_trade_uses: i32,
        pub xp: i32,
        pub special_price: i32,
        pub price_multiplier: f32,
        pub demand: i32,
    }
    pub(super) fn read_trade_list_response_trades(
        mut reader: &mut Reader,
    ) -> Result<TradeListResponse_Trades> {
        let input_item1: InventorySlot = MD::deserialize(reader)?;
        let output_item: InventorySlot = MD::deserialize(reader)?;
        let input_item2: Option<InventorySlot> = MD::deserialize(reader)?;
        let trade_disabled: bool = MD::deserialize(reader)?;
        let nb_trade_uses: i32 = MD::deserialize(reader)?;
        let maximum_nb_trade_uses: i32 = MD::deserialize(reader)?;
        let xp: i32 = MD::deserialize(reader)?;
        let special_price: i32 = MD::deserialize(reader)?;
        let price_multiplier: f32 = MD::deserialize(reader)?;
        let demand: i32 = MD::deserialize(reader)?;

        let result = TradeListResponse_Trades {
            input_item1,
            output_item,
            input_item2,
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
    impl TradeListResponse_Trades {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            input_item1: InventorySlot,
            output_item: InventorySlot,
            input_item2: Option<InventorySlot>,
            trade_disabled: bool,
            nb_trade_uses: i32,
            maximum_nb_trade_uses: i32,
            xp: i32,
            special_price: i32,
            price_multiplier: f32,
            demand: i32,
        ) -> IoResult<()> {
            unimplemented!();
            unimplemented!();
            match input_item2 {
                Some(input_item2_1) => {
                    writer.write_all(&[1])?;
                    unimplemented!();
                }
                None => writer.write_all(&[0])?,
            }
            writer.write_all(&[trade_disabled as u8])?;
            writer.write_i32::<BE>(nb_trade_uses)?;
            writer.write_i32::<BE>(maximum_nb_trade_uses)?;
            writer.write_i32::<BE>(xp)?;
            writer.write_i32::<BE>(special_price)?;
            writer.write_f32::<BE>(price_multiplier)?;
            writer.write_i32::<BE>(demand)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct TradeListResponse {
        pub window_id: i32,
        pub trades: Vec<TradeListResponse_Trades>,
        pub villager_level: i32,
        pub experience: i32,
        pub is_regular_villager: bool,
        pub can_restock: bool,
    }
    pub(super) fn read_trade_list_response(mut reader: &mut Reader) -> Result<TradeListResponse> {
        let window_id: i32 = read_varint(&mut reader)?;
        let count_array: u8 = MD::deserialize(reader)?;
        let mut trades = Vec::with_capacity(count_array as usize);
        for _ in 0..count_array {
            let x: TradeListResponse_Trades = read_trade_list_response_trades(reader)?;
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
    impl TradeListResponse {
        pub(crate) fn write<W: Write>(
            mut _writer: &mut W,
            _window_id: i32,
            _trades: &[TradeListResponse_Trades],
            _villager_level: i32,
            _experience: i32,
            _is_regular_villager: bool,
            _can_restock: bool,
        ) -> IoResult<()> {
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
    pub(super) fn read_rel_entity_move_response(
        mut reader: &mut Reader,
    ) -> Result<RelEntityMoveResponse> {
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
    impl RelEntityMoveResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            entity_id: i32,
            d_x: i16,
            d_y: i16,
            d_z: i16,
            on_ground: bool,
        ) -> IoResult<()> {
            write_varint(&mut writer, entity_id as u32)?;
            writer.write_i16::<BE>(d_x)?;
            writer.write_i16::<BE>(d_y)?;
            writer.write_i16::<BE>(d_z)?;
            writer.write_all(&[on_ground as u8])?;
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
    pub(super) fn read_entity_move_look_response(
        mut reader: &mut Reader,
    ) -> Result<EntityMoveLookResponse> {
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
    impl EntityMoveLookResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            entity_id: i32,
            d_x: i16,
            d_y: i16,
            d_z: i16,
            yaw: i8,
            pitch: i8,
            on_ground: bool,
        ) -> IoResult<()> {
            write_varint(&mut writer, entity_id as u32)?;
            writer.write_i16::<BE>(d_x)?;
            writer.write_i16::<BE>(d_y)?;
            writer.write_i16::<BE>(d_z)?;
            writer.write_i8(yaw)?;
            writer.write_i8(pitch)?;
            writer.write_all(&[on_ground as u8])?;
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
    pub(super) fn read_entity_look_response(mut reader: &mut Reader) -> Result<EntityLookResponse> {
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
    impl EntityLookResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            entity_id: i32,
            yaw: i8,
            pitch: i8,
            on_ground: bool,
        ) -> IoResult<()> {
            write_varint(&mut writer, entity_id as u32)?;
            writer.write_i8(yaw)?;
            writer.write_i8(pitch)?;
            writer.write_all(&[on_ground as u8])?;
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
    pub(super) fn read_vehicle_move_response(
        mut reader: &mut Reader,
    ) -> Result<VehicleMoveResponse> {
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
    impl VehicleMoveResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            x: f64,
            y: f64,
            z: f64,
            yaw: f32,
            pitch: f32,
        ) -> IoResult<()> {
            writer.write_f64::<BE>(x)?;
            writer.write_f64::<BE>(y)?;
            writer.write_f64::<BE>(z)?;
            writer.write_f32::<BE>(yaw)?;
            writer.write_f32::<BE>(pitch)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct OpenBookResponse {
        pub hand: i32,
    }
    pub(super) fn read_open_book_response(mut reader: &mut Reader) -> Result<OpenBookResponse> {
        let hand: i32 = read_varint(&mut reader)?;

        let result = OpenBookResponse { hand };
        Ok(result)
    }
    impl OpenBookResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, hand: i32) -> IoResult<()> {
            write_varint(&mut writer, hand as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct OpenSignEntityResponse {
        pub location: Position,
    }
    pub(super) fn read_open_sign_entity_response(
        mut reader: &mut Reader,
    ) -> Result<OpenSignEntityResponse> {
        let location: Position = MD::deserialize(reader)?;

        let result = OpenSignEntityResponse { location };
        Ok(result)
    }
    impl OpenSignEntityResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, location: Position) -> IoResult<()> {
            location.write(&mut writer)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct CraftRecipeResponse {
        pub window_id: i8,
        pub recipe: IndexedString,
    }
    pub(super) fn read_craft_recipe_response(
        mut reader: &mut Reader,
    ) -> Result<CraftRecipeResponse> {
        let window_id: i8 = MD::deserialize(reader)?;
        let recipe: IndexedString = MD::deserialize(reader)?;

        let result = CraftRecipeResponse { window_id, recipe };
        Ok(result)
    }
    impl CraftRecipeResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            window_id: i8,
            recipe: &str,
        ) -> IoResult<()> {
            writer.write_i8(window_id)?;
            write_varint(&mut writer, recipe.len() as u32)?;
            writer.write_all(recipe.as_bytes())?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct AbilitiesResponse {
        pub flags: i8,
        pub flying_speed: f32,
        pub walking_speed: f32,
    }
    pub(super) fn read_abilities_response(mut reader: &mut Reader) -> Result<AbilitiesResponse> {
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
    impl AbilitiesResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            flags: i8,
            flying_speed: f32,
            walking_speed: f32,
        ) -> IoResult<()> {
            writer.write_i8(flags)?;
            writer.write_f32::<BE>(flying_speed)?;
            writer.write_f32::<BE>(walking_speed)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EndCombatEventResponse {
        pub duration: i32,
        pub entity_id: i32,
    }
    pub(super) fn read_end_combat_event_response(
        mut reader: &mut Reader,
    ) -> Result<EndCombatEventResponse> {
        let duration: i32 = read_varint(&mut reader)?;
        let entity_id: i32 = MD::deserialize(reader)?;

        let result = EndCombatEventResponse {
            duration,
            entity_id,
        };
        Ok(result)
    }
    impl EndCombatEventResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            duration: i32,
            entity_id: i32,
        ) -> IoResult<()> {
            write_varint(&mut writer, duration as u32)?;
            writer.write_i32::<BE>(entity_id)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EnterCombatEventResponse {}
    pub(super) fn read_enter_combat_event_response(
        mut _reader: &mut Reader,
    ) -> Result<EnterCombatEventResponse> {
        let result = EnterCombatEventResponse {};
        Ok(result)
    }
    impl EnterCombatEventResponse {
        pub(crate) fn write<W: Write>(mut _writer: &mut W) -> IoResult<()> {
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct DeathCombatEventResponse {
        pub player_id: i32,
        pub entity_id: i32,
        pub message: IndexedString,
    }
    pub(super) fn read_death_combat_event_response(
        mut reader: &mut Reader,
    ) -> Result<DeathCombatEventResponse> {
        let player_id: i32 = read_varint(&mut reader)?;
        let entity_id: i32 = MD::deserialize(reader)?;
        let message: IndexedString = MD::deserialize(reader)?;

        let result = DeathCombatEventResponse {
            player_id,
            entity_id,
            message,
        };
        Ok(result)
    }
    impl DeathCombatEventResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            player_id: i32,
            entity_id: i32,
            message: &str,
        ) -> IoResult<()> {
            write_varint(&mut writer, player_id as u32)?;
            writer.write_i32::<BE>(entity_id)?;
            write_varint(&mut writer, message.len() as u32)?;
            writer.write_all(message.as_bytes())?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct PlayerInfoResponse {}
    pub(super) fn read_player_info_response(
        mut _reader: &mut Reader,
    ) -> Result<PlayerInfoResponse> {
        let result = PlayerInfoResponse {};
        Ok(result)
    }
    impl PlayerInfoResponse {
        pub(crate) fn write<W: Write>(mut _writer: &mut W) -> IoResult<()> {
            Ok(())
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
    pub(super) fn read_position_response(mut reader: &mut Reader) -> Result<PositionResponse> {
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
    impl PositionResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            x: f64,
            y: f64,
            z: f64,
            yaw: f32,
            pitch: f32,
            flags: i8,
            teleport_id: i32,
            dismount_vehicle: bool,
        ) -> IoResult<()> {
            writer.write_f64::<BE>(x)?;
            writer.write_f64::<BE>(y)?;
            writer.write_f64::<BE>(z)?;
            writer.write_f32::<BE>(yaw)?;
            writer.write_f32::<BE>(pitch)?;
            writer.write_i8(flags)?;
            write_varint(&mut writer, teleport_id as u32)?;
            writer.write_all(&[dismount_vehicle as u8])?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UnlockRecipesResponse {}
    pub(super) fn read_unlock_recipes_response(
        mut _reader: &mut Reader,
    ) -> Result<UnlockRecipesResponse> {
        let result = UnlockRecipesResponse {};
        Ok(result)
    }
    impl UnlockRecipesResponse {
        pub(crate) fn write<W: Write>(mut _writer: &mut W) -> IoResult<()> {
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EntityDestroyResponse {
        pub entity_ids: Vec<i32>,
    }
    pub(super) fn read_entity_destroy_response(
        mut reader: &mut Reader,
    ) -> Result<EntityDestroyResponse> {
        let count_array: i32 = read_varint(&mut reader)?;
        let mut entity_ids = Vec::with_capacity(count_array as usize);
        for _ in 0..count_array {
            let x: i32 = read_varint(&mut reader)?;
            entity_ids.push(x);
        }

        let result = EntityDestroyResponse { entity_ids };
        Ok(result)
    }
    impl EntityDestroyResponse {
        pub(crate) fn write<W: Write>(mut _writer: &mut W, _entity_ids: &[i32]) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct RemoveEntityEffectResponse {
        pub entity_id: i32,
        pub effect_id: i32,
    }
    pub(super) fn read_remove_entity_effect_response(
        mut reader: &mut Reader,
    ) -> Result<RemoveEntityEffectResponse> {
        let entity_id: i32 = read_varint(&mut reader)?;
        let effect_id: i32 = read_varint(&mut reader)?;

        let result = RemoveEntityEffectResponse {
            entity_id,
            effect_id,
        };
        Ok(result)
    }
    impl RemoveEntityEffectResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            entity_id: i32,
            effect_id: i32,
        ) -> IoResult<()> {
            write_varint(&mut writer, entity_id as u32)?;
            write_varint(&mut writer, effect_id as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ResourcePackSendResponse {
        pub url: IndexedString,
        pub hash: IndexedString,
        pub forced: bool,
        pub prompt_message: Option<IndexedString>,
    }
    pub(super) fn read_resource_pack_send_response(
        mut reader: &mut Reader,
    ) -> Result<ResourcePackSendResponse> {
        let url: IndexedString = MD::deserialize(reader)?;
        let hash: IndexedString = MD::deserialize(reader)?;
        let forced: bool = MD::deserialize(reader)?;
        let prompt_message: Option<IndexedString> = MD::deserialize(reader)?;

        let result = ResourcePackSendResponse {
            url,
            hash,
            forced,
            prompt_message,
        };
        Ok(result)
    }
    impl ResourcePackSendResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            url: &str,
            hash: &str,
            forced: bool,
            prompt_message: Option<&str>,
        ) -> IoResult<()> {
            write_varint(&mut writer, url.len() as u32)?;
            writer.write_all(url.as_bytes())?;
            write_varint(&mut writer, hash.len() as u32)?;
            writer.write_all(hash.as_bytes())?;
            writer.write_all(&[forced as u8])?;
            match prompt_message {
                Some(prompt_message_1) => {
                    writer.write_all(&[1])?;

                    write_varint(&mut writer, prompt_message_1.len() as u32)?;
                    writer.write_all(prompt_message_1.as_bytes())?;
                }
                None => writer.write_all(&[0])?,
            }
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct RespawnResponse {
        pub dimension: IndexedNbt,
        pub world_name: IndexedString,
        pub hashed_seed: i64,
        pub gamemode: u8,
        pub previous_gamemode: u8,
        pub is_debug: bool,
        pub is_flat: bool,
        pub copy_metadata: bool,
    }
    pub(super) fn read_respawn_response(mut reader: &mut Reader) -> Result<RespawnResponse> {
        let dimension: IndexedNbt = MD::deserialize(reader)?;
        let world_name: IndexedString = MD::deserialize(reader)?;
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
    impl RespawnResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            dimension: IndexedNbt,
            world_name: &str,
            hashed_seed: i64,
            gamemode: u8,
            previous_gamemode: u8,
            is_debug: bool,
            is_flat: bool,
            copy_metadata: bool,
        ) -> IoResult<()> {
            unimplemented!();
            write_varint(&mut writer, world_name.len() as u32)?;
            writer.write_all(world_name.as_bytes())?;
            writer.write_i64::<BE>(hashed_seed)?;
            writer.write_u8(gamemode)?;
            writer.write_u8(previous_gamemode)?;
            writer.write_all(&[is_debug as u8])?;
            writer.write_all(&[is_flat as u8])?;
            writer.write_all(&[copy_metadata as u8])?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EntityHeadRotationResponse {
        pub entity_id: i32,
        pub head_yaw: i8,
    }
    pub(super) fn read_entity_head_rotation_response(
        mut reader: &mut Reader,
    ) -> Result<EntityHeadRotationResponse> {
        let entity_id: i32 = read_varint(&mut reader)?;
        let head_yaw: i8 = MD::deserialize(reader)?;

        let result = EntityHeadRotationResponse {
            entity_id,
            head_yaw,
        };
        Ok(result)
    }
    impl EntityHeadRotationResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            entity_id: i32,
            head_yaw: i8,
        ) -> IoResult<()> {
            write_varint(&mut writer, entity_id as u32)?;
            writer.write_i8(head_yaw)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct CameraResponse {
        pub camera_id: i32,
    }
    pub(super) fn read_camera_response(mut reader: &mut Reader) -> Result<CameraResponse> {
        let camera_id: i32 = read_varint(&mut reader)?;

        let result = CameraResponse { camera_id };
        Ok(result)
    }
    impl CameraResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, camera_id: i32) -> IoResult<()> {
            write_varint(&mut writer, camera_id as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct HeldItemSlotResponse {
        pub slot: i8,
    }
    pub(super) fn read_held_item_slot_response(
        mut reader: &mut Reader,
    ) -> Result<HeldItemSlotResponse> {
        let slot: i8 = MD::deserialize(reader)?;

        let result = HeldItemSlotResponse { slot };
        Ok(result)
    }
    impl HeldItemSlotResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, slot: i8) -> IoResult<()> {
            writer.write_i8(slot)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UpdateViewPositionResponse {
        pub chunk_x: i32,
        pub chunk_z: i32,
    }
    pub(super) fn read_update_view_position_response(
        mut reader: &mut Reader,
    ) -> Result<UpdateViewPositionResponse> {
        let chunk_x: i32 = read_varint(&mut reader)?;
        let chunk_z: i32 = read_varint(&mut reader)?;

        let result = UpdateViewPositionResponse { chunk_x, chunk_z };
        Ok(result)
    }
    impl UpdateViewPositionResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            chunk_x: i32,
            chunk_z: i32,
        ) -> IoResult<()> {
            write_varint(&mut writer, chunk_x as u32)?;
            write_varint(&mut writer, chunk_z as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UpdateViewDistanceResponse {
        pub view_distance: i32,
    }
    pub(super) fn read_update_view_distance_response(
        mut reader: &mut Reader,
    ) -> Result<UpdateViewDistanceResponse> {
        let view_distance: i32 = read_varint(&mut reader)?;

        let result = UpdateViewDistanceResponse { view_distance };
        Ok(result)
    }
    impl UpdateViewDistanceResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, view_distance: i32) -> IoResult<()> {
            write_varint(&mut writer, view_distance as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ScoreboardDisplayObjectiveResponse {
        pub position: i8,
        pub name: IndexedString,
    }
    pub(super) fn read_scoreboard_display_objective_response(
        mut reader: &mut Reader,
    ) -> Result<ScoreboardDisplayObjectiveResponse> {
        let position: i8 = MD::deserialize(reader)?;
        let name: IndexedString = MD::deserialize(reader)?;

        let result = ScoreboardDisplayObjectiveResponse { position, name };
        Ok(result)
    }
    impl ScoreboardDisplayObjectiveResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            position: i8,
            name: &str,
        ) -> IoResult<()> {
            writer.write_i8(position)?;
            write_varint(&mut writer, name.len() as u32)?;
            writer.write_all(name.as_bytes())?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EntityMetadataResponse {}
    pub(super) fn read_entity_metadata_response(
        mut _reader: &mut Reader,
    ) -> Result<EntityMetadataResponse> {
        let result = EntityMetadataResponse {};
        Ok(result)
    }
    impl EntityMetadataResponse {
        pub(crate) fn write<W: Write>(mut _writer: &mut W) -> IoResult<()> {
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct AttachEntityResponse {
        pub entity_id: i32,
        pub vehicle_id: i32,
    }
    pub(super) fn read_attach_entity_response(
        mut reader: &mut Reader,
    ) -> Result<AttachEntityResponse> {
        let entity_id: i32 = MD::deserialize(reader)?;
        let vehicle_id: i32 = MD::deserialize(reader)?;

        let result = AttachEntityResponse {
            entity_id,
            vehicle_id,
        };
        Ok(result)
    }
    impl AttachEntityResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            entity_id: i32,
            vehicle_id: i32,
        ) -> IoResult<()> {
            writer.write_i32::<BE>(entity_id)?;
            writer.write_i32::<BE>(vehicle_id)?;
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
    pub(super) fn read_entity_velocity_response(
        mut reader: &mut Reader,
    ) -> Result<EntityVelocityResponse> {
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
    impl EntityVelocityResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            entity_id: i32,
            velocity_x: i16,
            velocity_y: i16,
            velocity_z: i16,
        ) -> IoResult<()> {
            write_varint(&mut writer, entity_id as u32)?;
            writer.write_i16::<BE>(velocity_x)?;
            writer.write_i16::<BE>(velocity_y)?;
            writer.write_i16::<BE>(velocity_z)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EntityEquipmentResponse {}
    pub(super) fn read_entity_equipment_response(
        mut _reader: &mut Reader,
    ) -> Result<EntityEquipmentResponse> {
        let result = EntityEquipmentResponse {};
        Ok(result)
    }
    impl EntityEquipmentResponse {
        pub(crate) fn write<W: Write>(mut _writer: &mut W) -> IoResult<()> {
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ExperienceResponse {
        pub experience_bar: f32,
        pub level: i32,
        pub total_experience: i32,
    }
    pub(super) fn read_experience_response(mut reader: &mut Reader) -> Result<ExperienceResponse> {
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
    impl ExperienceResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            experience_bar: f32,
            level: i32,
            total_experience: i32,
        ) -> IoResult<()> {
            writer.write_f32::<BE>(experience_bar)?;
            write_varint(&mut writer, level as u32)?;
            write_varint(&mut writer, total_experience as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UpdateHealthResponse {
        pub health: f32,
        pub food: i32,
        pub food_saturation: f32,
    }
    pub(super) fn read_update_health_response(
        mut reader: &mut Reader,
    ) -> Result<UpdateHealthResponse> {
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
    impl UpdateHealthResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            health: f32,
            food: i32,
            food_saturation: f32,
        ) -> IoResult<()> {
            writer.write_f32::<BE>(health)?;
            write_varint(&mut writer, food as u32)?;
            writer.write_f32::<BE>(food_saturation)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ScoreboardObjectiveResponse {}
    pub(super) fn read_scoreboard_objective_response(
        mut _reader: &mut Reader,
    ) -> Result<ScoreboardObjectiveResponse> {
        let result = ScoreboardObjectiveResponse {};
        Ok(result)
    }
    impl ScoreboardObjectiveResponse {
        pub(crate) fn write<W: Write>(mut _writer: &mut W) -> IoResult<()> {
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SetPassengersResponse {
        pub entity_id: i32,
        pub passengers: Vec<i32>,
    }
    pub(super) fn read_set_passengers_response(
        mut reader: &mut Reader,
    ) -> Result<SetPassengersResponse> {
        let entity_id: i32 = read_varint(&mut reader)?;
        let count_array: i32 = read_varint(&mut reader)?;
        let mut passengers = Vec::with_capacity(count_array as usize);
        for _ in 0..count_array {
            let x: i32 = read_varint(&mut reader)?;
            passengers.push(x);
        }

        let result = SetPassengersResponse {
            entity_id,
            passengers,
        };
        Ok(result)
    }
    impl SetPassengersResponse {
        pub(crate) fn write<W: Write>(
            mut _writer: &mut W,
            _entity_id: i32,
            _passengers: &[i32],
        ) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct TeamsResponse {}
    pub(super) fn read_teams_response(mut _reader: &mut Reader) -> Result<TeamsResponse> {
        let result = TeamsResponse {};
        Ok(result)
    }
    impl TeamsResponse {
        pub(crate) fn write<W: Write>(mut _writer: &mut W) -> IoResult<()> {
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ScoreboardScoreResponse {}
    pub(super) fn read_scoreboard_score_response(
        mut _reader: &mut Reader,
    ) -> Result<ScoreboardScoreResponse> {
        let result = ScoreboardScoreResponse {};
        Ok(result)
    }
    impl ScoreboardScoreResponse {
        pub(crate) fn write<W: Write>(mut _writer: &mut W) -> IoResult<()> {
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SpawnPositionResponse {
        pub location: Position,
        pub angle: f32,
    }
    pub(super) fn read_spawn_position_response(
        mut reader: &mut Reader,
    ) -> Result<SpawnPositionResponse> {
        let location: Position = MD::deserialize(reader)?;
        let angle: f32 = MD::deserialize(reader)?;

        let result = SpawnPositionResponse { location, angle };
        Ok(result)
    }
    impl SpawnPositionResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            location: Position,
            angle: f32,
        ) -> IoResult<()> {
            location.write(&mut writer)?;
            writer.write_f32::<BE>(angle)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct UpdateTimeResponse {
        pub age: i64,
        pub time: i64,
    }
    pub(super) fn read_update_time_response(mut reader: &mut Reader) -> Result<UpdateTimeResponse> {
        let age: i64 = MD::deserialize(reader)?;
        let time: i64 = MD::deserialize(reader)?;

        let result = UpdateTimeResponse { age, time };
        Ok(result)
    }
    impl UpdateTimeResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, age: i64, time: i64) -> IoResult<()> {
            writer.write_i64::<BE>(age)?;
            writer.write_i64::<BE>(time)?;
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
    pub(super) fn read_entity_sound_effect_response(
        mut reader: &mut Reader,
    ) -> Result<EntitySoundEffectResponse> {
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
    impl EntitySoundEffectResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            sound_id: i32,
            sound_category: i32,
            entity_id: i32,
            volume: f32,
            pitch: f32,
        ) -> IoResult<()> {
            write_varint(&mut writer, sound_id as u32)?;
            write_varint(&mut writer, sound_category as u32)?;
            write_varint(&mut writer, entity_id as u32)?;
            writer.write_f32::<BE>(volume)?;
            writer.write_f32::<BE>(pitch)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct StopSoundResponse {}
    pub(super) fn read_stop_sound_response(mut _reader: &mut Reader) -> Result<StopSoundResponse> {
        let result = StopSoundResponse {};
        Ok(result)
    }
    impl StopSoundResponse {
        pub(crate) fn write<W: Write>(mut _writer: &mut W) -> IoResult<()> {
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
    pub(super) fn read_sound_effect_response(
        mut reader: &mut Reader,
    ) -> Result<SoundEffectResponse> {
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
    impl SoundEffectResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            sound_id: i32,
            sound_category: i32,
            x: i32,
            y: i32,
            z: i32,
            volume: f32,
            pitch: f32,
        ) -> IoResult<()> {
            write_varint(&mut writer, sound_id as u32)?;
            write_varint(&mut writer, sound_category as u32)?;
            writer.write_i32::<BE>(x)?;
            writer.write_i32::<BE>(y)?;
            writer.write_i32::<BE>(z)?;
            writer.write_f32::<BE>(volume)?;
            writer.write_f32::<BE>(pitch)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct PlayerlistHeaderResponse {
        pub header: IndexedString,
        pub footer: IndexedString,
    }
    pub(super) fn read_playerlist_header_response(
        mut reader: &mut Reader,
    ) -> Result<PlayerlistHeaderResponse> {
        let header: IndexedString = MD::deserialize(reader)?;
        let footer: IndexedString = MD::deserialize(reader)?;

        let result = PlayerlistHeaderResponse { header, footer };
        Ok(result)
    }
    impl PlayerlistHeaderResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            header: &str,
            footer: &str,
        ) -> IoResult<()> {
            write_varint(&mut writer, header.len() as u32)?;
            writer.write_all(header.as_bytes())?;
            write_varint(&mut writer, footer.len() as u32)?;
            writer.write_all(footer.as_bytes())?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct CollectResponse {
        pub collected_entity_id: i32,
        pub collector_entity_id: i32,
        pub pickup_item_count: i32,
    }
    pub(super) fn read_collect_response(mut reader: &mut Reader) -> Result<CollectResponse> {
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
    impl CollectResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            collected_entity_id: i32,
            collector_entity_id: i32,
            pickup_item_count: i32,
        ) -> IoResult<()> {
            write_varint(&mut writer, collected_entity_id as u32)?;
            write_varint(&mut writer, collector_entity_id as u32)?;
            write_varint(&mut writer, pickup_item_count as u32)?;
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
    pub(super) fn read_entity_teleport_response(
        mut reader: &mut Reader,
    ) -> Result<EntityTeleportResponse> {
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
    impl EntityTeleportResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            entity_id: i32,
            x: f64,
            y: f64,
            z: f64,
            yaw: i8,
            pitch: i8,
            on_ground: bool,
        ) -> IoResult<()> {
            write_varint(&mut writer, entity_id as u32)?;
            writer.write_f64::<BE>(x)?;
            writer.write_f64::<BE>(y)?;
            writer.write_f64::<BE>(z)?;
            writer.write_i8(yaw)?;
            writer.write_i8(pitch)?;
            writer.write_all(&[on_ground as u8])?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EntityUpdateAttributesResponse_Modifiers {
        pub uuid: u128,
        pub amount: f64,
        pub operation: i8,
    }
    pub(super) fn read_entity_update_attributes_response_modifiers(
        mut reader: &mut Reader,
    ) -> Result<EntityUpdateAttributesResponse_Modifiers> {
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
    impl EntityUpdateAttributesResponse_Modifiers {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            uuid: u128,
            amount: f64,
            operation: i8,
        ) -> IoResult<()> {
            writer.write_u128::<BE>(uuid)?;
            writer.write_f64::<BE>(amount)?;
            writer.write_i8(operation)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct EntityUpdateAttributesResponse_Properties {
        pub key: IndexedString,
        pub value: f64,
        pub modifiers: Vec<EntityUpdateAttributesResponse_Modifiers>,
    }
    pub(super) fn read_entity_update_attributes_response_properties(
        mut reader: &mut Reader,
    ) -> Result<EntityUpdateAttributesResponse_Properties> {
        let key: IndexedString = MD::deserialize(reader)?;
        let value: f64 = MD::deserialize(reader)?;
        let count_array: i32 = read_varint(&mut reader)?;
        let mut modifiers = Vec::with_capacity(count_array as usize);
        for _ in 0..count_array {
            let x: EntityUpdateAttributesResponse_Modifiers =
                read_entity_update_attributes_response_modifiers(reader)?;
            modifiers.push(x);
        }

        let result = EntityUpdateAttributesResponse_Properties {
            key,
            value,
            modifiers,
        };
        Ok(result)
    }
    impl EntityUpdateAttributesResponse_Properties {
        pub(crate) fn write<W: Write>(
            mut _writer: &mut W,
            _key: &str,
            _value: f64,
            _modifiers: &[EntityUpdateAttributesResponse_Modifiers],
        ) -> IoResult<()> {
            unimplemented!();
        }
    }
    #[derive(Debug)]
    pub struct EntityUpdateAttributesResponse {
        pub entity_id: i32,
        pub properties: Vec<EntityUpdateAttributesResponse_Properties>,
    }
    pub(super) fn read_entity_update_attributes_response(
        mut reader: &mut Reader,
    ) -> Result<EntityUpdateAttributesResponse> {
        let entity_id: i32 = read_varint(&mut reader)?;
        let count_array: i32 = read_varint(&mut reader)?;
        let mut properties = Vec::with_capacity(count_array as usize);
        for _ in 0..count_array {
            let x: EntityUpdateAttributesResponse_Properties =
                read_entity_update_attributes_response_properties(reader)?;
            properties.push(x);
        }

        let result = EntityUpdateAttributesResponse {
            entity_id,
            properties,
        };
        Ok(result)
    }
    impl EntityUpdateAttributesResponse {
        pub(crate) fn write<W: Write>(
            mut _writer: &mut W,
            _entity_id: i32,
            _properties: &[EntityUpdateAttributesResponse_Properties],
        ) -> IoResult<()> {
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
    pub(super) fn read_entity_effect_response(
        mut reader: &mut Reader,
    ) -> Result<EntityEffectResponse> {
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
    impl EntityEffectResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            entity_id: i32,
            effect_id: i32,
            amplifier: i8,
            duration: i32,
            hide_particles: i8,
        ) -> IoResult<()> {
            write_varint(&mut writer, entity_id as u32)?;
            write_varint(&mut writer, effect_id as u32)?;
            writer.write_i8(amplifier)?;
            write_varint(&mut writer, duration as u32)?;
            writer.write_i8(hide_particles)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SelectAdvancementTabResponse {
        pub id: Option<IndexedString>,
    }
    pub(super) fn read_select_advancement_tab_response(
        mut reader: &mut Reader,
    ) -> Result<SelectAdvancementTabResponse> {
        let id: Option<IndexedString> = MD::deserialize(reader)?;

        let result = SelectAdvancementTabResponse { id };
        Ok(result)
    }
    impl SelectAdvancementTabResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, id: Option<&str>) -> IoResult<()> {
            match id {
                Some(id_1) => {
                    writer.write_all(&[1])?;

                    write_varint(&mut writer, id_1.len() as u32)?;
                    writer.write_all(id_1.as_bytes())?;
                }
                None => writer.write_all(&[0])?,
            }
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct DeclareRecipesResponse {}
    pub(super) fn read_declare_recipes_response(
        mut _reader: &mut Reader,
    ) -> Result<DeclareRecipesResponse> {
        let result = DeclareRecipesResponse {};
        Ok(result)
    }
    impl DeclareRecipesResponse {
        pub(crate) fn write<W: Write>(mut _writer: &mut W) -> IoResult<()> {
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct TagsResponse {}
    pub(super) fn read_tags_response(mut _reader: &mut Reader) -> Result<TagsResponse> {
        let result = TagsResponse {};
        Ok(result)
    }
    impl TagsResponse {
        pub(crate) fn write<W: Write>(mut _writer: &mut W) -> IoResult<()> {
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct AcknowledgePlayerDiggingResponse {
        pub location: Position,
        pub block: i32,
        pub status: i32,
        pub successful: bool,
    }
    pub(super) fn read_acknowledge_player_digging_response(
        mut reader: &mut Reader,
    ) -> Result<AcknowledgePlayerDiggingResponse> {
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
    impl AcknowledgePlayerDiggingResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            location: Position,
            block: i32,
            status: i32,
            successful: bool,
        ) -> IoResult<()> {
            location.write(&mut writer)?;
            write_varint(&mut writer, block as u32)?;
            write_varint(&mut writer, status as u32)?;
            writer.write_all(&[successful as u8])?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SculkVibrationSignalResponse {}
    pub(super) fn read_sculk_vibration_signal_response(
        mut _reader: &mut Reader,
    ) -> Result<SculkVibrationSignalResponse> {
        let result = SculkVibrationSignalResponse {};
        Ok(result)
    }
    impl SculkVibrationSignalResponse {
        pub(crate) fn write<W: Write>(mut _writer: &mut W) -> IoResult<()> {
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ClearTitlesResponse {
        pub reset: bool,
    }
    pub(super) fn read_clear_titles_response(
        mut reader: &mut Reader,
    ) -> Result<ClearTitlesResponse> {
        let reset: bool = MD::deserialize(reader)?;

        let result = ClearTitlesResponse { reset };
        Ok(result)
    }
    impl ClearTitlesResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, reset: bool) -> IoResult<()> {
            writer.write_all(&[reset as u8])?;
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
    pub(super) fn read_initialize_world_border_response(
        mut reader: &mut Reader,
    ) -> Result<InitializeWorldBorderResponse> {
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
    impl InitializeWorldBorderResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            x: f64,
            z: f64,
            old_diameter: f64,
            new_diameter: f64,
            speed: i64,
            portal_teleport_boundary: i32,
            warning_blocks: i32,
            warning_time: i32,
        ) -> IoResult<()> {
            writer.write_f64::<BE>(x)?;
            writer.write_f64::<BE>(z)?;
            writer.write_f64::<BE>(old_diameter)?;
            writer.write_f64::<BE>(new_diameter)?;
            write_varlong(&mut writer, speed as u64)?;
            write_varint(&mut writer, portal_teleport_boundary as u32)?;
            write_varint(&mut writer, warning_blocks as u32)?;
            write_varint(&mut writer, warning_time as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct ActionBarResponse {
        pub text: IndexedString,
    }
    pub(super) fn read_action_bar_response(mut reader: &mut Reader) -> Result<ActionBarResponse> {
        let text: IndexedString = MD::deserialize(reader)?;

        let result = ActionBarResponse { text };
        Ok(result)
    }
    impl ActionBarResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, text: &str) -> IoResult<()> {
            write_varint(&mut writer, text.len() as u32)?;
            writer.write_all(text.as_bytes())?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct WorldBorderCenterResponse {
        pub x: f64,
        pub z: f64,
    }
    pub(super) fn read_world_border_center_response(
        mut reader: &mut Reader,
    ) -> Result<WorldBorderCenterResponse> {
        let x: f64 = MD::deserialize(reader)?;
        let z: f64 = MD::deserialize(reader)?;

        let result = WorldBorderCenterResponse { x, z };
        Ok(result)
    }
    impl WorldBorderCenterResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, x: f64, z: f64) -> IoResult<()> {
            writer.write_f64::<BE>(x)?;
            writer.write_f64::<BE>(z)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct WorldBorderLerpSizeResponse {
        pub old_diameter: f64,
        pub new_diameter: f64,
        pub speed: i64,
    }
    pub(super) fn read_world_border_lerp_size_response(
        mut reader: &mut Reader,
    ) -> Result<WorldBorderLerpSizeResponse> {
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
    impl WorldBorderLerpSizeResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            old_diameter: f64,
            new_diameter: f64,
            speed: i64,
        ) -> IoResult<()> {
            writer.write_f64::<BE>(old_diameter)?;
            writer.write_f64::<BE>(new_diameter)?;
            write_varlong(&mut writer, speed as u64)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct WorldBorderSizeResponse {
        pub diameter: f64,
    }
    pub(super) fn read_world_border_size_response(
        mut reader: &mut Reader,
    ) -> Result<WorldBorderSizeResponse> {
        let diameter: f64 = MD::deserialize(reader)?;

        let result = WorldBorderSizeResponse { diameter };
        Ok(result)
    }
    impl WorldBorderSizeResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, diameter: f64) -> IoResult<()> {
            writer.write_f64::<BE>(diameter)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct WorldBorderWarningDelayResponse {
        pub warning_time: i32,
    }
    pub(super) fn read_world_border_warning_delay_response(
        mut reader: &mut Reader,
    ) -> Result<WorldBorderWarningDelayResponse> {
        let warning_time: i32 = read_varint(&mut reader)?;

        let result = WorldBorderWarningDelayResponse { warning_time };
        Ok(result)
    }
    impl WorldBorderWarningDelayResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, warning_time: i32) -> IoResult<()> {
            write_varint(&mut writer, warning_time as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct WorldBorderWarningReachResponse {
        pub warning_blocks: i32,
    }
    pub(super) fn read_world_border_warning_reach_response(
        mut reader: &mut Reader,
    ) -> Result<WorldBorderWarningReachResponse> {
        let warning_blocks: i32 = read_varint(&mut reader)?;

        let result = WorldBorderWarningReachResponse { warning_blocks };
        Ok(result)
    }
    impl WorldBorderWarningReachResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, warning_blocks: i32) -> IoResult<()> {
            write_varint(&mut writer, warning_blocks as u32)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct PlayPingResponse {
        pub id: i32,
    }
    pub(super) fn read_play_ping_response(mut reader: &mut Reader) -> Result<PlayPingResponse> {
        let id: i32 = MD::deserialize(reader)?;

        let result = PlayPingResponse { id };
        Ok(result)
    }
    impl PlayPingResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, id: i32) -> IoResult<()> {
            writer.write_i32::<BE>(id)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SetTitleSubtitleResponse {
        pub text: IndexedString,
    }
    pub(super) fn read_set_title_subtitle_response(
        mut reader: &mut Reader,
    ) -> Result<SetTitleSubtitleResponse> {
        let text: IndexedString = MD::deserialize(reader)?;

        let result = SetTitleSubtitleResponse { text };
        Ok(result)
    }
    impl SetTitleSubtitleResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, text: &str) -> IoResult<()> {
            write_varint(&mut writer, text.len() as u32)?;
            writer.write_all(text.as_bytes())?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SetTitleTextResponse {
        pub text: IndexedString,
    }
    pub(super) fn read_set_title_text_response(
        mut reader: &mut Reader,
    ) -> Result<SetTitleTextResponse> {
        let text: IndexedString = MD::deserialize(reader)?;

        let result = SetTitleTextResponse { text };
        Ok(result)
    }
    impl SetTitleTextResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, text: &str) -> IoResult<()> {
            write_varint(&mut writer, text.len() as u32)?;
            writer.write_all(text.as_bytes())?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SetTitleTimeResponse {
        pub fade_in: i32,
        pub stay: i32,
        pub fade_out: i32,
    }
    pub(super) fn read_set_title_time_response(
        mut reader: &mut Reader,
    ) -> Result<SetTitleTimeResponse> {
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
    impl SetTitleTimeResponse {
        pub(crate) fn write<W: Write>(
            mut writer: &mut W,
            fade_in: i32,
            stay: i32,
            fade_out: i32,
        ) -> IoResult<()> {
            writer.write_i32::<BE>(fade_in)?;
            writer.write_i32::<BE>(stay)?;
            writer.write_i32::<BE>(fade_out)?;
            Ok(())
        }
    }
    #[derive(Debug)]
    pub struct SimulationDistanceResponse {
        pub distance: i32,
    }
    pub(super) fn read_simulation_distance_response(
        mut reader: &mut Reader,
    ) -> Result<SimulationDistanceResponse> {
        let distance: i32 = read_varint(&mut reader)?;

        let result = SimulationDistanceResponse { distance };
        Ok(result)
    }
    impl SimulationDistanceResponse {
        pub(crate) fn write<W: Write>(mut writer: &mut W, distance: i32) -> IoResult<()> {
            write_varint(&mut writer, distance as u32)?;
            Ok(())
        }
    }
}
#[derive(Debug)]
pub enum Packet {
    SetProtocolRequest(handshaking::SetProtocolRequest),
    LegacyServerListPingRequest(handshaking::LegacyServerListPingRequest),
    PingStartRequest(status::PingStartRequest),
    PingRequest(status::PingRequest),
    ServerInfoResponse(status::ServerInfoResponse),
    PingResponse(status::PingResponse),
    LoginStartRequest(login::LoginStartRequest),
    EncryptionBeginRequest(login::EncryptionBeginRequest),
    LoginPluginResponse(login::LoginPluginResponse),
    DisconnectResponse(login::DisconnectResponse),
    EncryptionBeginResponse(login::EncryptionBeginResponse),
    SuccessResponse(login::SuccessResponse),
    CompressResponse(login::CompressResponse),
    LoginPluginRequest(login::LoginPluginRequest),
    TeleportConfirmRequest(play::TeleportConfirmRequest),
    QueryBlockNbtRequest(play::QueryBlockNbtRequest),
    SetDifficultyRequest(play::SetDifficultyRequest),
    EditBookRequest(play::EditBookRequest),
    QueryEntityNbtRequest(play::QueryEntityNbtRequest),
    PickItemRequest(play::PickItemRequest),
    NameItemRequest(play::NameItemRequest),
    SelectTradeRequest(play::SelectTradeRequest),
    SetBeaconEffectRequest(play::SetBeaconEffectRequest),
    UpdateCommandBlockRequest(play::UpdateCommandBlockRequest),
    UpdateCommandBlockMinecartRequest(play::UpdateCommandBlockMinecartRequest),
    UpdateStructureBlockRequest(play::UpdateStructureBlockRequest),
    TabCompleteRequest(play::TabCompleteRequest),
    ChatRequest(play::ChatRequest),
    ClientCommandRequest(play::ClientCommandRequest),
    SettingsRequest(play::SettingsRequest),
    EnchantItemRequest(play::EnchantItemRequest),
    WindowClickRequest(play::WindowClickRequest),
    CloseWindowRequest(play::CloseWindowRequest),
    CustomPayloadRequest(play::CustomPayloadRequest),
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
    CraftRecipeRequest(play::CraftRecipeRequest),
    AbilitiesRequest(play::AbilitiesRequest),
    BlockDigRequest(play::BlockDigRequest),
    EntityActionRequest(play::EntityActionRequest),
    SteerVehicleRequest(play::SteerVehicleRequest),
    DisplayedRecipeRequest(play::DisplayedRecipeRequest),
    RecipeBookRequest(play::RecipeBookRequest),
    ResourcePackReceiveRequest(play::ResourcePackReceiveRequest),
    HeldItemSlotRequest(play::HeldItemSlotRequest),
    SetCreativeSlotRequest(play::SetCreativeSlotRequest),
    UpdateJigsawBlockRequest(play::UpdateJigsawBlockRequest),
    UpdateSignRequest(play::UpdateSignRequest),
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
    TileEntityDataResponse(play::TileEntityDataResponse),
    BlockActionResponse(play::BlockActionResponse),
    BlockChangeResponse(play::BlockChangeResponse),
    BossBarResponse(play::BossBarResponse),
    DifficultyResponse(play::DifficultyResponse),
    TabCompleteResponse(play::TabCompleteResponse),
    DeclareCommandsResponse(play::DeclareCommandsResponse),
    FacePlayerResponse(play::FacePlayerResponse),
    NbtQueryResponse(play::NbtQueryResponse),
    ChatResponse(play::ChatResponse),
    MultiBlockChangeResponse(play::MultiBlockChangeResponse),
    CloseWindowResponse(play::CloseWindowResponse),
    OpenWindowResponse(play::OpenWindowResponse),
    WindowItemsResponse(play::WindowItemsResponse),
    CraftProgressBarResponse(play::CraftProgressBarResponse),
    SetSlotResponse(play::SetSlotResponse),
    SetCooldownResponse(play::SetCooldownResponse),
    CustomPayloadResponse(play::CustomPayloadResponse),
    NamedSoundEffectResponse(play::NamedSoundEffectResponse),
    KickDisconnectResponse(play::KickDisconnectResponse),
    EntityStatusResponse(play::EntityStatusResponse),
    ExplosionResponse(play::ExplosionResponse),
    UnloadChunkResponse(play::UnloadChunkResponse),
    GameStateChangeResponse(play::GameStateChangeResponse),
    OpenHorseWindowResponse(play::OpenHorseWindowResponse),
    KeepAliveResponse(play::KeepAliveResponse),
    MapChunkResponse(play::MapChunkResponse),
    WorldEventResponse(play::WorldEventResponse),
    WorldParticlesResponse(play::WorldParticlesResponse),
    UpdateLightResponse(play::UpdateLightResponse),
    LoginResponse(play::LoginResponse),
    MapResponse(play::MapResponse),
    TradeListResponse(play::TradeListResponse),
    RelEntityMoveResponse(play::RelEntityMoveResponse),
    EntityMoveLookResponse(play::EntityMoveLookResponse),
    EntityLookResponse(play::EntityLookResponse),
    VehicleMoveResponse(play::VehicleMoveResponse),
    OpenBookResponse(play::OpenBookResponse),
    OpenSignEntityResponse(play::OpenSignEntityResponse),
    CraftRecipeResponse(play::CraftRecipeResponse),
    AbilitiesResponse(play::AbilitiesResponse),
    EndCombatEventResponse(play::EndCombatEventResponse),
    EnterCombatEventResponse(play::EnterCombatEventResponse),
    DeathCombatEventResponse(play::DeathCombatEventResponse),
    PlayerInfoResponse(play::PlayerInfoResponse),
    PositionResponse(play::PositionResponse),
    UnlockRecipesResponse(play::UnlockRecipesResponse),
    EntityDestroyResponse(play::EntityDestroyResponse),
    RemoveEntityEffectResponse(play::RemoveEntityEffectResponse),
    ResourcePackSendResponse(play::ResourcePackSendResponse),
    RespawnResponse(play::RespawnResponse),
    EntityHeadRotationResponse(play::EntityHeadRotationResponse),
    CameraResponse(play::CameraResponse),
    HeldItemSlotResponse(play::HeldItemSlotResponse),
    UpdateViewPositionResponse(play::UpdateViewPositionResponse),
    UpdateViewDistanceResponse(play::UpdateViewDistanceResponse),
    ScoreboardDisplayObjectiveResponse(play::ScoreboardDisplayObjectiveResponse),
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
    PlayerlistHeaderResponse(play::PlayerlistHeaderResponse),
    CollectResponse(play::CollectResponse),
    EntityTeleportResponse(play::EntityTeleportResponse),
    EntityUpdateAttributesResponse(play::EntityUpdateAttributesResponse),
    EntityEffectResponse(play::EntityEffectResponse),
    SelectAdvancementTabResponse(play::SelectAdvancementTabResponse),
    DeclareRecipesResponse(play::DeclareRecipesResponse),
    TagsResponse(play::TagsResponse),
    AcknowledgePlayerDiggingResponse(play::AcknowledgePlayerDiggingResponse),
    SculkVibrationSignalResponse(play::SculkVibrationSignalResponse),
    ClearTitlesResponse(play::ClearTitlesResponse),
    InitializeWorldBorderResponse(play::InitializeWorldBorderResponse),
    ActionBarResponse(play::ActionBarResponse),
    WorldBorderCenterResponse(play::WorldBorderCenterResponse),
    WorldBorderLerpSizeResponse(play::WorldBorderLerpSizeResponse),
    WorldBorderSizeResponse(play::WorldBorderSizeResponse),
    WorldBorderWarningDelayResponse(play::WorldBorderWarningDelayResponse),
    WorldBorderWarningReachResponse(play::WorldBorderWarningReachResponse),
    PlayPingResponse(play::PlayPingResponse),
    SetTitleSubtitleResponse(play::SetTitleSubtitleResponse),
    SetTitleTextResponse(play::SetTitleTextResponse),
    SetTitleTimeResponse(play::SetTitleTimeResponse),
    SimulationDistanceResponse(play::SimulationDistanceResponse),
}

pub fn de_packets<'r>(
    state: ConnectionState,
    direction: PacketDirection,
    id: u32,
    reader: &'r mut Reader<'r>,
) -> Result<Packet> {
    use ConnectionState as S;
    use PacketDirection as D;

    let packet = match (state, direction, id) {
        (S::Handshaking, D::ClientToServer, 0x0) => {
            let p = handshaking::read_set_protocol_request(reader)?;
            Packet::SetProtocolRequest(p)
        }
        (S::Handshaking, D::ClientToServer, 0xfe) => {
            let p = handshaking::read_legacy_server_list_ping_request(reader)?;
            Packet::LegacyServerListPingRequest(p)
        }
        (S::Status, D::ClientToServer, 0x0) => {
            let p = status::read_ping_start_request(reader)?;
            Packet::PingStartRequest(p)
        }
        (S::Status, D::ClientToServer, 0x1) => {
            let p = status::read_ping_request(reader)?;
            Packet::PingRequest(p)
        }
        (S::Status, D::ServerToClient, 0x0) => {
            let p = status::read_server_info_response(reader)?;
            Packet::ServerInfoResponse(p)
        }
        (S::Status, D::ServerToClient, 0x1) => {
            let p = status::read_ping_response(reader)?;
            Packet::PingResponse(p)
        }
        (S::Login, D::ClientToServer, 0x0) => {
            let p = login::read_login_start_request(reader)?;
            Packet::LoginStartRequest(p)
        }
        (S::Login, D::ClientToServer, 0x1) => {
            let p = login::read_encryption_begin_request(reader)?;
            Packet::EncryptionBeginRequest(p)
        }
        (S::Login, D::ClientToServer, 0x2) => {
            let p = login::read_login_plugin_response(reader)?;
            Packet::LoginPluginResponse(p)
        }
        (S::Login, D::ServerToClient, 0x0) => {
            let p = login::read_disconnect_response(reader)?;
            Packet::DisconnectResponse(p)
        }
        (S::Login, D::ServerToClient, 0x1) => {
            let p = login::read_encryption_begin_response(reader)?;
            Packet::EncryptionBeginResponse(p)
        }
        (S::Login, D::ServerToClient, 0x2) => {
            let p = login::read_success_response(reader)?;
            Packet::SuccessResponse(p)
        }
        (S::Login, D::ServerToClient, 0x3) => {
            let p = login::read_compress_response(reader)?;
            Packet::CompressResponse(p)
        }
        (S::Login, D::ServerToClient, 0x4) => {
            let p = login::read_login_plugin_request(reader)?;
            Packet::LoginPluginRequest(p)
        }
        (S::Play, D::ClientToServer, 0x0) => {
            let p = play::read_teleport_confirm_request(reader)?;
            Packet::TeleportConfirmRequest(p)
        }
        (S::Play, D::ClientToServer, 0x1) => {
            let p = play::read_query_block_nbt_request(reader)?;
            Packet::QueryBlockNbtRequest(p)
        }
        (S::Play, D::ClientToServer, 0x2) => {
            let p = play::read_set_difficulty_request(reader)?;
            Packet::SetDifficultyRequest(p)
        }
        (S::Play, D::ClientToServer, 0x3) => {
            let p = play::read_chat_request(reader)?;
            Packet::ChatRequest(p)
        }
        (S::Play, D::ClientToServer, 0x4) => {
            let p = play::read_client_command_request(reader)?;
            Packet::ClientCommandRequest(p)
        }
        (S::Play, D::ClientToServer, 0x5) => {
            let p = play::read_settings_request(reader)?;
            Packet::SettingsRequest(p)
        }
        (S::Play, D::ClientToServer, 0x6) => {
            let p = play::read_tab_complete_request(reader)?;
            Packet::TabCompleteRequest(p)
        }
        (S::Play, D::ClientToServer, 0x7) => {
            let p = play::read_enchant_item_request(reader)?;
            Packet::EnchantItemRequest(p)
        }
        (S::Play, D::ClientToServer, 0x8) => {
            let p = play::read_window_click_request(reader)?;
            Packet::WindowClickRequest(p)
        }
        (S::Play, D::ClientToServer, 0x9) => {
            let p = play::read_close_window_request(reader)?;
            Packet::CloseWindowRequest(p)
        }
        (S::Play, D::ClientToServer, 0xa) => {
            let p = play::read_custom_payload_request(reader)?;
            Packet::CustomPayloadRequest(p)
        }
        (S::Play, D::ClientToServer, 0xb) => {
            let p = play::read_edit_book_request(reader)?;
            Packet::EditBookRequest(p)
        }
        (S::Play, D::ClientToServer, 0xc) => {
            let p = play::read_query_entity_nbt_request(reader)?;
            Packet::QueryEntityNbtRequest(p)
        }
        (S::Play, D::ClientToServer, 0xd) => {
            let p = play::read_use_entity_request(reader)?;
            Packet::UseEntityRequest(p)
        }
        (S::Play, D::ClientToServer, 0xe) => {
            let p = play::read_generate_structure_request(reader)?;
            Packet::GenerateStructureRequest(p)
        }
        (S::Play, D::ClientToServer, 0xf) => {
            let p = play::read_keep_alive_request(reader)?;
            Packet::KeepAliveRequest(p)
        }
        (S::Play, D::ClientToServer, 0x10) => {
            let p = play::read_lock_difficulty_request(reader)?;
            Packet::LockDifficultyRequest(p)
        }
        (S::Play, D::ClientToServer, 0x11) => {
            let p = play::read_position_request(reader)?;
            Packet::PositionRequest(p)
        }
        (S::Play, D::ClientToServer, 0x12) => {
            let p = play::read_position_look_request(reader)?;
            Packet::PositionLookRequest(p)
        }
        (S::Play, D::ClientToServer, 0x13) => {
            let p = play::read_look_request(reader)?;
            Packet::LookRequest(p)
        }
        (S::Play, D::ClientToServer, 0x14) => {
            let p = play::read_flying_request(reader)?;
            Packet::FlyingRequest(p)
        }
        (S::Play, D::ClientToServer, 0x15) => {
            let p = play::read_vehicle_move_request(reader)?;
            Packet::VehicleMoveRequest(p)
        }
        (S::Play, D::ClientToServer, 0x16) => {
            let p = play::read_steer_boat_request(reader)?;
            Packet::SteerBoatRequest(p)
        }
        (S::Play, D::ClientToServer, 0x17) => {
            let p = play::read_pick_item_request(reader)?;
            Packet::PickItemRequest(p)
        }
        (S::Play, D::ClientToServer, 0x18) => {
            let p = play::read_craft_recipe_request(reader)?;
            Packet::CraftRecipeRequest(p)
        }
        (S::Play, D::ClientToServer, 0x19) => {
            let p = play::read_abilities_request(reader)?;
            Packet::AbilitiesRequest(p)
        }
        (S::Play, D::ClientToServer, 0x1a) => {
            let p = play::read_block_dig_request(reader)?;
            Packet::BlockDigRequest(p)
        }
        (S::Play, D::ClientToServer, 0x1b) => {
            let p = play::read_entity_action_request(reader)?;
            Packet::EntityActionRequest(p)
        }
        (S::Play, D::ClientToServer, 0x1c) => {
            let p = play::read_steer_vehicle_request(reader)?;
            Packet::SteerVehicleRequest(p)
        }
        (S::Play, D::ClientToServer, 0x1d) => {
            let p = play::read_pong_request(reader)?;
            Packet::PongRequest(p)
        }
        (S::Play, D::ClientToServer, 0x1e) => {
            let p = play::read_recipe_book_request(reader)?;
            Packet::RecipeBookRequest(p)
        }
        (S::Play, D::ClientToServer, 0x1f) => {
            let p = play::read_displayed_recipe_request(reader)?;
            Packet::DisplayedRecipeRequest(p)
        }
        (S::Play, D::ClientToServer, 0x20) => {
            let p = play::read_name_item_request(reader)?;
            Packet::NameItemRequest(p)
        }
        (S::Play, D::ClientToServer, 0x21) => {
            let p = play::read_resource_pack_receive_request(reader)?;
            Packet::ResourcePackReceiveRequest(p)
        }
        (S::Play, D::ClientToServer, 0x22) => {
            let p = play::read_advancement_tab_request(reader)?;
            Packet::AdvancementTabRequest(p)
        }
        (S::Play, D::ClientToServer, 0x23) => {
            let p = play::read_select_trade_request(reader)?;
            Packet::SelectTradeRequest(p)
        }
        (S::Play, D::ClientToServer, 0x24) => {
            let p = play::read_set_beacon_effect_request(reader)?;
            Packet::SetBeaconEffectRequest(p)
        }
        (S::Play, D::ClientToServer, 0x25) => {
            let p = play::read_held_item_slot_request(reader)?;
            Packet::HeldItemSlotRequest(p)
        }
        (S::Play, D::ClientToServer, 0x26) => {
            let p = play::read_update_command_block_request(reader)?;
            Packet::UpdateCommandBlockRequest(p)
        }
        (S::Play, D::ClientToServer, 0x27) => {
            let p = play::read_update_command_block_minecart_request(reader)?;
            Packet::UpdateCommandBlockMinecartRequest(p)
        }
        (S::Play, D::ClientToServer, 0x28) => {
            let p = play::read_set_creative_slot_request(reader)?;
            Packet::SetCreativeSlotRequest(p)
        }
        (S::Play, D::ClientToServer, 0x29) => {
            let p = play::read_update_jigsaw_block_request(reader)?;
            Packet::UpdateJigsawBlockRequest(p)
        }
        (S::Play, D::ClientToServer, 0x2a) => {
            let p = play::read_update_structure_block_request(reader)?;
            Packet::UpdateStructureBlockRequest(p)
        }
        (S::Play, D::ClientToServer, 0x2b) => {
            let p = play::read_update_sign_request(reader)?;
            Packet::UpdateSignRequest(p)
        }
        (S::Play, D::ClientToServer, 0x2c) => {
            let p = play::read_arm_animation_request(reader)?;
            Packet::ArmAnimationRequest(p)
        }
        (S::Play, D::ClientToServer, 0x2d) => {
            let p = play::read_spectate_request(reader)?;
            Packet::SpectateRequest(p)
        }
        (S::Play, D::ClientToServer, 0x2e) => {
            let p = play::read_block_place_request(reader)?;
            Packet::BlockPlaceRequest(p)
        }
        (S::Play, D::ClientToServer, 0x2f) => {
            let p = play::read_use_item_request(reader)?;
            Packet::UseItemRequest(p)
        }
        (S::Play, D::ServerToClient, 0x0) => {
            let p = play::read_spawn_entity_response(reader)?;
            Packet::SpawnEntityResponse(p)
        }
        (S::Play, D::ServerToClient, 0x1) => {
            let p = play::read_spawn_entity_experience_orb_response(reader)?;
            Packet::SpawnEntityExperienceOrbResponse(p)
        }
        (S::Play, D::ServerToClient, 0x2) => {
            let p = play::read_spawn_entity_living_response(reader)?;
            Packet::SpawnEntityLivingResponse(p)
        }
        (S::Play, D::ServerToClient, 0x3) => {
            let p = play::read_spawn_entity_painting_response(reader)?;
            Packet::SpawnEntityPaintingResponse(p)
        }
        (S::Play, D::ServerToClient, 0x4) => {
            let p = play::read_named_entity_spawn_response(reader)?;
            Packet::NamedEntitySpawnResponse(p)
        }
        (S::Play, D::ServerToClient, 0x5) => {
            let p = play::read_sculk_vibration_signal_response(reader)?;
            Packet::SculkVibrationSignalResponse(p)
        }
        (S::Play, D::ServerToClient, 0x6) => {
            let p = play::read_animation_response(reader)?;
            Packet::AnimationResponse(p)
        }
        (S::Play, D::ServerToClient, 0x7) => {
            let p = play::read_statistics_response(reader)?;
            Packet::StatisticsResponse(p)
        }
        (S::Play, D::ServerToClient, 0x8) => {
            let p = play::read_acknowledge_player_digging_response(reader)?;
            Packet::AcknowledgePlayerDiggingResponse(p)
        }
        (S::Play, D::ServerToClient, 0x9) => {
            let p = play::read_block_break_animation_response(reader)?;
            Packet::BlockBreakAnimationResponse(p)
        }
        (S::Play, D::ServerToClient, 0xa) => {
            let p = play::read_tile_entity_data_response(reader)?;
            Packet::TileEntityDataResponse(p)
        }
        (S::Play, D::ServerToClient, 0xb) => {
            let p = play::read_block_action_response(reader)?;
            Packet::BlockActionResponse(p)
        }
        (S::Play, D::ServerToClient, 0xc) => {
            let p = play::read_block_change_response(reader)?;
            Packet::BlockChangeResponse(p)
        }
        (S::Play, D::ServerToClient, 0xd) => {
            let p = play::read_boss_bar_response(reader)?;
            Packet::BossBarResponse(p)
        }
        (S::Play, D::ServerToClient, 0xe) => {
            let p = play::read_difficulty_response(reader)?;
            Packet::DifficultyResponse(p)
        }
        (S::Play, D::ServerToClient, 0xf) => {
            let p = play::read_chat_response(reader)?;
            Packet::ChatResponse(p)
        }
        (S::Play, D::ServerToClient, 0x10) => {
            let p = play::read_clear_titles_response(reader)?;
            Packet::ClearTitlesResponse(p)
        }
        (S::Play, D::ServerToClient, 0x11) => {
            let p = play::read_tab_complete_response(reader)?;
            Packet::TabCompleteResponse(p)
        }
        (S::Play, D::ServerToClient, 0x12) => {
            let p = play::read_declare_commands_response(reader)?;
            Packet::DeclareCommandsResponse(p)
        }
        (S::Play, D::ServerToClient, 0x13) => {
            let p = play::read_close_window_response(reader)?;
            Packet::CloseWindowResponse(p)
        }
        (S::Play, D::ServerToClient, 0x14) => {
            let p = play::read_window_items_response(reader)?;
            Packet::WindowItemsResponse(p)
        }
        (S::Play, D::ServerToClient, 0x15) => {
            let p = play::read_craft_progress_bar_response(reader)?;
            Packet::CraftProgressBarResponse(p)
        }
        (S::Play, D::ServerToClient, 0x16) => {
            let p = play::read_set_slot_response(reader)?;
            Packet::SetSlotResponse(p)
        }
        (S::Play, D::ServerToClient, 0x17) => {
            let p = play::read_set_cooldown_response(reader)?;
            Packet::SetCooldownResponse(p)
        }
        (S::Play, D::ServerToClient, 0x18) => {
            let p = play::read_custom_payload_response(reader)?;
            Packet::CustomPayloadResponse(p)
        }
        (S::Play, D::ServerToClient, 0x19) => {
            let p = play::read_named_sound_effect_response(reader)?;
            Packet::NamedSoundEffectResponse(p)
        }
        (S::Play, D::ServerToClient, 0x1a) => {
            let p = play::read_kick_disconnect_response(reader)?;
            Packet::KickDisconnectResponse(p)
        }
        (S::Play, D::ServerToClient, 0x1b) => {
            let p = play::read_entity_status_response(reader)?;
            Packet::EntityStatusResponse(p)
        }
        (S::Play, D::ServerToClient, 0x1c) => {
            let p = play::read_explosion_response(reader)?;
            Packet::ExplosionResponse(p)
        }
        (S::Play, D::ServerToClient, 0x1d) => {
            let p = play::read_unload_chunk_response(reader)?;
            Packet::UnloadChunkResponse(p)
        }
        (S::Play, D::ServerToClient, 0x1e) => {
            let p = play::read_game_state_change_response(reader)?;
            Packet::GameStateChangeResponse(p)
        }
        (S::Play, D::ServerToClient, 0x1f) => {
            let p = play::read_open_horse_window_response(reader)?;
            Packet::OpenHorseWindowResponse(p)
        }
        (S::Play, D::ServerToClient, 0x20) => {
            let p = play::read_initialize_world_border_response(reader)?;
            Packet::InitializeWorldBorderResponse(p)
        }
        (S::Play, D::ServerToClient, 0x21) => {
            let p = play::read_keep_alive_response(reader)?;
            Packet::KeepAliveResponse(p)
        }
        (S::Play, D::ServerToClient, 0x22) => {
            let p = play::read_map_chunk_response(reader)?;
            Packet::MapChunkResponse(p)
        }
        (S::Play, D::ServerToClient, 0x23) => {
            let p = play::read_world_event_response(reader)?;
            Packet::WorldEventResponse(p)
        }
        (S::Play, D::ServerToClient, 0x24) => {
            let p = play::read_world_particles_response(reader)?;
            Packet::WorldParticlesResponse(p)
        }
        (S::Play, D::ServerToClient, 0x25) => {
            let p = play::read_update_light_response(reader)?;
            Packet::UpdateLightResponse(p)
        }
        (S::Play, D::ServerToClient, 0x26) => {
            let p = play::read_login_response(reader)?;
            Packet::LoginResponse(p)
        }
        (S::Play, D::ServerToClient, 0x27) => {
            let p = play::read_map_response(reader)?;
            Packet::MapResponse(p)
        }
        (S::Play, D::ServerToClient, 0x28) => {
            let p = play::read_trade_list_response(reader)?;
            Packet::TradeListResponse(p)
        }
        (S::Play, D::ServerToClient, 0x29) => {
            let p = play::read_rel_entity_move_response(reader)?;
            Packet::RelEntityMoveResponse(p)
        }
        (S::Play, D::ServerToClient, 0x2a) => {
            let p = play::read_entity_move_look_response(reader)?;
            Packet::EntityMoveLookResponse(p)
        }
        (S::Play, D::ServerToClient, 0x2b) => {
            let p = play::read_entity_look_response(reader)?;
            Packet::EntityLookResponse(p)
        }
        (S::Play, D::ServerToClient, 0x2c) => {
            let p = play::read_vehicle_move_response(reader)?;
            Packet::VehicleMoveResponse(p)
        }
        (S::Play, D::ServerToClient, 0x2d) => {
            let p = play::read_open_book_response(reader)?;
            Packet::OpenBookResponse(p)
        }
        (S::Play, D::ServerToClient, 0x2e) => {
            let p = play::read_open_window_response(reader)?;
            Packet::OpenWindowResponse(p)
        }
        (S::Play, D::ServerToClient, 0x2f) => {
            let p = play::read_open_sign_entity_response(reader)?;
            Packet::OpenSignEntityResponse(p)
        }
        (S::Play, D::ServerToClient, 0x30) => {
            let p = play::read_play_ping_response(reader)?;
            Packet::PlayPingResponse(p)
        }
        (S::Play, D::ServerToClient, 0x31) => {
            let p = play::read_craft_recipe_response(reader)?;
            Packet::CraftRecipeResponse(p)
        }
        (S::Play, D::ServerToClient, 0x32) => {
            let p = play::read_abilities_response(reader)?;
            Packet::AbilitiesResponse(p)
        }
        (S::Play, D::ServerToClient, 0x33) => {
            let p = play::read_end_combat_event_response(reader)?;
            Packet::EndCombatEventResponse(p)
        }
        (S::Play, D::ServerToClient, 0x34) => {
            let p = play::read_enter_combat_event_response(reader)?;
            Packet::EnterCombatEventResponse(p)
        }
        (S::Play, D::ServerToClient, 0x35) => {
            let p = play::read_death_combat_event_response(reader)?;
            Packet::DeathCombatEventResponse(p)
        }
        (S::Play, D::ServerToClient, 0x36) => {
            let p = play::read_player_info_response(reader)?;
            Packet::PlayerInfoResponse(p)
        }
        (S::Play, D::ServerToClient, 0x37) => {
            let p = play::read_face_player_response(reader)?;
            Packet::FacePlayerResponse(p)
        }
        (S::Play, D::ServerToClient, 0x38) => {
            let p = play::read_position_response(reader)?;
            Packet::PositionResponse(p)
        }
        (S::Play, D::ServerToClient, 0x39) => {
            let p = play::read_unlock_recipes_response(reader)?;
            Packet::UnlockRecipesResponse(p)
        }
        (S::Play, D::ServerToClient, 0x3a) => {
            let p = play::read_entity_destroy_response(reader)?;
            Packet::EntityDestroyResponse(p)
        }
        (S::Play, D::ServerToClient, 0x3b) => {
            let p = play::read_remove_entity_effect_response(reader)?;
            Packet::RemoveEntityEffectResponse(p)
        }
        (S::Play, D::ServerToClient, 0x3c) => {
            let p = play::read_resource_pack_send_response(reader)?;
            Packet::ResourcePackSendResponse(p)
        }
        (S::Play, D::ServerToClient, 0x3d) => {
            let p = play::read_respawn_response(reader)?;
            Packet::RespawnResponse(p)
        }
        (S::Play, D::ServerToClient, 0x3e) => {
            let p = play::read_entity_head_rotation_response(reader)?;
            Packet::EntityHeadRotationResponse(p)
        }
        (S::Play, D::ServerToClient, 0x3f) => {
            let p = play::read_multi_block_change_response(reader)?;
            Packet::MultiBlockChangeResponse(p)
        }
        (S::Play, D::ServerToClient, 0x40) => {
            let p = play::read_select_advancement_tab_response(reader)?;
            Packet::SelectAdvancementTabResponse(p)
        }
        (S::Play, D::ServerToClient, 0x41) => {
            let p = play::read_action_bar_response(reader)?;
            Packet::ActionBarResponse(p)
        }
        (S::Play, D::ServerToClient, 0x42) => {
            let p = play::read_world_border_center_response(reader)?;
            Packet::WorldBorderCenterResponse(p)
        }
        (S::Play, D::ServerToClient, 0x43) => {
            let p = play::read_world_border_lerp_size_response(reader)?;
            Packet::WorldBorderLerpSizeResponse(p)
        }
        (S::Play, D::ServerToClient, 0x44) => {
            let p = play::read_world_border_size_response(reader)?;
            Packet::WorldBorderSizeResponse(p)
        }
        (S::Play, D::ServerToClient, 0x45) => {
            let p = play::read_world_border_warning_delay_response(reader)?;
            Packet::WorldBorderWarningDelayResponse(p)
        }
        (S::Play, D::ServerToClient, 0x46) => {
            let p = play::read_world_border_warning_reach_response(reader)?;
            Packet::WorldBorderWarningReachResponse(p)
        }
        (S::Play, D::ServerToClient, 0x47) => {
            let p = play::read_camera_response(reader)?;
            Packet::CameraResponse(p)
        }
        (S::Play, D::ServerToClient, 0x48) => {
            let p = play::read_held_item_slot_response(reader)?;
            Packet::HeldItemSlotResponse(p)
        }
        (S::Play, D::ServerToClient, 0x49) => {
            let p = play::read_update_view_position_response(reader)?;
            Packet::UpdateViewPositionResponse(p)
        }
        (S::Play, D::ServerToClient, 0x4a) => {
            let p = play::read_update_view_distance_response(reader)?;
            Packet::UpdateViewDistanceResponse(p)
        }
        (S::Play, D::ServerToClient, 0x4b) => {
            let p = play::read_spawn_position_response(reader)?;
            Packet::SpawnPositionResponse(p)
        }
        (S::Play, D::ServerToClient, 0x4c) => {
            let p = play::read_scoreboard_display_objective_response(reader)?;
            Packet::ScoreboardDisplayObjectiveResponse(p)
        }
        (S::Play, D::ServerToClient, 0x4d) => {
            let p = play::read_entity_metadata_response(reader)?;
            Packet::EntityMetadataResponse(p)
        }
        (S::Play, D::ServerToClient, 0x4e) => {
            let p = play::read_attach_entity_response(reader)?;
            Packet::AttachEntityResponse(p)
        }
        (S::Play, D::ServerToClient, 0x4f) => {
            let p = play::read_entity_velocity_response(reader)?;
            Packet::EntityVelocityResponse(p)
        }
        (S::Play, D::ServerToClient, 0x50) => {
            let p = play::read_entity_equipment_response(reader)?;
            Packet::EntityEquipmentResponse(p)
        }
        (S::Play, D::ServerToClient, 0x51) => {
            let p = play::read_experience_response(reader)?;
            Packet::ExperienceResponse(p)
        }
        (S::Play, D::ServerToClient, 0x52) => {
            let p = play::read_update_health_response(reader)?;
            Packet::UpdateHealthResponse(p)
        }
        (S::Play, D::ServerToClient, 0x53) => {
            let p = play::read_scoreboard_objective_response(reader)?;
            Packet::ScoreboardObjectiveResponse(p)
        }
        (S::Play, D::ServerToClient, 0x54) => {
            let p = play::read_set_passengers_response(reader)?;
            Packet::SetPassengersResponse(p)
        }
        (S::Play, D::ServerToClient, 0x55) => {
            let p = play::read_teams_response(reader)?;
            Packet::TeamsResponse(p)
        }
        (S::Play, D::ServerToClient, 0x56) => {
            let p = play::read_scoreboard_score_response(reader)?;
            Packet::ScoreboardScoreResponse(p)
        }
        (S::Play, D::ServerToClient, 0x57) => {
            let p = play::read_simulation_distance_response(reader)?;
            Packet::SimulationDistanceResponse(p)
        }
        (S::Play, D::ServerToClient, 0x58) => {
            let p = play::read_set_title_subtitle_response(reader)?;
            Packet::SetTitleSubtitleResponse(p)
        }
        (S::Play, D::ServerToClient, 0x59) => {
            let p = play::read_update_time_response(reader)?;
            Packet::UpdateTimeResponse(p)
        }
        (S::Play, D::ServerToClient, 0x5a) => {
            let p = play::read_set_title_text_response(reader)?;
            Packet::SetTitleTextResponse(p)
        }
        (S::Play, D::ServerToClient, 0x5b) => {
            let p = play::read_set_title_time_response(reader)?;
            Packet::SetTitleTimeResponse(p)
        }
        (S::Play, D::ServerToClient, 0x5c) => {
            let p = play::read_entity_sound_effect_response(reader)?;
            Packet::EntitySoundEffectResponse(p)
        }
        (S::Play, D::ServerToClient, 0x5d) => {
            let p = play::read_sound_effect_response(reader)?;
            Packet::SoundEffectResponse(p)
        }
        (S::Play, D::ServerToClient, 0x5e) => {
            let p = play::read_stop_sound_response(reader)?;
            Packet::StopSoundResponse(p)
        }
        (S::Play, D::ServerToClient, 0x5f) => {
            let p = play::read_playerlist_header_response(reader)?;
            Packet::PlayerlistHeaderResponse(p)
        }
        (S::Play, D::ServerToClient, 0x60) => {
            let p = play::read_nbt_query_response(reader)?;
            Packet::NbtQueryResponse(p)
        }
        (S::Play, D::ServerToClient, 0x61) => {
            let p = play::read_collect_response(reader)?;
            Packet::CollectResponse(p)
        }
        (S::Play, D::ServerToClient, 0x62) => {
            let p = play::read_entity_teleport_response(reader)?;
            Packet::EntityTeleportResponse(p)
        }
        (S::Play, D::ServerToClient, 0x63) => {
            let p = play::read_advancements_response(reader)?;
            Packet::AdvancementsResponse(p)
        }
        (S::Play, D::ServerToClient, 0x64) => {
            let p = play::read_entity_update_attributes_response(reader)?;
            Packet::EntityUpdateAttributesResponse(p)
        }
        (S::Play, D::ServerToClient, 0x65) => {
            let p = play::read_entity_effect_response(reader)?;
            Packet::EntityEffectResponse(p)
        }
        (S::Play, D::ServerToClient, 0x66) => {
            let p = play::read_declare_recipes_response(reader)?;
            Packet::DeclareRecipesResponse(p)
        }
        (S::Play, D::ServerToClient, 0x67) => {
            let p = play::read_tags_response(reader)?;
            Packet::TagsResponse(p)
        }
        _ => {
            return Err(anyhow!("unknown packet id={}", id));
        }
    };
    Ok(packet)
}
