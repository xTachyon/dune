#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]
pub mod handshaking {
    use crate::protocol::de::MinecraftDeserialize;
    use crate::protocol::de::Reader;
    use crate::protocol::varint::read_varint;
    use anyhow::Result;
    use core::marker::PhantomData;

    #[derive(Debug)]
    pub struct SetProtocolRequest<'p> {
        pub protocol_version: i32,
        pub server_host: &'p str,
        pub server_port: u16,
        pub next_state: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_set_protocol_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<SetProtocolRequest<'p>> {
        let protocol_version = read_varint(&mut reader)?;
        let server_host = reader.read_range()?;
        let server_port = MinecraftDeserialize::deserialize(&mut reader)?;
        let next_state = read_varint(&mut reader)?;
        let server_host = reader.get_str_from(server_host)?;

        let result = SetProtocolRequest {
            protocol_version,
            server_host,
            server_port,
            next_state,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct LegacyServerListPingRequest<'p> {
        pub payload: u8,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_legacy_server_list_ping_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<LegacyServerListPingRequest<'p>> {
        let payload = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = LegacyServerListPingRequest {
            payload,
            oof: PhantomData {},
        };
        Ok(result)
    }
}
pub mod status {
    use crate::protocol::de::MinecraftDeserialize;
    use crate::protocol::de::Reader;
    use crate::protocol::varint::read_varint;
    use anyhow::Result;
    use core::marker::PhantomData;

    #[derive(Debug)]
    pub struct PingStartRequest<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_ping_start_request<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<PingStartRequest<'p>> {
        let result = PingStartRequest {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct PingRequest<'p> {
        pub time: i64,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_ping_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<PingRequest<'p>> {
        let time = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = PingRequest {
            time,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ServerInfoResponse<'p> {
        pub response: &'p str,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_server_info_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<ServerInfoResponse<'p>> {
        let response = reader.read_range()?;
        let response = reader.get_str_from(response)?;

        let result = ServerInfoResponse {
            response,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct PingResponse<'p> {
        pub time: i64,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_ping_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<PingResponse<'p>> {
        let time = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = PingResponse {
            time,
            oof: PhantomData {},
        };
        Ok(result)
    }
}
pub mod login {
    use crate::protocol::de::MinecraftDeserialize;
    use crate::protocol::de::Reader;
    use crate::protocol::varint::read_varint;
    use anyhow::Result;
    use core::marker::PhantomData;

    #[derive(Debug)]
    pub struct LoginStartRequest<'p> {
        pub username: &'p str,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_login_start_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<LoginStartRequest<'p>> {
        let username = reader.read_range()?;
        let username = reader.get_str_from(username)?;

        let result = LoginStartRequest {
            username,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EncryptionBeginRequest<'p> {
        pub shared_secret: &'p [u8],
        pub verify_token: &'p [u8],
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_encryption_begin_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<EncryptionBeginRequest<'p>> {
        let shared_secret = reader.read_range()?;
        let verify_token = reader.read_range()?;
        let shared_secret = reader.get_buf_from(shared_secret)?;
        let verify_token = reader.get_buf_from(verify_token)?;

        let result = EncryptionBeginRequest {
            shared_secret,
            verify_token,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct LoginPluginResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_login_plugin_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<LoginPluginResponse<'p>> {
        let result = LoginPluginResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct DisconnectResponse<'p> {
        pub reason: &'p str,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_disconnect_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<DisconnectResponse<'p>> {
        let reason = reader.read_range()?;
        let reason = reader.get_str_from(reason)?;

        let result = DisconnectResponse {
            reason,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EncryptionBeginResponse<'p> {
        pub server_id: &'p str,
        pub public_key: &'p [u8],
        pub verify_token: &'p [u8],
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_encryption_begin_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<EncryptionBeginResponse<'p>> {
        let server_id = reader.read_range()?;
        let public_key = reader.read_range()?;
        let verify_token = reader.read_range()?;
        let server_id = reader.get_str_from(server_id)?;
        let public_key = reader.get_buf_from(public_key)?;
        let verify_token = reader.get_buf_from(verify_token)?;

        let result = EncryptionBeginResponse {
            server_id,
            public_key,
            verify_token,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SuccessResponse<'p> {
        pub uuid: u128,
        pub username: &'p str,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_success_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<SuccessResponse<'p>> {
        let uuid = MinecraftDeserialize::deserialize(&mut reader)?;
        let username = reader.read_range()?;
        let username = reader.get_str_from(username)?;

        let result = SuccessResponse {
            uuid,
            username,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct CompressResponse<'p> {
        pub threshold: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_compress_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<CompressResponse<'p>> {
        let threshold = read_varint(&mut reader)?;

        let result = CompressResponse {
            threshold,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct LoginPluginRequest<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_login_plugin_request<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<LoginPluginRequest<'p>> {
        let result = LoginPluginRequest {
            oof: PhantomData {},
        };
        Ok(result)
    }
}
pub mod play {
    use crate::protocol::de::MinecraftDeserialize;
    use crate::protocol::de::Reader;
    use crate::protocol::varint::read_varint;
    use anyhow::Result;
    use core::marker::PhantomData;

    #[derive(Debug)]
    pub struct TeleportConfirmRequest<'p> {
        pub teleport_id: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_teleport_confirm_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<TeleportConfirmRequest<'p>> {
        let teleport_id = read_varint(&mut reader)?;

        let result = TeleportConfirmRequest {
            teleport_id,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct QueryBlockNbtRequest<'p> {
        pub transaction_id: i32,
        pub location: crate::protocol::de::Position,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_query_block_nbt_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<QueryBlockNbtRequest<'p>> {
        let transaction_id = read_varint(&mut reader)?;
        let location = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = QueryBlockNbtRequest {
            transaction_id,
            location,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SetDifficultyRequest<'p> {
        pub new_difficulty: u8,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_set_difficulty_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<SetDifficultyRequest<'p>> {
        let new_difficulty = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = SetDifficultyRequest {
            new_difficulty,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EditBookRequest<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_edit_book_request<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<EditBookRequest<'p>> {
        let result = EditBookRequest {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct QueryEntityNbtRequest<'p> {
        pub transaction_id: i32,
        pub entity_id: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_query_entity_nbt_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<QueryEntityNbtRequest<'p>> {
        let transaction_id = read_varint(&mut reader)?;
        let entity_id = read_varint(&mut reader)?;

        let result = QueryEntityNbtRequest {
            transaction_id,
            entity_id,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct PickItemRequest<'p> {
        pub slot: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_pick_item_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<PickItemRequest<'p>> {
        let slot = read_varint(&mut reader)?;

        let result = PickItemRequest {
            slot,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct NameItemRequest<'p> {
        pub name: &'p str,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_name_item_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<NameItemRequest<'p>> {
        let name = reader.read_range()?;
        let name = reader.get_str_from(name)?;

        let result = NameItemRequest {
            name,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SelectTradeRequest<'p> {
        pub slot: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_select_trade_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<SelectTradeRequest<'p>> {
        let slot = read_varint(&mut reader)?;

        let result = SelectTradeRequest {
            slot,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SetBeaconEffectRequest<'p> {
        pub primary_effect: i32,
        pub secondary_effect: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_set_beacon_effect_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<SetBeaconEffectRequest<'p>> {
        let primary_effect = read_varint(&mut reader)?;
        let secondary_effect = read_varint(&mut reader)?;

        let result = SetBeaconEffectRequest {
            primary_effect,
            secondary_effect,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UpdateCommandBlockRequest<'p> {
        pub location: crate::protocol::de::Position,
        pub command: &'p str,
        pub mode: i32,
        pub flags: u8,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_update_command_block_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<UpdateCommandBlockRequest<'p>> {
        let location = MinecraftDeserialize::deserialize(&mut reader)?;
        let command = reader.read_range()?;
        let mode = read_varint(&mut reader)?;
        let flags = MinecraftDeserialize::deserialize(&mut reader)?;
        let command = reader.get_str_from(command)?;

        let result = UpdateCommandBlockRequest {
            location,
            command,
            mode,
            flags,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UpdateCommandBlockMinecartRequest<'p> {
        pub entity_id: i32,
        pub command: &'p str,
        pub track_output: bool,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_update_command_block_minecart_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<UpdateCommandBlockMinecartRequest<'p>> {
        let entity_id = read_varint(&mut reader)?;
        let command = reader.read_range()?;
        let track_output = MinecraftDeserialize::deserialize(&mut reader)?;
        let command = reader.get_str_from(command)?;

        let result = UpdateCommandBlockMinecartRequest {
            entity_id,
            command,
            track_output,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UpdateStructureBlockRequest<'p> {
        pub location: crate::protocol::de::Position,
        pub action: i32,
        pub mode: i32,
        pub name: &'p str,
        pub offset_x: u8,
        pub offset_y: u8,
        pub offset_z: u8,
        pub size_x: u8,
        pub size_y: u8,
        pub size_z: u8,
        pub mirror: i32,
        pub rotation: i32,
        pub metadata: &'p str,
        pub integrity: f32,
        pub seed: i32,
        pub flags: u8,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_update_structure_block_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<UpdateStructureBlockRequest<'p>> {
        let location = MinecraftDeserialize::deserialize(&mut reader)?;
        let action = read_varint(&mut reader)?;
        let mode = read_varint(&mut reader)?;
        let name = reader.read_range()?;
        let offset_x = MinecraftDeserialize::deserialize(&mut reader)?;
        let offset_y = MinecraftDeserialize::deserialize(&mut reader)?;
        let offset_z = MinecraftDeserialize::deserialize(&mut reader)?;
        let size_x = MinecraftDeserialize::deserialize(&mut reader)?;
        let size_y = MinecraftDeserialize::deserialize(&mut reader)?;
        let size_z = MinecraftDeserialize::deserialize(&mut reader)?;
        let mirror = read_varint(&mut reader)?;
        let rotation = read_varint(&mut reader)?;
        let metadata = reader.read_range()?;
        let integrity = MinecraftDeserialize::deserialize(&mut reader)?;
        let seed = read_varint(&mut reader)?;
        let flags = MinecraftDeserialize::deserialize(&mut reader)?;
        let name = reader.get_str_from(name)?;
        let metadata = reader.get_str_from(metadata)?;

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
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct TabCompleteRequest<'p> {
        pub transaction_id: i32,
        pub text: &'p str,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_tab_complete_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<TabCompleteRequest<'p>> {
        let transaction_id = read_varint(&mut reader)?;
        let text = reader.read_range()?;
        let text = reader.get_str_from(text)?;

        let result = TabCompleteRequest {
            transaction_id,
            text,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ChatRequest<'p> {
        pub message: &'p str,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_chat_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<ChatRequest<'p>> {
        let message = reader.read_range()?;
        let message = reader.get_str_from(message)?;

        let result = ChatRequest {
            message,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ClientCommandRequest<'p> {
        pub action_id: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_client_command_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<ClientCommandRequest<'p>> {
        let action_id = read_varint(&mut reader)?;

        let result = ClientCommandRequest {
            action_id,
            oof: PhantomData {},
        };
        Ok(result)
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
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_settings_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<SettingsRequest<'p>> {
        let locale = reader.read_range()?;
        let view_distance = MinecraftDeserialize::deserialize(&mut reader)?;
        let chat_flags = read_varint(&mut reader)?;
        let chat_colors = MinecraftDeserialize::deserialize(&mut reader)?;
        let skin_parts = MinecraftDeserialize::deserialize(&mut reader)?;
        let main_hand = read_varint(&mut reader)?;
        let enable_text_filtering = MinecraftDeserialize::deserialize(&mut reader)?;
        let enable_server_listing = MinecraftDeserialize::deserialize(&mut reader)?;
        let locale = reader.get_str_from(locale)?;

        let result = SettingsRequest {
            locale,
            view_distance,
            chat_flags,
            chat_colors,
            skin_parts,
            main_hand,
            enable_text_filtering,
            enable_server_listing,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EnchantItemRequest<'p> {
        pub window_id: i8,
        pub enchantment: i8,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_enchant_item_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<EnchantItemRequest<'p>> {
        let window_id = MinecraftDeserialize::deserialize(&mut reader)?;
        let enchantment = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = EnchantItemRequest {
            window_id,
            enchantment,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct WindowClickRequest<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_window_click_request<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<WindowClickRequest<'p>> {
        let result = WindowClickRequest {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct CloseWindowRequest<'p> {
        pub window_id: u8,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_close_window_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<CloseWindowRequest<'p>> {
        let window_id = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = CloseWindowRequest {
            window_id,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct CustomPayloadRequest<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_custom_payload_request<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<CustomPayloadRequest<'p>> {
        let result = CustomPayloadRequest {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UseEntityRequest<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_use_entity_request<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<UseEntityRequest<'p>> {
        let result = UseEntityRequest {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct GenerateStructureRequest<'p> {
        pub location: crate::protocol::de::Position,
        pub levels: i32,
        pub keep_jigsaws: bool,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_generate_structure_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<GenerateStructureRequest<'p>> {
        let location = MinecraftDeserialize::deserialize(&mut reader)?;
        let levels = read_varint(&mut reader)?;
        let keep_jigsaws = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = GenerateStructureRequest {
            location,
            levels,
            keep_jigsaws,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct KeepAliveRequest<'p> {
        pub keep_alive_id: i64,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_keep_alive_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<KeepAliveRequest<'p>> {
        let keep_alive_id = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = KeepAliveRequest {
            keep_alive_id,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct LockDifficultyRequest<'p> {
        pub locked: bool,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_lock_difficulty_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<LockDifficultyRequest<'p>> {
        let locked = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = LockDifficultyRequest {
            locked,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct PositionRequest<'p> {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub on_ground: bool,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_position_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<PositionRequest<'p>> {
        let x = MinecraftDeserialize::deserialize(&mut reader)?;
        let y = MinecraftDeserialize::deserialize(&mut reader)?;
        let z = MinecraftDeserialize::deserialize(&mut reader)?;
        let on_ground = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = PositionRequest {
            x,
            y,
            z,
            on_ground,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct PositionLookRequest<'p> {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub yaw: f32,
        pub pitch: f32,
        pub on_ground: bool,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_position_look_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<PositionLookRequest<'p>> {
        let x = MinecraftDeserialize::deserialize(&mut reader)?;
        let y = MinecraftDeserialize::deserialize(&mut reader)?;
        let z = MinecraftDeserialize::deserialize(&mut reader)?;
        let yaw = MinecraftDeserialize::deserialize(&mut reader)?;
        let pitch = MinecraftDeserialize::deserialize(&mut reader)?;
        let on_ground = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = PositionLookRequest {
            x,
            y,
            z,
            yaw,
            pitch,
            on_ground,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct LookRequest<'p> {
        pub yaw: f32,
        pub pitch: f32,
        pub on_ground: bool,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_look_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<LookRequest<'p>> {
        let yaw = MinecraftDeserialize::deserialize(&mut reader)?;
        let pitch = MinecraftDeserialize::deserialize(&mut reader)?;
        let on_ground = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = LookRequest {
            yaw,
            pitch,
            on_ground,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct FlyingRequest<'p> {
        pub on_ground: bool,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_flying_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<FlyingRequest<'p>> {
        let on_ground = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = FlyingRequest {
            on_ground,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct VehicleMoveRequest<'p> {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub yaw: f32,
        pub pitch: f32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_vehicle_move_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<VehicleMoveRequest<'p>> {
        let x = MinecraftDeserialize::deserialize(&mut reader)?;
        let y = MinecraftDeserialize::deserialize(&mut reader)?;
        let z = MinecraftDeserialize::deserialize(&mut reader)?;
        let yaw = MinecraftDeserialize::deserialize(&mut reader)?;
        let pitch = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = VehicleMoveRequest {
            x,
            y,
            z,
            yaw,
            pitch,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SteerBoatRequest<'p> {
        pub left_paddle: bool,
        pub right_paddle: bool,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_steer_boat_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<SteerBoatRequest<'p>> {
        let left_paddle = MinecraftDeserialize::deserialize(&mut reader)?;
        let right_paddle = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = SteerBoatRequest {
            left_paddle,
            right_paddle,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct CraftRecipeRequest<'p> {
        pub window_id: i8,
        pub recipe: &'p str,
        pub make_all: bool,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_craft_recipe_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<CraftRecipeRequest<'p>> {
        let window_id = MinecraftDeserialize::deserialize(&mut reader)?;
        let recipe = reader.read_range()?;
        let make_all = MinecraftDeserialize::deserialize(&mut reader)?;
        let recipe = reader.get_str_from(recipe)?;

        let result = CraftRecipeRequest {
            window_id,
            recipe,
            make_all,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct AbilitiesRequest<'p> {
        pub flags: i8,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_abilities_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<AbilitiesRequest<'p>> {
        let flags = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = AbilitiesRequest {
            flags,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct BlockDigRequest<'p> {
        pub status: i8,
        pub location: crate::protocol::de::Position,
        pub face: i8,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_block_dig_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<BlockDigRequest<'p>> {
        let status = MinecraftDeserialize::deserialize(&mut reader)?;
        let location = MinecraftDeserialize::deserialize(&mut reader)?;
        let face = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = BlockDigRequest {
            status,
            location,
            face,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityActionRequest<'p> {
        pub entity_id: i32,
        pub action_id: i32,
        pub jump_boost: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_entity_action_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<EntityActionRequest<'p>> {
        let entity_id = read_varint(&mut reader)?;
        let action_id = read_varint(&mut reader)?;
        let jump_boost = read_varint(&mut reader)?;

        let result = EntityActionRequest {
            entity_id,
            action_id,
            jump_boost,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SteerVehicleRequest<'p> {
        pub sideways: f32,
        pub forward: f32,
        pub jump: u8,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_steer_vehicle_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<SteerVehicleRequest<'p>> {
        let sideways = MinecraftDeserialize::deserialize(&mut reader)?;
        let forward = MinecraftDeserialize::deserialize(&mut reader)?;
        let jump = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = SteerVehicleRequest {
            sideways,
            forward,
            jump,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct DisplayedRecipeRequest<'p> {
        pub recipe_id: &'p str,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_displayed_recipe_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<DisplayedRecipeRequest<'p>> {
        let recipe_id = reader.read_range()?;
        let recipe_id = reader.get_str_from(recipe_id)?;

        let result = DisplayedRecipeRequest {
            recipe_id,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct RecipeBookRequest<'p> {
        pub book_id: i32,
        pub book_open: bool,
        pub filter_active: bool,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_recipe_book_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<RecipeBookRequest<'p>> {
        let book_id = read_varint(&mut reader)?;
        let book_open = MinecraftDeserialize::deserialize(&mut reader)?;
        let filter_active = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = RecipeBookRequest {
            book_id,
            book_open,
            filter_active,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ResourcePackReceiveRequest<'p> {
        pub result: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_resource_pack_receive_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<ResourcePackReceiveRequest<'p>> {
        let result = read_varint(&mut reader)?;

        let result = ResourcePackReceiveRequest {
            result,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct HeldItemSlotRequest<'p> {
        pub slot_id: i16,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_held_item_slot_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<HeldItemSlotRequest<'p>> {
        let slot_id = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = HeldItemSlotRequest {
            slot_id,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SetCreativeSlotRequest<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_set_creative_slot_request<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<SetCreativeSlotRequest<'p>> {
        let result = SetCreativeSlotRequest {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UpdateJigsawBlockRequest<'p> {
        pub location: crate::protocol::de::Position,
        pub name: &'p str,
        pub target: &'p str,
        pub pool: &'p str,
        pub final_state: &'p str,
        pub joint_type: &'p str,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_update_jigsaw_block_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<UpdateJigsawBlockRequest<'p>> {
        let location = MinecraftDeserialize::deserialize(&mut reader)?;
        let name = reader.read_range()?;
        let target = reader.read_range()?;
        let pool = reader.read_range()?;
        let final_state = reader.read_range()?;
        let joint_type = reader.read_range()?;
        let name = reader.get_str_from(name)?;
        let target = reader.get_str_from(target)?;
        let pool = reader.get_str_from(pool)?;
        let final_state = reader.get_str_from(final_state)?;
        let joint_type = reader.get_str_from(joint_type)?;

        let result = UpdateJigsawBlockRequest {
            location,
            name,
            target,
            pool,
            final_state,
            joint_type,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UpdateSignRequest<'p> {
        pub location: crate::protocol::de::Position,
        pub text1: &'p str,
        pub text2: &'p str,
        pub text3: &'p str,
        pub text4: &'p str,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_update_sign_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<UpdateSignRequest<'p>> {
        let location = MinecraftDeserialize::deserialize(&mut reader)?;
        let text1 = reader.read_range()?;
        let text2 = reader.read_range()?;
        let text3 = reader.read_range()?;
        let text4 = reader.read_range()?;
        let text1 = reader.get_str_from(text1)?;
        let text2 = reader.get_str_from(text2)?;
        let text3 = reader.get_str_from(text3)?;
        let text4 = reader.get_str_from(text4)?;

        let result = UpdateSignRequest {
            location,
            text1,
            text2,
            text3,
            text4,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ArmAnimationRequest<'p> {
        pub hand: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_arm_animation_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<ArmAnimationRequest<'p>> {
        let hand = read_varint(&mut reader)?;

        let result = ArmAnimationRequest {
            hand,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SpectateRequest<'p> {
        pub target: u128,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_spectate_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<SpectateRequest<'p>> {
        let target = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = SpectateRequest {
            target,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct BlockPlaceRequest<'p> {
        pub hand: i32,
        pub location: crate::protocol::de::Position,
        pub direction: i32,
        pub cursor_x: f32,
        pub cursor_y: f32,
        pub cursor_z: f32,
        pub inside_block: bool,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_block_place_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<BlockPlaceRequest<'p>> {
        let hand = read_varint(&mut reader)?;
        let location = MinecraftDeserialize::deserialize(&mut reader)?;
        let direction = read_varint(&mut reader)?;
        let cursor_x = MinecraftDeserialize::deserialize(&mut reader)?;
        let cursor_y = MinecraftDeserialize::deserialize(&mut reader)?;
        let cursor_z = MinecraftDeserialize::deserialize(&mut reader)?;
        let inside_block = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = BlockPlaceRequest {
            hand,
            location,
            direction,
            cursor_x,
            cursor_y,
            cursor_z,
            inside_block,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UseItemRequest<'p> {
        pub hand: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_use_item_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<UseItemRequest<'p>> {
        let hand = read_varint(&mut reader)?;

        let result = UseItemRequest {
            hand,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct AdvancementTabRequest<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_advancement_tab_request<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<AdvancementTabRequest<'p>> {
        let result = AdvancementTabRequest {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct PongRequest<'p> {
        pub id: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_pong_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<PongRequest<'p>> {
        let id = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = PongRequest {
            id,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SpawnEntityResponse<'p> {
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
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_spawn_entity_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<SpawnEntityResponse<'p>> {
        let entity_id = read_varint(&mut reader)?;
        let object_uuid = MinecraftDeserialize::deserialize(&mut reader)?;
        let type_ = read_varint(&mut reader)?;
        let x = MinecraftDeserialize::deserialize(&mut reader)?;
        let y = MinecraftDeserialize::deserialize(&mut reader)?;
        let z = MinecraftDeserialize::deserialize(&mut reader)?;
        let pitch = MinecraftDeserialize::deserialize(&mut reader)?;
        let yaw = MinecraftDeserialize::deserialize(&mut reader)?;
        let object_data = MinecraftDeserialize::deserialize(&mut reader)?;
        let velocity_x = MinecraftDeserialize::deserialize(&mut reader)?;
        let velocity_y = MinecraftDeserialize::deserialize(&mut reader)?;
        let velocity_z = MinecraftDeserialize::deserialize(&mut reader)?;

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
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SpawnEntityExperienceOrbResponse<'p> {
        pub entity_id: i32,
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub count: i16,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_spawn_entity_experience_orb_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<SpawnEntityExperienceOrbResponse<'p>> {
        let entity_id = read_varint(&mut reader)?;
        let x = MinecraftDeserialize::deserialize(&mut reader)?;
        let y = MinecraftDeserialize::deserialize(&mut reader)?;
        let z = MinecraftDeserialize::deserialize(&mut reader)?;
        let count = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = SpawnEntityExperienceOrbResponse {
            entity_id,
            x,
            y,
            z,
            count,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SpawnEntityLivingResponse<'p> {
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
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_spawn_entity_living_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<SpawnEntityLivingResponse<'p>> {
        let entity_id = read_varint(&mut reader)?;
        let entity_uuid = MinecraftDeserialize::deserialize(&mut reader)?;
        let type_ = read_varint(&mut reader)?;
        let x = MinecraftDeserialize::deserialize(&mut reader)?;
        let y = MinecraftDeserialize::deserialize(&mut reader)?;
        let z = MinecraftDeserialize::deserialize(&mut reader)?;
        let yaw = MinecraftDeserialize::deserialize(&mut reader)?;
        let pitch = MinecraftDeserialize::deserialize(&mut reader)?;
        let head_pitch = MinecraftDeserialize::deserialize(&mut reader)?;
        let velocity_x = MinecraftDeserialize::deserialize(&mut reader)?;
        let velocity_y = MinecraftDeserialize::deserialize(&mut reader)?;
        let velocity_z = MinecraftDeserialize::deserialize(&mut reader)?;

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
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SpawnEntityPaintingResponse<'p> {
        pub entity_id: i32,
        pub entity_uuid: u128,
        pub title: i32,
        pub location: crate::protocol::de::Position,
        pub direction: u8,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_spawn_entity_painting_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<SpawnEntityPaintingResponse<'p>> {
        let entity_id = read_varint(&mut reader)?;
        let entity_uuid = MinecraftDeserialize::deserialize(&mut reader)?;
        let title = read_varint(&mut reader)?;
        let location = MinecraftDeserialize::deserialize(&mut reader)?;
        let direction = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = SpawnEntityPaintingResponse {
            entity_id,
            entity_uuid,
            title,
            location,
            direction,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct NamedEntitySpawnResponse<'p> {
        pub entity_id: i32,
        pub player_uuid: u128,
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub yaw: i8,
        pub pitch: i8,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_named_entity_spawn_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<NamedEntitySpawnResponse<'p>> {
        let entity_id = read_varint(&mut reader)?;
        let player_uuid = MinecraftDeserialize::deserialize(&mut reader)?;
        let x = MinecraftDeserialize::deserialize(&mut reader)?;
        let y = MinecraftDeserialize::deserialize(&mut reader)?;
        let z = MinecraftDeserialize::deserialize(&mut reader)?;
        let yaw = MinecraftDeserialize::deserialize(&mut reader)?;
        let pitch = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = NamedEntitySpawnResponse {
            entity_id,
            player_uuid,
            x,
            y,
            z,
            yaw,
            pitch,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct AnimationResponse<'p> {
        pub entity_id: i32,
        pub animation: u8,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_animation_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<AnimationResponse<'p>> {
        let entity_id = read_varint(&mut reader)?;
        let animation = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = AnimationResponse {
            entity_id,
            animation,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct StatisticsResponseCategory_IdStatistic_IdValue<'p> {
        pub category_id: i32,
        pub statistic_id: i32,
        pub value: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_statistics_response_category__id_statistic__id_value<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<StatisticsResponseCategory_IdStatistic_IdValue<'p>> {
        let category_id = read_varint(&mut reader)?;
        let statistic_id = read_varint(&mut reader)?;
        let value = read_varint(&mut reader)?;

        let result = StatisticsResponseCategory_IdStatistic_IdValue {
            category_id,
            statistic_id,
            value,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct StatisticsResponse<'p> {
        pub entries: Vec<StatisticsResponseCategory_IdStatistic_IdValue<'p>>,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_statistics_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<StatisticsResponse<'p>> {
        let count_array = read_varint(&mut reader)?;
        let mut entries = Vec::with_capacity(count_array as usize);
        for _ in 0..count_array {
            let x = packet_statistics_response_category__id_statistic__id_value(reader)?;
            entries.push(x);
        }

        let result = StatisticsResponse {
            entries,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct AdvancementsResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_advancements_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<AdvancementsResponse<'p>> {
        let result = AdvancementsResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct BlockBreakAnimationResponse<'p> {
        pub entity_id: i32,
        pub location: crate::protocol::de::Position,
        pub destroy_stage: i8,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_block_break_animation_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<BlockBreakAnimationResponse<'p>> {
        let entity_id = read_varint(&mut reader)?;
        let location = MinecraftDeserialize::deserialize(&mut reader)?;
        let destroy_stage = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = BlockBreakAnimationResponse {
            entity_id,
            location,
            destroy_stage,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct TileEntityDataResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_tile_entity_data_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<TileEntityDataResponse<'p>> {
        let result = TileEntityDataResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct BlockActionResponse<'p> {
        pub location: crate::protocol::de::Position,
        pub byte1: u8,
        pub byte2: u8,
        pub block_id: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_block_action_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<BlockActionResponse<'p>> {
        let location = MinecraftDeserialize::deserialize(&mut reader)?;
        let byte1 = MinecraftDeserialize::deserialize(&mut reader)?;
        let byte2 = MinecraftDeserialize::deserialize(&mut reader)?;
        let block_id = read_varint(&mut reader)?;

        let result = BlockActionResponse {
            location,
            byte1,
            byte2,
            block_id,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct BlockChangeResponse<'p> {
        pub location: crate::protocol::de::Position,
        pub type_: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_block_change_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<BlockChangeResponse<'p>> {
        let location = MinecraftDeserialize::deserialize(&mut reader)?;
        let type_ = read_varint(&mut reader)?;

        let result = BlockChangeResponse {
            location,
            type_,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct BossBarResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_boss_bar_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<BossBarResponse<'p>> {
        let result = BossBarResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct DifficultyResponse<'p> {
        pub difficulty: u8,
        pub difficulty_locked: bool,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_difficulty_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<DifficultyResponse<'p>> {
        let difficulty = MinecraftDeserialize::deserialize(&mut reader)?;
        let difficulty_locked = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = DifficultyResponse {
            difficulty,
            difficulty_locked,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct TabCompleteResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_tab_complete_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<TabCompleteResponse<'p>> {
        let result = TabCompleteResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct DeclareCommandsResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_declare_commands_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<DeclareCommandsResponse<'p>> {
        let result = DeclareCommandsResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct FacePlayerResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_face_player_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<FacePlayerResponse<'p>> {
        let result = FacePlayerResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct NbtQueryResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_nbt_query_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<NbtQueryResponse<'p>> {
        let result = NbtQueryResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ChatResponse<'p> {
        pub message: &'p str,
        pub position: i8,
        pub sender: u128,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_chat_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<ChatResponse<'p>> {
        let message = reader.read_range()?;
        let position = MinecraftDeserialize::deserialize(&mut reader)?;
        let sender = MinecraftDeserialize::deserialize(&mut reader)?;
        let message = reader.get_str_from(message)?;

        let result = ChatResponse {
            message,
            position,
            sender,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct MultiBlockChangeResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_multi_block_change_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<MultiBlockChangeResponse<'p>> {
        let result = MultiBlockChangeResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct CloseWindowResponse<'p> {
        pub window_id: u8,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_close_window_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<CloseWindowResponse<'p>> {
        let window_id = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = CloseWindowResponse {
            window_id,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct OpenWindowResponse<'p> {
        pub window_id: i32,
        pub inventory_type: i32,
        pub window_title: &'p str,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_open_window_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<OpenWindowResponse<'p>> {
        let window_id = read_varint(&mut reader)?;
        let inventory_type = read_varint(&mut reader)?;
        let window_title = reader.read_range()?;
        let window_title = reader.get_str_from(window_title)?;

        let result = OpenWindowResponse {
            window_id,
            inventory_type,
            window_title,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct WindowItemsResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_window_items_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<WindowItemsResponse<'p>> {
        let result = WindowItemsResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct CraftProgressBarResponse<'p> {
        pub window_id: u8,
        pub property: i16,
        pub value: i16,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_craft_progress_bar_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<CraftProgressBarResponse<'p>> {
        let window_id = MinecraftDeserialize::deserialize(&mut reader)?;
        let property = MinecraftDeserialize::deserialize(&mut reader)?;
        let value = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = CraftProgressBarResponse {
            window_id,
            property,
            value,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SetSlotResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_set_slot_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<SetSlotResponse<'p>> {
        let result = SetSlotResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SetCooldownResponse<'p> {
        pub item_id: i32,
        pub cooldown_ticks: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_set_cooldown_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<SetCooldownResponse<'p>> {
        let item_id = read_varint(&mut reader)?;
        let cooldown_ticks = read_varint(&mut reader)?;

        let result = SetCooldownResponse {
            item_id,
            cooldown_ticks,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct CustomPayloadResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_custom_payload_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<CustomPayloadResponse<'p>> {
        let result = CustomPayloadResponse {
            oof: PhantomData {},
        };
        Ok(result)
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
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_named_sound_effect_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<NamedSoundEffectResponse<'p>> {
        let sound_name = reader.read_range()?;
        let sound_category = read_varint(&mut reader)?;
        let x = MinecraftDeserialize::deserialize(&mut reader)?;
        let y = MinecraftDeserialize::deserialize(&mut reader)?;
        let z = MinecraftDeserialize::deserialize(&mut reader)?;
        let volume = MinecraftDeserialize::deserialize(&mut reader)?;
        let pitch = MinecraftDeserialize::deserialize(&mut reader)?;
        let sound_name = reader.get_str_from(sound_name)?;

        let result = NamedSoundEffectResponse {
            sound_name,
            sound_category,
            x,
            y,
            z,
            volume,
            pitch,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct KickDisconnectResponse<'p> {
        pub reason: &'p str,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_kick_disconnect_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<KickDisconnectResponse<'p>> {
        let reason = reader.read_range()?;
        let reason = reader.get_str_from(reason)?;

        let result = KickDisconnectResponse {
            reason,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityStatusResponse<'p> {
        pub entity_id: i32,
        pub entity_status: i8,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_entity_status_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<EntityStatusResponse<'p>> {
        let entity_id = MinecraftDeserialize::deserialize(&mut reader)?;
        let entity_status = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = EntityStatusResponse {
            entity_id,
            entity_status,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ExplosionResponseXYZ<'p> {
        pub x: i8,
        pub y: i8,
        pub z: i8,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_explosion_response_xyz<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<ExplosionResponseXYZ<'p>> {
        let x = MinecraftDeserialize::deserialize(&mut reader)?;
        let y = MinecraftDeserialize::deserialize(&mut reader)?;
        let z = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = ExplosionResponseXYZ {
            x,
            y,
            z,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ExplosionResponse<'p> {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub radius: f32,
        pub affected_block_offsets: Vec<ExplosionResponseXYZ<'p>>,
        pub player_motion_x: f32,
        pub player_motion_y: f32,
        pub player_motion_z: f32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_explosion_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<ExplosionResponse<'p>> {
        let x = MinecraftDeserialize::deserialize(&mut reader)?;
        let y = MinecraftDeserialize::deserialize(&mut reader)?;
        let z = MinecraftDeserialize::deserialize(&mut reader)?;
        let radius = MinecraftDeserialize::deserialize(&mut reader)?;
        let count_array = read_varint(&mut reader)?;
        let mut affected_block_offsets = Vec::with_capacity(count_array as usize);
        for _ in 0..count_array {
            let x = packet_explosion_response_xyz(reader)?;
            affected_block_offsets.push(x);
        }
        let player_motion_x = MinecraftDeserialize::deserialize(&mut reader)?;
        let player_motion_y = MinecraftDeserialize::deserialize(&mut reader)?;
        let player_motion_z = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = ExplosionResponse {
            x,
            y,
            z,
            radius,
            affected_block_offsets,
            player_motion_x,
            player_motion_y,
            player_motion_z,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UnloadChunkResponse<'p> {
        pub chunk_x: i32,
        pub chunk_z: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_unload_chunk_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<UnloadChunkResponse<'p>> {
        let chunk_x = MinecraftDeserialize::deserialize(&mut reader)?;
        let chunk_z = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = UnloadChunkResponse {
            chunk_x,
            chunk_z,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct GameStateChangeResponse<'p> {
        pub reason: u8,
        pub game_mode: f32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_game_state_change_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<GameStateChangeResponse<'p>> {
        let reason = MinecraftDeserialize::deserialize(&mut reader)?;
        let game_mode = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = GameStateChangeResponse {
            reason,
            game_mode,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct OpenHorseWindowResponse<'p> {
        pub window_id: u8,
        pub nb_slots: i32,
        pub entity_id: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_open_horse_window_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<OpenHorseWindowResponse<'p>> {
        let window_id = MinecraftDeserialize::deserialize(&mut reader)?;
        let nb_slots = read_varint(&mut reader)?;
        let entity_id = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = OpenHorseWindowResponse {
            window_id,
            nb_slots,
            entity_id,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct KeepAliveResponse<'p> {
        pub keep_alive_id: i64,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_keep_alive_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<KeepAliveResponse<'p>> {
        let keep_alive_id = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = KeepAliveResponse {
            keep_alive_id,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct MapChunkResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_map_chunk_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<MapChunkResponse<'p>> {
        let result = MapChunkResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct WorldEventResponse<'p> {
        pub effect_id: i32,
        pub location: crate::protocol::de::Position,
        pub data: i32,
        pub global: bool,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_world_event_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<WorldEventResponse<'p>> {
        let effect_id = MinecraftDeserialize::deserialize(&mut reader)?;
        let location = MinecraftDeserialize::deserialize(&mut reader)?;
        let data = MinecraftDeserialize::deserialize(&mut reader)?;
        let global = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = WorldEventResponse {
            effect_id,
            location,
            data,
            global,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct WorldParticlesResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_world_particles_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<WorldParticlesResponse<'p>> {
        let result = WorldParticlesResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UpdateLightResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_update_light_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<UpdateLightResponse<'p>> {
        let result = UpdateLightResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct LoginResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_login_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<LoginResponse<'p>> {
        let result = LoginResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct MapResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_map_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<MapResponse<'p>> {
        let result = MapResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct TradeListResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_trade_list_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<TradeListResponse<'p>> {
        let result = TradeListResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct RelEntityMoveResponse<'p> {
        pub entity_id: i32,
        pub d_x: i16,
        pub d_y: i16,
        pub d_z: i16,
        pub on_ground: bool,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_rel_entity_move_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<RelEntityMoveResponse<'p>> {
        let entity_id = read_varint(&mut reader)?;
        let d_x = MinecraftDeserialize::deserialize(&mut reader)?;
        let d_y = MinecraftDeserialize::deserialize(&mut reader)?;
        let d_z = MinecraftDeserialize::deserialize(&mut reader)?;
        let on_ground = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = RelEntityMoveResponse {
            entity_id,
            d_x,
            d_y,
            d_z,
            on_ground,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityMoveLookResponse<'p> {
        pub entity_id: i32,
        pub d_x: i16,
        pub d_y: i16,
        pub d_z: i16,
        pub yaw: i8,
        pub pitch: i8,
        pub on_ground: bool,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_entity_move_look_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<EntityMoveLookResponse<'p>> {
        let entity_id = read_varint(&mut reader)?;
        let d_x = MinecraftDeserialize::deserialize(&mut reader)?;
        let d_y = MinecraftDeserialize::deserialize(&mut reader)?;
        let d_z = MinecraftDeserialize::deserialize(&mut reader)?;
        let yaw = MinecraftDeserialize::deserialize(&mut reader)?;
        let pitch = MinecraftDeserialize::deserialize(&mut reader)?;
        let on_ground = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = EntityMoveLookResponse {
            entity_id,
            d_x,
            d_y,
            d_z,
            yaw,
            pitch,
            on_ground,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityLookResponse<'p> {
        pub entity_id: i32,
        pub yaw: i8,
        pub pitch: i8,
        pub on_ground: bool,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_entity_look_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<EntityLookResponse<'p>> {
        let entity_id = read_varint(&mut reader)?;
        let yaw = MinecraftDeserialize::deserialize(&mut reader)?;
        let pitch = MinecraftDeserialize::deserialize(&mut reader)?;
        let on_ground = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = EntityLookResponse {
            entity_id,
            yaw,
            pitch,
            on_ground,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct VehicleMoveResponse<'p> {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub yaw: f32,
        pub pitch: f32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_vehicle_move_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<VehicleMoveResponse<'p>> {
        let x = MinecraftDeserialize::deserialize(&mut reader)?;
        let y = MinecraftDeserialize::deserialize(&mut reader)?;
        let z = MinecraftDeserialize::deserialize(&mut reader)?;
        let yaw = MinecraftDeserialize::deserialize(&mut reader)?;
        let pitch = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = VehicleMoveResponse {
            x,
            y,
            z,
            yaw,
            pitch,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct OpenBookResponse<'p> {
        pub hand: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_open_book_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<OpenBookResponse<'p>> {
        let hand = read_varint(&mut reader)?;

        let result = OpenBookResponse {
            hand,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct OpenSignEntityResponse<'p> {
        pub location: crate::protocol::de::Position,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_open_sign_entity_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<OpenSignEntityResponse<'p>> {
        let location = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = OpenSignEntityResponse {
            location,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct CraftRecipeResponse<'p> {
        pub window_id: i8,
        pub recipe: &'p str,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_craft_recipe_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<CraftRecipeResponse<'p>> {
        let window_id = MinecraftDeserialize::deserialize(&mut reader)?;
        let recipe = reader.read_range()?;
        let recipe = reader.get_str_from(recipe)?;

        let result = CraftRecipeResponse {
            window_id,
            recipe,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct AbilitiesResponse<'p> {
        pub flags: i8,
        pub flying_speed: f32,
        pub walking_speed: f32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_abilities_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<AbilitiesResponse<'p>> {
        let flags = MinecraftDeserialize::deserialize(&mut reader)?;
        let flying_speed = MinecraftDeserialize::deserialize(&mut reader)?;
        let walking_speed = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = AbilitiesResponse {
            flags,
            flying_speed,
            walking_speed,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EndCombatEventResponse<'p> {
        pub duration: i32,
        pub entity_id: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_end_combat_event_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<EndCombatEventResponse<'p>> {
        let duration = read_varint(&mut reader)?;
        let entity_id = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = EndCombatEventResponse {
            duration,
            entity_id,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EnterCombatEventResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_enter_combat_event_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<EnterCombatEventResponse<'p>> {
        let result = EnterCombatEventResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct DeathCombatEventResponse<'p> {
        pub player_id: i32,
        pub entity_id: i32,
        pub message: &'p str,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_death_combat_event_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<DeathCombatEventResponse<'p>> {
        let player_id = read_varint(&mut reader)?;
        let entity_id = MinecraftDeserialize::deserialize(&mut reader)?;
        let message = reader.read_range()?;
        let message = reader.get_str_from(message)?;

        let result = DeathCombatEventResponse {
            player_id,
            entity_id,
            message,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct PlayerInfoResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_player_info_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<PlayerInfoResponse<'p>> {
        let result = PlayerInfoResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct PositionResponse<'p> {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub yaw: f32,
        pub pitch: f32,
        pub flags: i8,
        pub teleport_id: i32,
        pub dismount_vehicle: bool,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_position_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<PositionResponse<'p>> {
        let x = MinecraftDeserialize::deserialize(&mut reader)?;
        let y = MinecraftDeserialize::deserialize(&mut reader)?;
        let z = MinecraftDeserialize::deserialize(&mut reader)?;
        let yaw = MinecraftDeserialize::deserialize(&mut reader)?;
        let pitch = MinecraftDeserialize::deserialize(&mut reader)?;
        let flags = MinecraftDeserialize::deserialize(&mut reader)?;
        let teleport_id = read_varint(&mut reader)?;
        let dismount_vehicle = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = PositionResponse {
            x,
            y,
            z,
            yaw,
            pitch,
            flags,
            teleport_id,
            dismount_vehicle,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UnlockRecipesResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_unlock_recipes_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<UnlockRecipesResponse<'p>> {
        let result = UnlockRecipesResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityDestroyResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_entity_destroy_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<EntityDestroyResponse<'p>> {
        let result = EntityDestroyResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct RemoveEntityEffectResponse<'p> {
        pub entity_id: i32,
        pub effect_id: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_remove_entity_effect_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<RemoveEntityEffectResponse<'p>> {
        let entity_id = read_varint(&mut reader)?;
        let effect_id = read_varint(&mut reader)?;

        let result = RemoveEntityEffectResponse {
            entity_id,
            effect_id,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ResourcePackSendResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_resource_pack_send_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<ResourcePackSendResponse<'p>> {
        let result = ResourcePackSendResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct RespawnResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_respawn_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<RespawnResponse<'p>> {
        let result = RespawnResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityHeadRotationResponse<'p> {
        pub entity_id: i32,
        pub head_yaw: i8,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_entity_head_rotation_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<EntityHeadRotationResponse<'p>> {
        let entity_id = read_varint(&mut reader)?;
        let head_yaw = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = EntityHeadRotationResponse {
            entity_id,
            head_yaw,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct CameraResponse<'p> {
        pub camera_id: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_camera_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<CameraResponse<'p>> {
        let camera_id = read_varint(&mut reader)?;

        let result = CameraResponse {
            camera_id,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct HeldItemSlotResponse<'p> {
        pub slot: i8,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_held_item_slot_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<HeldItemSlotResponse<'p>> {
        let slot = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = HeldItemSlotResponse {
            slot,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UpdateViewPositionResponse<'p> {
        pub chunk_x: i32,
        pub chunk_z: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_update_view_position_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<UpdateViewPositionResponse<'p>> {
        let chunk_x = read_varint(&mut reader)?;
        let chunk_z = read_varint(&mut reader)?;

        let result = UpdateViewPositionResponse {
            chunk_x,
            chunk_z,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UpdateViewDistanceResponse<'p> {
        pub view_distance: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_update_view_distance_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<UpdateViewDistanceResponse<'p>> {
        let view_distance = read_varint(&mut reader)?;

        let result = UpdateViewDistanceResponse {
            view_distance,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ScoreboardDisplayObjectiveResponse<'p> {
        pub position: i8,
        pub name: &'p str,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_scoreboard_display_objective_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<ScoreboardDisplayObjectiveResponse<'p>> {
        let position = MinecraftDeserialize::deserialize(&mut reader)?;
        let name = reader.read_range()?;
        let name = reader.get_str_from(name)?;

        let result = ScoreboardDisplayObjectiveResponse {
            position,
            name,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityMetadataResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_entity_metadata_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<EntityMetadataResponse<'p>> {
        let result = EntityMetadataResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct AttachEntityResponse<'p> {
        pub entity_id: i32,
        pub vehicle_id: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_attach_entity_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<AttachEntityResponse<'p>> {
        let entity_id = MinecraftDeserialize::deserialize(&mut reader)?;
        let vehicle_id = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = AttachEntityResponse {
            entity_id,
            vehicle_id,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityVelocityResponse<'p> {
        pub entity_id: i32,
        pub velocity_x: i16,
        pub velocity_y: i16,
        pub velocity_z: i16,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_entity_velocity_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<EntityVelocityResponse<'p>> {
        let entity_id = read_varint(&mut reader)?;
        let velocity_x = MinecraftDeserialize::deserialize(&mut reader)?;
        let velocity_y = MinecraftDeserialize::deserialize(&mut reader)?;
        let velocity_z = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = EntityVelocityResponse {
            entity_id,
            velocity_x,
            velocity_y,
            velocity_z,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityEquipmentResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_entity_equipment_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<EntityEquipmentResponse<'p>> {
        let result = EntityEquipmentResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ExperienceResponse<'p> {
        pub experience_bar: f32,
        pub level: i32,
        pub total_experience: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_experience_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<ExperienceResponse<'p>> {
        let experience_bar = MinecraftDeserialize::deserialize(&mut reader)?;
        let level = read_varint(&mut reader)?;
        let total_experience = read_varint(&mut reader)?;

        let result = ExperienceResponse {
            experience_bar,
            level,
            total_experience,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UpdateHealthResponse<'p> {
        pub health: f32,
        pub food: i32,
        pub food_saturation: f32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_update_health_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<UpdateHealthResponse<'p>> {
        let health = MinecraftDeserialize::deserialize(&mut reader)?;
        let food = read_varint(&mut reader)?;
        let food_saturation = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = UpdateHealthResponse {
            health,
            food,
            food_saturation,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ScoreboardObjectiveResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_scoreboard_objective_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<ScoreboardObjectiveResponse<'p>> {
        let result = ScoreboardObjectiveResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SetPassengersResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_set_passengers_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<SetPassengersResponse<'p>> {
        let result = SetPassengersResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct TeamsResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_teams_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<TeamsResponse<'p>> {
        let result = TeamsResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ScoreboardScoreResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_scoreboard_score_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<ScoreboardScoreResponse<'p>> {
        let result = ScoreboardScoreResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SpawnPositionResponse<'p> {
        pub location: crate::protocol::de::Position,
        pub angle: f32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_spawn_position_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<SpawnPositionResponse<'p>> {
        let location = MinecraftDeserialize::deserialize(&mut reader)?;
        let angle = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = SpawnPositionResponse {
            location,
            angle,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UpdateTimeResponse<'p> {
        pub age: i64,
        pub time: i64,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_update_time_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<UpdateTimeResponse<'p>> {
        let age = MinecraftDeserialize::deserialize(&mut reader)?;
        let time = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = UpdateTimeResponse {
            age,
            time,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntitySoundEffectResponse<'p> {
        pub sound_id: i32,
        pub sound_category: i32,
        pub entity_id: i32,
        pub volume: f32,
        pub pitch: f32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_entity_sound_effect_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<EntitySoundEffectResponse<'p>> {
        let sound_id = read_varint(&mut reader)?;
        let sound_category = read_varint(&mut reader)?;
        let entity_id = read_varint(&mut reader)?;
        let volume = MinecraftDeserialize::deserialize(&mut reader)?;
        let pitch = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = EntitySoundEffectResponse {
            sound_id,
            sound_category,
            entity_id,
            volume,
            pitch,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct StopSoundResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_stop_sound_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<StopSoundResponse<'p>> {
        let result = StopSoundResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SoundEffectResponse<'p> {
        pub sound_id: i32,
        pub sound_category: i32,
        pub x: i32,
        pub y: i32,
        pub z: i32,
        pub volume: f32,
        pub pitch: f32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_sound_effect_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<SoundEffectResponse<'p>> {
        let sound_id = read_varint(&mut reader)?;
        let sound_category = read_varint(&mut reader)?;
        let x = MinecraftDeserialize::deserialize(&mut reader)?;
        let y = MinecraftDeserialize::deserialize(&mut reader)?;
        let z = MinecraftDeserialize::deserialize(&mut reader)?;
        let volume = MinecraftDeserialize::deserialize(&mut reader)?;
        let pitch = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = SoundEffectResponse {
            sound_id,
            sound_category,
            x,
            y,
            z,
            volume,
            pitch,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct PlayerlistHeaderResponse<'p> {
        pub header: &'p str,
        pub footer: &'p str,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_playerlist_header_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<PlayerlistHeaderResponse<'p>> {
        let header = reader.read_range()?;
        let footer = reader.read_range()?;
        let header = reader.get_str_from(header)?;
        let footer = reader.get_str_from(footer)?;

        let result = PlayerlistHeaderResponse {
            header,
            footer,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct CollectResponse<'p> {
        pub collected_entity_id: i32,
        pub collector_entity_id: i32,
        pub pickup_item_count: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_collect_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<CollectResponse<'p>> {
        let collected_entity_id = read_varint(&mut reader)?;
        let collector_entity_id = read_varint(&mut reader)?;
        let pickup_item_count = read_varint(&mut reader)?;

        let result = CollectResponse {
            collected_entity_id,
            collector_entity_id,
            pickup_item_count,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityTeleportResponse<'p> {
        pub entity_id: i32,
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub yaw: i8,
        pub pitch: i8,
        pub on_ground: bool,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_entity_teleport_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<EntityTeleportResponse<'p>> {
        let entity_id = read_varint(&mut reader)?;
        let x = MinecraftDeserialize::deserialize(&mut reader)?;
        let y = MinecraftDeserialize::deserialize(&mut reader)?;
        let z = MinecraftDeserialize::deserialize(&mut reader)?;
        let yaw = MinecraftDeserialize::deserialize(&mut reader)?;
        let pitch = MinecraftDeserialize::deserialize(&mut reader)?;
        let on_ground = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = EntityTeleportResponse {
            entity_id,
            x,
            y,
            z,
            yaw,
            pitch,
            on_ground,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityUpdateAttributesResponseUuidAmountOperation<'p> {
        pub uuid: u128,
        pub amount: f64,
        pub operation: i8,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_entity_update_attributes_response_uuid_amount_operation<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<EntityUpdateAttributesResponseUuidAmountOperation<'p>> {
        let uuid = MinecraftDeserialize::deserialize(&mut reader)?;
        let amount = MinecraftDeserialize::deserialize(&mut reader)?;
        let operation = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = EntityUpdateAttributesResponseUuidAmountOperation {
            uuid,
            amount,
            operation,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityUpdateAttributesResponseKeyValueModifiers<'p> {
        pub key: &'p str,
        pub value: f64,
        pub modifiers: Vec<EntityUpdateAttributesResponseUuidAmountOperation<'p>>,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_entity_update_attributes_response_key_value_modifiers<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<EntityUpdateAttributesResponseKeyValueModifiers<'p>> {
        let key = reader.read_range()?;
        let value = MinecraftDeserialize::deserialize(&mut reader)?;
        let count_array = read_varint(&mut reader)?;
        let mut modifiers = Vec::with_capacity(count_array as usize);
        for _ in 0..count_array {
            let x = packet_entity_update_attributes_response_uuid_amount_operation(reader)?;
            modifiers.push(x);
        }
        let key = reader.get_str_from(key)?;

        let result = EntityUpdateAttributesResponseKeyValueModifiers {
            key,
            value,
            modifiers,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityUpdateAttributesResponse<'p> {
        pub entity_id: i32,
        pub properties: Vec<EntityUpdateAttributesResponseKeyValueModifiers<'p>>,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_entity_update_attributes_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<EntityUpdateAttributesResponse<'p>> {
        let entity_id = read_varint(&mut reader)?;
        let count_array = read_varint(&mut reader)?;
        let mut properties = Vec::with_capacity(count_array as usize);
        for _ in 0..count_array {
            let x = packet_entity_update_attributes_response_key_value_modifiers(reader)?;
            properties.push(x);
        }

        let result = EntityUpdateAttributesResponse {
            entity_id,
            properties,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityEffectResponse<'p> {
        pub entity_id: i32,
        pub effect_id: i32,
        pub amplifier: i8,
        pub duration: i32,
        pub hide_particles: i8,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_entity_effect_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<EntityEffectResponse<'p>> {
        let entity_id = read_varint(&mut reader)?;
        let effect_id = read_varint(&mut reader)?;
        let amplifier = MinecraftDeserialize::deserialize(&mut reader)?;
        let duration = read_varint(&mut reader)?;
        let hide_particles = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = EntityEffectResponse {
            entity_id,
            effect_id,
            amplifier,
            duration,
            hide_particles,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SelectAdvancementTabResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_select_advancement_tab_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<SelectAdvancementTabResponse<'p>> {
        let result = SelectAdvancementTabResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct DeclareRecipesResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_declare_recipes_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<DeclareRecipesResponse<'p>> {
        let result = DeclareRecipesResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct TagsResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_tags_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<TagsResponse<'p>> {
        let result = TagsResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct AcknowledgePlayerDiggingResponse<'p> {
        pub location: crate::protocol::de::Position,
        pub block: i32,
        pub status: i32,
        pub successful: bool,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_acknowledge_player_digging_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<AcknowledgePlayerDiggingResponse<'p>> {
        let location = MinecraftDeserialize::deserialize(&mut reader)?;
        let block = read_varint(&mut reader)?;
        let status = read_varint(&mut reader)?;
        let successful = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = AcknowledgePlayerDiggingResponse {
            location,
            block,
            status,
            successful,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SculkVibrationSignalResponse<'p> {
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_sculk_vibration_signal_response<'p>(
        mut _reader: &'p mut Reader<'p>,
    ) -> Result<SculkVibrationSignalResponse<'p>> {
        let result = SculkVibrationSignalResponse {
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ClearTitlesResponse<'p> {
        pub reset: bool,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_clear_titles_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<ClearTitlesResponse<'p>> {
        let reset = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = ClearTitlesResponse {
            reset,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct InitializeWorldBorderResponse<'p> {
        pub x: f64,
        pub z: f64,
        pub old_diameter: f64,
        pub new_diameter: f64,
        pub speed: i32,
        pub portal_teleport_boundary: i32,
        pub warning_blocks: i32,
        pub warning_time: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_initialize_world_border_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<InitializeWorldBorderResponse<'p>> {
        let x = MinecraftDeserialize::deserialize(&mut reader)?;
        let z = MinecraftDeserialize::deserialize(&mut reader)?;
        let old_diameter = MinecraftDeserialize::deserialize(&mut reader)?;
        let new_diameter = MinecraftDeserialize::deserialize(&mut reader)?;
        let speed = read_varint(&mut reader)?;
        let portal_teleport_boundary = read_varint(&mut reader)?;
        let warning_blocks = read_varint(&mut reader)?;
        let warning_time = read_varint(&mut reader)?;

        let result = InitializeWorldBorderResponse {
            x,
            z,
            old_diameter,
            new_diameter,
            speed,
            portal_teleport_boundary,
            warning_blocks,
            warning_time,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ActionBarResponse<'p> {
        pub text: &'p str,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_action_bar_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<ActionBarResponse<'p>> {
        let text = reader.read_range()?;
        let text = reader.get_str_from(text)?;

        let result = ActionBarResponse {
            text,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct WorldBorderCenterResponse<'p> {
        pub x: f64,
        pub z: f64,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_world_border_center_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<WorldBorderCenterResponse<'p>> {
        let x = MinecraftDeserialize::deserialize(&mut reader)?;
        let z = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = WorldBorderCenterResponse {
            x,
            z,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct WorldBorderLerpSizeResponse<'p> {
        pub old_diameter: f64,
        pub new_diameter: f64,
        pub speed: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_world_border_lerp_size_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<WorldBorderLerpSizeResponse<'p>> {
        let old_diameter = MinecraftDeserialize::deserialize(&mut reader)?;
        let new_diameter = MinecraftDeserialize::deserialize(&mut reader)?;
        let speed = read_varint(&mut reader)?;

        let result = WorldBorderLerpSizeResponse {
            old_diameter,
            new_diameter,
            speed,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct WorldBorderSizeResponse<'p> {
        pub diameter: f64,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_world_border_size_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<WorldBorderSizeResponse<'p>> {
        let diameter = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = WorldBorderSizeResponse {
            diameter,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct WorldBorderWarningDelayResponse<'p> {
        pub warning_time: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_world_border_warning_delay_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<WorldBorderWarningDelayResponse<'p>> {
        let warning_time = read_varint(&mut reader)?;

        let result = WorldBorderWarningDelayResponse {
            warning_time,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct WorldBorderWarningReachResponse<'p> {
        pub warning_blocks: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_world_border_warning_reach_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<WorldBorderWarningReachResponse<'p>> {
        let warning_blocks = read_varint(&mut reader)?;

        let result = WorldBorderWarningReachResponse {
            warning_blocks,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct PlayPingResponse<'p> {
        pub id: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_play_ping_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<PlayPingResponse<'p>> {
        let id = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = PlayPingResponse {
            id,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SetTitleSubtitleResponse<'p> {
        pub text: &'p str,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_set_title_subtitle_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<SetTitleSubtitleResponse<'p>> {
        let text = reader.read_range()?;
        let text = reader.get_str_from(text)?;

        let result = SetTitleSubtitleResponse {
            text,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SetTitleTextResponse<'p> {
        pub text: &'p str,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_set_title_text_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<SetTitleTextResponse<'p>> {
        let text = reader.read_range()?;
        let text = reader.get_str_from(text)?;

        let result = SetTitleTextResponse {
            text,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SetTitleTimeResponse<'p> {
        pub fade_in: i32,
        pub stay: i32,
        pub fade_out: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_set_title_time_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<SetTitleTimeResponse<'p>> {
        let fade_in = MinecraftDeserialize::deserialize(&mut reader)?;
        let stay = MinecraftDeserialize::deserialize(&mut reader)?;
        let fade_out = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = SetTitleTimeResponse {
            fade_in,
            stay,
            fade_out,
            oof: PhantomData {},
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SimulationDistanceResponse<'p> {
        pub distance: i32,
        pub oof: PhantomData<&'p ()>,
    }
    pub(super) fn packet_simulation_distance_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<SimulationDistanceResponse<'p>> {
        let distance = read_varint(&mut reader)?;

        let result = SimulationDistanceResponse {
            distance,
            oof: PhantomData {},
        };
        Ok(result)
    }
}
use crate::protocol::de::Reader;
use crate::protocol::ConnectionState as S;
use crate::protocol::PacketDirection as D;
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub enum Packet<'p> {
    SetProtocolRequest(handshaking::SetProtocolRequest<'p>),
    LegacyServerListPingRequest(handshaking::LegacyServerListPingRequest<'p>),
    PingStartRequest(status::PingStartRequest<'p>),
    PingRequest(status::PingRequest<'p>),
    ServerInfoResponse(status::ServerInfoResponse<'p>),
    PingResponse(status::PingResponse<'p>),
    LoginStartRequest(login::LoginStartRequest<'p>),
    EncryptionBeginRequest(login::EncryptionBeginRequest<'p>),
    LoginPluginResponse(login::LoginPluginResponse<'p>),
    DisconnectResponse(login::DisconnectResponse<'p>),
    EncryptionBeginResponse(login::EncryptionBeginResponse<'p>),
    SuccessResponse(login::SuccessResponse<'p>),
    CompressResponse(login::CompressResponse<'p>),
    LoginPluginRequest(login::LoginPluginRequest<'p>),
    TeleportConfirmRequest(play::TeleportConfirmRequest<'p>),
    QueryBlockNbtRequest(play::QueryBlockNbtRequest<'p>),
    SetDifficultyRequest(play::SetDifficultyRequest<'p>),
    EditBookRequest(play::EditBookRequest<'p>),
    QueryEntityNbtRequest(play::QueryEntityNbtRequest<'p>),
    PickItemRequest(play::PickItemRequest<'p>),
    NameItemRequest(play::NameItemRequest<'p>),
    SelectTradeRequest(play::SelectTradeRequest<'p>),
    SetBeaconEffectRequest(play::SetBeaconEffectRequest<'p>),
    UpdateCommandBlockRequest(play::UpdateCommandBlockRequest<'p>),
    UpdateCommandBlockMinecartRequest(play::UpdateCommandBlockMinecartRequest<'p>),
    UpdateStructureBlockRequest(play::UpdateStructureBlockRequest<'p>),
    TabCompleteRequest(play::TabCompleteRequest<'p>),
    ChatRequest(play::ChatRequest<'p>),
    ClientCommandRequest(play::ClientCommandRequest<'p>),
    SettingsRequest(play::SettingsRequest<'p>),
    EnchantItemRequest(play::EnchantItemRequest<'p>),
    WindowClickRequest(play::WindowClickRequest<'p>),
    CloseWindowRequest(play::CloseWindowRequest<'p>),
    CustomPayloadRequest(play::CustomPayloadRequest<'p>),
    UseEntityRequest(play::UseEntityRequest<'p>),
    GenerateStructureRequest(play::GenerateStructureRequest<'p>),
    KeepAliveRequest(play::KeepAliveRequest<'p>),
    LockDifficultyRequest(play::LockDifficultyRequest<'p>),
    PositionRequest(play::PositionRequest<'p>),
    PositionLookRequest(play::PositionLookRequest<'p>),
    LookRequest(play::LookRequest<'p>),
    FlyingRequest(play::FlyingRequest<'p>),
    VehicleMoveRequest(play::VehicleMoveRequest<'p>),
    SteerBoatRequest(play::SteerBoatRequest<'p>),
    CraftRecipeRequest(play::CraftRecipeRequest<'p>),
    AbilitiesRequest(play::AbilitiesRequest<'p>),
    BlockDigRequest(play::BlockDigRequest<'p>),
    EntityActionRequest(play::EntityActionRequest<'p>),
    SteerVehicleRequest(play::SteerVehicleRequest<'p>),
    DisplayedRecipeRequest(play::DisplayedRecipeRequest<'p>),
    RecipeBookRequest(play::RecipeBookRequest<'p>),
    ResourcePackReceiveRequest(play::ResourcePackReceiveRequest<'p>),
    HeldItemSlotRequest(play::HeldItemSlotRequest<'p>),
    SetCreativeSlotRequest(play::SetCreativeSlotRequest<'p>),
    UpdateJigsawBlockRequest(play::UpdateJigsawBlockRequest<'p>),
    UpdateSignRequest(play::UpdateSignRequest<'p>),
    ArmAnimationRequest(play::ArmAnimationRequest<'p>),
    SpectateRequest(play::SpectateRequest<'p>),
    BlockPlaceRequest(play::BlockPlaceRequest<'p>),
    UseItemRequest(play::UseItemRequest<'p>),
    AdvancementTabRequest(play::AdvancementTabRequest<'p>),
    PongRequest(play::PongRequest<'p>),
    SpawnEntityResponse(play::SpawnEntityResponse<'p>),
    SpawnEntityExperienceOrbResponse(play::SpawnEntityExperienceOrbResponse<'p>),
    SpawnEntityLivingResponse(play::SpawnEntityLivingResponse<'p>),
    SpawnEntityPaintingResponse(play::SpawnEntityPaintingResponse<'p>),
    NamedEntitySpawnResponse(play::NamedEntitySpawnResponse<'p>),
    AnimationResponse(play::AnimationResponse<'p>),
    StatisticsResponse(play::StatisticsResponse<'p>),
    AdvancementsResponse(play::AdvancementsResponse<'p>),
    BlockBreakAnimationResponse(play::BlockBreakAnimationResponse<'p>),
    TileEntityDataResponse(play::TileEntityDataResponse<'p>),
    BlockActionResponse(play::BlockActionResponse<'p>),
    BlockChangeResponse(play::BlockChangeResponse<'p>),
    BossBarResponse(play::BossBarResponse<'p>),
    DifficultyResponse(play::DifficultyResponse<'p>),
    TabCompleteResponse(play::TabCompleteResponse<'p>),
    DeclareCommandsResponse(play::DeclareCommandsResponse<'p>),
    FacePlayerResponse(play::FacePlayerResponse<'p>),
    NbtQueryResponse(play::NbtQueryResponse<'p>),
    ChatResponse(play::ChatResponse<'p>),
    MultiBlockChangeResponse(play::MultiBlockChangeResponse<'p>),
    CloseWindowResponse(play::CloseWindowResponse<'p>),
    OpenWindowResponse(play::OpenWindowResponse<'p>),
    WindowItemsResponse(play::WindowItemsResponse<'p>),
    CraftProgressBarResponse(play::CraftProgressBarResponse<'p>),
    SetSlotResponse(play::SetSlotResponse<'p>),
    SetCooldownResponse(play::SetCooldownResponse<'p>),
    CustomPayloadResponse(play::CustomPayloadResponse<'p>),
    NamedSoundEffectResponse(play::NamedSoundEffectResponse<'p>),
    KickDisconnectResponse(play::KickDisconnectResponse<'p>),
    EntityStatusResponse(play::EntityStatusResponse<'p>),
    ExplosionResponse(play::ExplosionResponse<'p>),
    UnloadChunkResponse(play::UnloadChunkResponse<'p>),
    GameStateChangeResponse(play::GameStateChangeResponse<'p>),
    OpenHorseWindowResponse(play::OpenHorseWindowResponse<'p>),
    KeepAliveResponse(play::KeepAliveResponse<'p>),
    MapChunkResponse(play::MapChunkResponse<'p>),
    WorldEventResponse(play::WorldEventResponse<'p>),
    WorldParticlesResponse(play::WorldParticlesResponse<'p>),
    UpdateLightResponse(play::UpdateLightResponse<'p>),
    LoginResponse(play::LoginResponse<'p>),
    MapResponse(play::MapResponse<'p>),
    TradeListResponse(play::TradeListResponse<'p>),
    RelEntityMoveResponse(play::RelEntityMoveResponse<'p>),
    EntityMoveLookResponse(play::EntityMoveLookResponse<'p>),
    EntityLookResponse(play::EntityLookResponse<'p>),
    VehicleMoveResponse(play::VehicleMoveResponse<'p>),
    OpenBookResponse(play::OpenBookResponse<'p>),
    OpenSignEntityResponse(play::OpenSignEntityResponse<'p>),
    CraftRecipeResponse(play::CraftRecipeResponse<'p>),
    AbilitiesResponse(play::AbilitiesResponse<'p>),
    EndCombatEventResponse(play::EndCombatEventResponse<'p>),
    EnterCombatEventResponse(play::EnterCombatEventResponse<'p>),
    DeathCombatEventResponse(play::DeathCombatEventResponse<'p>),
    PlayerInfoResponse(play::PlayerInfoResponse<'p>),
    PositionResponse(play::PositionResponse<'p>),
    UnlockRecipesResponse(play::UnlockRecipesResponse<'p>),
    EntityDestroyResponse(play::EntityDestroyResponse<'p>),
    RemoveEntityEffectResponse(play::RemoveEntityEffectResponse<'p>),
    ResourcePackSendResponse(play::ResourcePackSendResponse<'p>),
    RespawnResponse(play::RespawnResponse<'p>),
    EntityHeadRotationResponse(play::EntityHeadRotationResponse<'p>),
    CameraResponse(play::CameraResponse<'p>),
    HeldItemSlotResponse(play::HeldItemSlotResponse<'p>),
    UpdateViewPositionResponse(play::UpdateViewPositionResponse<'p>),
    UpdateViewDistanceResponse(play::UpdateViewDistanceResponse<'p>),
    ScoreboardDisplayObjectiveResponse(play::ScoreboardDisplayObjectiveResponse<'p>),
    EntityMetadataResponse(play::EntityMetadataResponse<'p>),
    AttachEntityResponse(play::AttachEntityResponse<'p>),
    EntityVelocityResponse(play::EntityVelocityResponse<'p>),
    EntityEquipmentResponse(play::EntityEquipmentResponse<'p>),
    ExperienceResponse(play::ExperienceResponse<'p>),
    UpdateHealthResponse(play::UpdateHealthResponse<'p>),
    ScoreboardObjectiveResponse(play::ScoreboardObjectiveResponse<'p>),
    SetPassengersResponse(play::SetPassengersResponse<'p>),
    TeamsResponse(play::TeamsResponse<'p>),
    ScoreboardScoreResponse(play::ScoreboardScoreResponse<'p>),
    SpawnPositionResponse(play::SpawnPositionResponse<'p>),
    UpdateTimeResponse(play::UpdateTimeResponse<'p>),
    EntitySoundEffectResponse(play::EntitySoundEffectResponse<'p>),
    StopSoundResponse(play::StopSoundResponse<'p>),
    SoundEffectResponse(play::SoundEffectResponse<'p>),
    PlayerlistHeaderResponse(play::PlayerlistHeaderResponse<'p>),
    CollectResponse(play::CollectResponse<'p>),
    EntityTeleportResponse(play::EntityTeleportResponse<'p>),
    EntityUpdateAttributesResponse(play::EntityUpdateAttributesResponse<'p>),
    EntityEffectResponse(play::EntityEffectResponse<'p>),
    SelectAdvancementTabResponse(play::SelectAdvancementTabResponse<'p>),
    DeclareRecipesResponse(play::DeclareRecipesResponse<'p>),
    TagsResponse(play::TagsResponse<'p>),
    AcknowledgePlayerDiggingResponse(play::AcknowledgePlayerDiggingResponse<'p>),
    SculkVibrationSignalResponse(play::SculkVibrationSignalResponse<'p>),
    ClearTitlesResponse(play::ClearTitlesResponse<'p>),
    InitializeWorldBorderResponse(play::InitializeWorldBorderResponse<'p>),
    ActionBarResponse(play::ActionBarResponse<'p>),
    WorldBorderCenterResponse(play::WorldBorderCenterResponse<'p>),
    WorldBorderLerpSizeResponse(play::WorldBorderLerpSizeResponse<'p>),
    WorldBorderSizeResponse(play::WorldBorderSizeResponse<'p>),
    WorldBorderWarningDelayResponse(play::WorldBorderWarningDelayResponse<'p>),
    WorldBorderWarningReachResponse(play::WorldBorderWarningReachResponse<'p>),
    PlayPingResponse(play::PlayPingResponse<'p>),
    SetTitleSubtitleResponse(play::SetTitleSubtitleResponse<'p>),
    SetTitleTextResponse(play::SetTitleTextResponse<'p>),
    SetTitleTimeResponse(play::SetTitleTimeResponse<'p>),
    SimulationDistanceResponse(play::SimulationDistanceResponse<'p>),
}

pub fn de_packets<'r>(
    state: S,
    direction: D,
    id: u32,
    reader: &'r mut Reader<'r>,
) -> Result<Packet> {
    let packet = match (state, direction, id) {
        (S::Handshaking, D::ClientToServer, 0x0) => {
            let p = handshaking::packet_set_protocol_request(reader)?;
            Packet::SetProtocolRequest(p)
        }
        (S::Handshaking, D::ClientToServer, 0xfe) => {
            let p = handshaking::packet_legacy_server_list_ping_request(reader)?;
            Packet::LegacyServerListPingRequest(p)
        }
        (S::Status, D::ClientToServer, 0x0) => {
            let p = status::packet_ping_start_request(reader)?;
            Packet::PingStartRequest(p)
        }
        (S::Status, D::ClientToServer, 0x1) => {
            let p = status::packet_ping_request(reader)?;
            Packet::PingRequest(p)
        }
        (S::Status, D::ServerToClient, 0x0) => {
            let p = status::packet_server_info_response(reader)?;
            Packet::ServerInfoResponse(p)
        }
        (S::Status, D::ServerToClient, 0x1) => {
            let p = status::packet_ping_response(reader)?;
            Packet::PingResponse(p)
        }
        (S::Login, D::ClientToServer, 0x0) => {
            let p = login::packet_login_start_request(reader)?;
            Packet::LoginStartRequest(p)
        }
        (S::Login, D::ClientToServer, 0x1) => {
            let p = login::packet_encryption_begin_request(reader)?;
            Packet::EncryptionBeginRequest(p)
        }
        (S::Login, D::ClientToServer, 0x2) => {
            let p = login::packet_login_plugin_response(reader)?;
            Packet::LoginPluginResponse(p)
        }
        (S::Login, D::ServerToClient, 0x0) => {
            let p = login::packet_disconnect_response(reader)?;
            Packet::DisconnectResponse(p)
        }
        (S::Login, D::ServerToClient, 0x1) => {
            let p = login::packet_encryption_begin_response(reader)?;
            Packet::EncryptionBeginResponse(p)
        }
        (S::Login, D::ServerToClient, 0x2) => {
            let p = login::packet_success_response(reader)?;
            Packet::SuccessResponse(p)
        }
        (S::Login, D::ServerToClient, 0x3) => {
            let p = login::packet_compress_response(reader)?;
            Packet::CompressResponse(p)
        }
        (S::Login, D::ServerToClient, 0x4) => {
            let p = login::packet_login_plugin_request(reader)?;
            Packet::LoginPluginRequest(p)
        }
        (S::Play, D::ClientToServer, 0x0) => {
            let p = play::packet_teleport_confirm_request(reader)?;
            Packet::TeleportConfirmRequest(p)
        }
        (S::Play, D::ClientToServer, 0x1) => {
            let p = play::packet_query_block_nbt_request(reader)?;
            Packet::QueryBlockNbtRequest(p)
        }
        (S::Play, D::ClientToServer, 0x2) => {
            let p = play::packet_set_difficulty_request(reader)?;
            Packet::SetDifficultyRequest(p)
        }
        (S::Play, D::ClientToServer, 0x3) => {
            let p = play::packet_chat_request(reader)?;
            Packet::ChatRequest(p)
        }
        (S::Play, D::ClientToServer, 0x4) => {
            let p = play::packet_client_command_request(reader)?;
            Packet::ClientCommandRequest(p)
        }
        (S::Play, D::ClientToServer, 0x5) => {
            let p = play::packet_settings_request(reader)?;
            Packet::SettingsRequest(p)
        }
        (S::Play, D::ClientToServer, 0x6) => {
            let p = play::packet_tab_complete_request(reader)?;
            Packet::TabCompleteRequest(p)
        }
        (S::Play, D::ClientToServer, 0x7) => {
            let p = play::packet_enchant_item_request(reader)?;
            Packet::EnchantItemRequest(p)
        }
        (S::Play, D::ClientToServer, 0x8) => {
            let p = play::packet_window_click_request(reader)?;
            Packet::WindowClickRequest(p)
        }
        (S::Play, D::ClientToServer, 0x9) => {
            let p = play::packet_close_window_request(reader)?;
            Packet::CloseWindowRequest(p)
        }
        (S::Play, D::ClientToServer, 0xa) => {
            let p = play::packet_custom_payload_request(reader)?;
            Packet::CustomPayloadRequest(p)
        }
        (S::Play, D::ClientToServer, 0xb) => {
            let p = play::packet_edit_book_request(reader)?;
            Packet::EditBookRequest(p)
        }
        (S::Play, D::ClientToServer, 0xc) => {
            let p = play::packet_query_entity_nbt_request(reader)?;
            Packet::QueryEntityNbtRequest(p)
        }
        (S::Play, D::ClientToServer, 0xd) => {
            let p = play::packet_use_entity_request(reader)?;
            Packet::UseEntityRequest(p)
        }
        (S::Play, D::ClientToServer, 0xe) => {
            let p = play::packet_generate_structure_request(reader)?;
            Packet::GenerateStructureRequest(p)
        }
        (S::Play, D::ClientToServer, 0xf) => {
            let p = play::packet_keep_alive_request(reader)?;
            Packet::KeepAliveRequest(p)
        }
        (S::Play, D::ClientToServer, 0x10) => {
            let p = play::packet_lock_difficulty_request(reader)?;
            Packet::LockDifficultyRequest(p)
        }
        (S::Play, D::ClientToServer, 0x11) => {
            let p = play::packet_position_request(reader)?;
            Packet::PositionRequest(p)
        }
        (S::Play, D::ClientToServer, 0x12) => {
            let p = play::packet_position_look_request(reader)?;
            Packet::PositionLookRequest(p)
        }
        (S::Play, D::ClientToServer, 0x13) => {
            let p = play::packet_look_request(reader)?;
            Packet::LookRequest(p)
        }
        (S::Play, D::ClientToServer, 0x14) => {
            let p = play::packet_flying_request(reader)?;
            Packet::FlyingRequest(p)
        }
        (S::Play, D::ClientToServer, 0x15) => {
            let p = play::packet_vehicle_move_request(reader)?;
            Packet::VehicleMoveRequest(p)
        }
        (S::Play, D::ClientToServer, 0x16) => {
            let p = play::packet_steer_boat_request(reader)?;
            Packet::SteerBoatRequest(p)
        }
        (S::Play, D::ClientToServer, 0x17) => {
            let p = play::packet_pick_item_request(reader)?;
            Packet::PickItemRequest(p)
        }
        (S::Play, D::ClientToServer, 0x18) => {
            let p = play::packet_craft_recipe_request(reader)?;
            Packet::CraftRecipeRequest(p)
        }
        (S::Play, D::ClientToServer, 0x19) => {
            let p = play::packet_abilities_request(reader)?;
            Packet::AbilitiesRequest(p)
        }
        (S::Play, D::ClientToServer, 0x1a) => {
            let p = play::packet_block_dig_request(reader)?;
            Packet::BlockDigRequest(p)
        }
        (S::Play, D::ClientToServer, 0x1b) => {
            let p = play::packet_entity_action_request(reader)?;
            Packet::EntityActionRequest(p)
        }
        (S::Play, D::ClientToServer, 0x1c) => {
            let p = play::packet_steer_vehicle_request(reader)?;
            Packet::SteerVehicleRequest(p)
        }
        (S::Play, D::ClientToServer, 0x1d) => {
            let p = play::packet_pong_request(reader)?;
            Packet::PongRequest(p)
        }
        (S::Play, D::ClientToServer, 0x1e) => {
            let p = play::packet_recipe_book_request(reader)?;
            Packet::RecipeBookRequest(p)
        }
        (S::Play, D::ClientToServer, 0x1f) => {
            let p = play::packet_displayed_recipe_request(reader)?;
            Packet::DisplayedRecipeRequest(p)
        }
        (S::Play, D::ClientToServer, 0x20) => {
            let p = play::packet_name_item_request(reader)?;
            Packet::NameItemRequest(p)
        }
        (S::Play, D::ClientToServer, 0x21) => {
            let p = play::packet_resource_pack_receive_request(reader)?;
            Packet::ResourcePackReceiveRequest(p)
        }
        (S::Play, D::ClientToServer, 0x22) => {
            let p = play::packet_advancement_tab_request(reader)?;
            Packet::AdvancementTabRequest(p)
        }
        (S::Play, D::ClientToServer, 0x23) => {
            let p = play::packet_select_trade_request(reader)?;
            Packet::SelectTradeRequest(p)
        }
        (S::Play, D::ClientToServer, 0x24) => {
            let p = play::packet_set_beacon_effect_request(reader)?;
            Packet::SetBeaconEffectRequest(p)
        }
        (S::Play, D::ClientToServer, 0x25) => {
            let p = play::packet_held_item_slot_request(reader)?;
            Packet::HeldItemSlotRequest(p)
        }
        (S::Play, D::ClientToServer, 0x26) => {
            let p = play::packet_update_command_block_request(reader)?;
            Packet::UpdateCommandBlockRequest(p)
        }
        (S::Play, D::ClientToServer, 0x27) => {
            let p = play::packet_update_command_block_minecart_request(reader)?;
            Packet::UpdateCommandBlockMinecartRequest(p)
        }
        (S::Play, D::ClientToServer, 0x28) => {
            let p = play::packet_set_creative_slot_request(reader)?;
            Packet::SetCreativeSlotRequest(p)
        }
        (S::Play, D::ClientToServer, 0x29) => {
            let p = play::packet_update_jigsaw_block_request(reader)?;
            Packet::UpdateJigsawBlockRequest(p)
        }
        (S::Play, D::ClientToServer, 0x2a) => {
            let p = play::packet_update_structure_block_request(reader)?;
            Packet::UpdateStructureBlockRequest(p)
        }
        (S::Play, D::ClientToServer, 0x2b) => {
            let p = play::packet_update_sign_request(reader)?;
            Packet::UpdateSignRequest(p)
        }
        (S::Play, D::ClientToServer, 0x2c) => {
            let p = play::packet_arm_animation_request(reader)?;
            Packet::ArmAnimationRequest(p)
        }
        (S::Play, D::ClientToServer, 0x2d) => {
            let p = play::packet_spectate_request(reader)?;
            Packet::SpectateRequest(p)
        }
        (S::Play, D::ClientToServer, 0x2e) => {
            let p = play::packet_block_place_request(reader)?;
            Packet::BlockPlaceRequest(p)
        }
        (S::Play, D::ClientToServer, 0x2f) => {
            let p = play::packet_use_item_request(reader)?;
            Packet::UseItemRequest(p)
        }
        (S::Play, D::ServerToClient, 0x0) => {
            let p = play::packet_spawn_entity_response(reader)?;
            Packet::SpawnEntityResponse(p)
        }
        (S::Play, D::ServerToClient, 0x1) => {
            let p = play::packet_spawn_entity_experience_orb_response(reader)?;
            Packet::SpawnEntityExperienceOrbResponse(p)
        }
        (S::Play, D::ServerToClient, 0x2) => {
            let p = play::packet_spawn_entity_living_response(reader)?;
            Packet::SpawnEntityLivingResponse(p)
        }
        (S::Play, D::ServerToClient, 0x3) => {
            let p = play::packet_spawn_entity_painting_response(reader)?;
            Packet::SpawnEntityPaintingResponse(p)
        }
        (S::Play, D::ServerToClient, 0x4) => {
            let p = play::packet_named_entity_spawn_response(reader)?;
            Packet::NamedEntitySpawnResponse(p)
        }
        (S::Play, D::ServerToClient, 0x5) => {
            let p = play::packet_sculk_vibration_signal_response(reader)?;
            Packet::SculkVibrationSignalResponse(p)
        }
        (S::Play, D::ServerToClient, 0x6) => {
            let p = play::packet_animation_response(reader)?;
            Packet::AnimationResponse(p)
        }
        (S::Play, D::ServerToClient, 0x7) => {
            let p = play::packet_statistics_response(reader)?;
            Packet::StatisticsResponse(p)
        }
        (S::Play, D::ServerToClient, 0x8) => {
            let p = play::packet_acknowledge_player_digging_response(reader)?;
            Packet::AcknowledgePlayerDiggingResponse(p)
        }
        (S::Play, D::ServerToClient, 0x9) => {
            let p = play::packet_block_break_animation_response(reader)?;
            Packet::BlockBreakAnimationResponse(p)
        }
        (S::Play, D::ServerToClient, 0xa) => {
            let p = play::packet_tile_entity_data_response(reader)?;
            Packet::TileEntityDataResponse(p)
        }
        (S::Play, D::ServerToClient, 0xb) => {
            let p = play::packet_block_action_response(reader)?;
            Packet::BlockActionResponse(p)
        }
        (S::Play, D::ServerToClient, 0xc) => {
            let p = play::packet_block_change_response(reader)?;
            Packet::BlockChangeResponse(p)
        }
        (S::Play, D::ServerToClient, 0xd) => {
            let p = play::packet_boss_bar_response(reader)?;
            Packet::BossBarResponse(p)
        }
        (S::Play, D::ServerToClient, 0xe) => {
            let p = play::packet_difficulty_response(reader)?;
            Packet::DifficultyResponse(p)
        }
        (S::Play, D::ServerToClient, 0xf) => {
            let p = play::packet_chat_response(reader)?;
            Packet::ChatResponse(p)
        }
        (S::Play, D::ServerToClient, 0x10) => {
            let p = play::packet_clear_titles_response(reader)?;
            Packet::ClearTitlesResponse(p)
        }
        (S::Play, D::ServerToClient, 0x11) => {
            let p = play::packet_tab_complete_response(reader)?;
            Packet::TabCompleteResponse(p)
        }
        (S::Play, D::ServerToClient, 0x12) => {
            let p = play::packet_declare_commands_response(reader)?;
            Packet::DeclareCommandsResponse(p)
        }
        (S::Play, D::ServerToClient, 0x13) => {
            let p = play::packet_close_window_response(reader)?;
            Packet::CloseWindowResponse(p)
        }
        (S::Play, D::ServerToClient, 0x14) => {
            let p = play::packet_window_items_response(reader)?;
            Packet::WindowItemsResponse(p)
        }
        (S::Play, D::ServerToClient, 0x15) => {
            let p = play::packet_craft_progress_bar_response(reader)?;
            Packet::CraftProgressBarResponse(p)
        }
        (S::Play, D::ServerToClient, 0x16) => {
            let p = play::packet_set_slot_response(reader)?;
            Packet::SetSlotResponse(p)
        }
        (S::Play, D::ServerToClient, 0x17) => {
            let p = play::packet_set_cooldown_response(reader)?;
            Packet::SetCooldownResponse(p)
        }
        (S::Play, D::ServerToClient, 0x18) => {
            let p = play::packet_custom_payload_response(reader)?;
            Packet::CustomPayloadResponse(p)
        }
        (S::Play, D::ServerToClient, 0x19) => {
            let p = play::packet_named_sound_effect_response(reader)?;
            Packet::NamedSoundEffectResponse(p)
        }
        (S::Play, D::ServerToClient, 0x1a) => {
            let p = play::packet_kick_disconnect_response(reader)?;
            Packet::KickDisconnectResponse(p)
        }
        (S::Play, D::ServerToClient, 0x1b) => {
            let p = play::packet_entity_status_response(reader)?;
            Packet::EntityStatusResponse(p)
        }
        (S::Play, D::ServerToClient, 0x1c) => {
            let p = play::packet_explosion_response(reader)?;
            Packet::ExplosionResponse(p)
        }
        (S::Play, D::ServerToClient, 0x1d) => {
            let p = play::packet_unload_chunk_response(reader)?;
            Packet::UnloadChunkResponse(p)
        }
        (S::Play, D::ServerToClient, 0x1e) => {
            let p = play::packet_game_state_change_response(reader)?;
            Packet::GameStateChangeResponse(p)
        }
        (S::Play, D::ServerToClient, 0x1f) => {
            let p = play::packet_open_horse_window_response(reader)?;
            Packet::OpenHorseWindowResponse(p)
        }
        (S::Play, D::ServerToClient, 0x20) => {
            let p = play::packet_initialize_world_border_response(reader)?;
            Packet::InitializeWorldBorderResponse(p)
        }
        (S::Play, D::ServerToClient, 0x21) => {
            let p = play::packet_keep_alive_response(reader)?;
            Packet::KeepAliveResponse(p)
        }
        (S::Play, D::ServerToClient, 0x22) => {
            let p = play::packet_map_chunk_response(reader)?;
            Packet::MapChunkResponse(p)
        }
        (S::Play, D::ServerToClient, 0x23) => {
            let p = play::packet_world_event_response(reader)?;
            Packet::WorldEventResponse(p)
        }
        (S::Play, D::ServerToClient, 0x24) => {
            let p = play::packet_world_particles_response(reader)?;
            Packet::WorldParticlesResponse(p)
        }
        (S::Play, D::ServerToClient, 0x25) => {
            let p = play::packet_update_light_response(reader)?;
            Packet::UpdateLightResponse(p)
        }
        (S::Play, D::ServerToClient, 0x26) => {
            let p = play::packet_login_response(reader)?;
            Packet::LoginResponse(p)
        }
        (S::Play, D::ServerToClient, 0x27) => {
            let p = play::packet_map_response(reader)?;
            Packet::MapResponse(p)
        }
        (S::Play, D::ServerToClient, 0x28) => {
            let p = play::packet_trade_list_response(reader)?;
            Packet::TradeListResponse(p)
        }
        (S::Play, D::ServerToClient, 0x29) => {
            let p = play::packet_rel_entity_move_response(reader)?;
            Packet::RelEntityMoveResponse(p)
        }
        (S::Play, D::ServerToClient, 0x2a) => {
            let p = play::packet_entity_move_look_response(reader)?;
            Packet::EntityMoveLookResponse(p)
        }
        (S::Play, D::ServerToClient, 0x2b) => {
            let p = play::packet_entity_look_response(reader)?;
            Packet::EntityLookResponse(p)
        }
        (S::Play, D::ServerToClient, 0x2c) => {
            let p = play::packet_vehicle_move_response(reader)?;
            Packet::VehicleMoveResponse(p)
        }
        (S::Play, D::ServerToClient, 0x2d) => {
            let p = play::packet_open_book_response(reader)?;
            Packet::OpenBookResponse(p)
        }
        (S::Play, D::ServerToClient, 0x2e) => {
            let p = play::packet_open_window_response(reader)?;
            Packet::OpenWindowResponse(p)
        }
        (S::Play, D::ServerToClient, 0x2f) => {
            let p = play::packet_open_sign_entity_response(reader)?;
            Packet::OpenSignEntityResponse(p)
        }
        (S::Play, D::ServerToClient, 0x30) => {
            let p = play::packet_play_ping_response(reader)?;
            Packet::PlayPingResponse(p)
        }
        (S::Play, D::ServerToClient, 0x31) => {
            let p = play::packet_craft_recipe_response(reader)?;
            Packet::CraftRecipeResponse(p)
        }
        (S::Play, D::ServerToClient, 0x32) => {
            let p = play::packet_abilities_response(reader)?;
            Packet::AbilitiesResponse(p)
        }
        (S::Play, D::ServerToClient, 0x33) => {
            let p = play::packet_end_combat_event_response(reader)?;
            Packet::EndCombatEventResponse(p)
        }
        (S::Play, D::ServerToClient, 0x34) => {
            let p = play::packet_enter_combat_event_response(reader)?;
            Packet::EnterCombatEventResponse(p)
        }
        (S::Play, D::ServerToClient, 0x35) => {
            let p = play::packet_death_combat_event_response(reader)?;
            Packet::DeathCombatEventResponse(p)
        }
        (S::Play, D::ServerToClient, 0x36) => {
            let p = play::packet_player_info_response(reader)?;
            Packet::PlayerInfoResponse(p)
        }
        (S::Play, D::ServerToClient, 0x37) => {
            let p = play::packet_face_player_response(reader)?;
            Packet::FacePlayerResponse(p)
        }
        (S::Play, D::ServerToClient, 0x38) => {
            let p = play::packet_position_response(reader)?;
            Packet::PositionResponse(p)
        }
        (S::Play, D::ServerToClient, 0x39) => {
            let p = play::packet_unlock_recipes_response(reader)?;
            Packet::UnlockRecipesResponse(p)
        }
        (S::Play, D::ServerToClient, 0x3a) => {
            let p = play::packet_entity_destroy_response(reader)?;
            Packet::EntityDestroyResponse(p)
        }
        (S::Play, D::ServerToClient, 0x3b) => {
            let p = play::packet_remove_entity_effect_response(reader)?;
            Packet::RemoveEntityEffectResponse(p)
        }
        (S::Play, D::ServerToClient, 0x3c) => {
            let p = play::packet_resource_pack_send_response(reader)?;
            Packet::ResourcePackSendResponse(p)
        }
        (S::Play, D::ServerToClient, 0x3d) => {
            let p = play::packet_respawn_response(reader)?;
            Packet::RespawnResponse(p)
        }
        (S::Play, D::ServerToClient, 0x3e) => {
            let p = play::packet_entity_head_rotation_response(reader)?;
            Packet::EntityHeadRotationResponse(p)
        }
        (S::Play, D::ServerToClient, 0x3f) => {
            let p = play::packet_multi_block_change_response(reader)?;
            Packet::MultiBlockChangeResponse(p)
        }
        (S::Play, D::ServerToClient, 0x40) => {
            let p = play::packet_select_advancement_tab_response(reader)?;
            Packet::SelectAdvancementTabResponse(p)
        }
        (S::Play, D::ServerToClient, 0x41) => {
            let p = play::packet_action_bar_response(reader)?;
            Packet::ActionBarResponse(p)
        }
        (S::Play, D::ServerToClient, 0x42) => {
            let p = play::packet_world_border_center_response(reader)?;
            Packet::WorldBorderCenterResponse(p)
        }
        (S::Play, D::ServerToClient, 0x43) => {
            let p = play::packet_world_border_lerp_size_response(reader)?;
            Packet::WorldBorderLerpSizeResponse(p)
        }
        (S::Play, D::ServerToClient, 0x44) => {
            let p = play::packet_world_border_size_response(reader)?;
            Packet::WorldBorderSizeResponse(p)
        }
        (S::Play, D::ServerToClient, 0x45) => {
            let p = play::packet_world_border_warning_delay_response(reader)?;
            Packet::WorldBorderWarningDelayResponse(p)
        }
        (S::Play, D::ServerToClient, 0x46) => {
            let p = play::packet_world_border_warning_reach_response(reader)?;
            Packet::WorldBorderWarningReachResponse(p)
        }
        (S::Play, D::ServerToClient, 0x47) => {
            let p = play::packet_camera_response(reader)?;
            Packet::CameraResponse(p)
        }
        (S::Play, D::ServerToClient, 0x48) => {
            let p = play::packet_held_item_slot_response(reader)?;
            Packet::HeldItemSlotResponse(p)
        }
        (S::Play, D::ServerToClient, 0x49) => {
            let p = play::packet_update_view_position_response(reader)?;
            Packet::UpdateViewPositionResponse(p)
        }
        (S::Play, D::ServerToClient, 0x4a) => {
            let p = play::packet_update_view_distance_response(reader)?;
            Packet::UpdateViewDistanceResponse(p)
        }
        (S::Play, D::ServerToClient, 0x4b) => {
            let p = play::packet_spawn_position_response(reader)?;
            Packet::SpawnPositionResponse(p)
        }
        (S::Play, D::ServerToClient, 0x4c) => {
            let p = play::packet_scoreboard_display_objective_response(reader)?;
            Packet::ScoreboardDisplayObjectiveResponse(p)
        }
        (S::Play, D::ServerToClient, 0x4d) => {
            let p = play::packet_entity_metadata_response(reader)?;
            Packet::EntityMetadataResponse(p)
        }
        (S::Play, D::ServerToClient, 0x4e) => {
            let p = play::packet_attach_entity_response(reader)?;
            Packet::AttachEntityResponse(p)
        }
        (S::Play, D::ServerToClient, 0x4f) => {
            let p = play::packet_entity_velocity_response(reader)?;
            Packet::EntityVelocityResponse(p)
        }
        (S::Play, D::ServerToClient, 0x50) => {
            let p = play::packet_entity_equipment_response(reader)?;
            Packet::EntityEquipmentResponse(p)
        }
        (S::Play, D::ServerToClient, 0x51) => {
            let p = play::packet_experience_response(reader)?;
            Packet::ExperienceResponse(p)
        }
        (S::Play, D::ServerToClient, 0x52) => {
            let p = play::packet_update_health_response(reader)?;
            Packet::UpdateHealthResponse(p)
        }
        (S::Play, D::ServerToClient, 0x53) => {
            let p = play::packet_scoreboard_objective_response(reader)?;
            Packet::ScoreboardObjectiveResponse(p)
        }
        (S::Play, D::ServerToClient, 0x54) => {
            let p = play::packet_set_passengers_response(reader)?;
            Packet::SetPassengersResponse(p)
        }
        (S::Play, D::ServerToClient, 0x55) => {
            let p = play::packet_teams_response(reader)?;
            Packet::TeamsResponse(p)
        }
        (S::Play, D::ServerToClient, 0x56) => {
            let p = play::packet_scoreboard_score_response(reader)?;
            Packet::ScoreboardScoreResponse(p)
        }
        (S::Play, D::ServerToClient, 0x57) => {
            let p = play::packet_simulation_distance_response(reader)?;
            Packet::SimulationDistanceResponse(p)
        }
        (S::Play, D::ServerToClient, 0x58) => {
            let p = play::packet_set_title_subtitle_response(reader)?;
            Packet::SetTitleSubtitleResponse(p)
        }
        (S::Play, D::ServerToClient, 0x59) => {
            let p = play::packet_update_time_response(reader)?;
            Packet::UpdateTimeResponse(p)
        }
        (S::Play, D::ServerToClient, 0x5a) => {
            let p = play::packet_set_title_text_response(reader)?;
            Packet::SetTitleTextResponse(p)
        }
        (S::Play, D::ServerToClient, 0x5b) => {
            let p = play::packet_set_title_time_response(reader)?;
            Packet::SetTitleTimeResponse(p)
        }
        (S::Play, D::ServerToClient, 0x5c) => {
            let p = play::packet_entity_sound_effect_response(reader)?;
            Packet::EntitySoundEffectResponse(p)
        }
        (S::Play, D::ServerToClient, 0x5d) => {
            let p = play::packet_sound_effect_response(reader)?;
            Packet::SoundEffectResponse(p)
        }
        (S::Play, D::ServerToClient, 0x5e) => {
            let p = play::packet_stop_sound_response(reader)?;
            Packet::StopSoundResponse(p)
        }
        (S::Play, D::ServerToClient, 0x5f) => {
            let p = play::packet_playerlist_header_response(reader)?;
            Packet::PlayerlistHeaderResponse(p)
        }
        (S::Play, D::ServerToClient, 0x60) => {
            let p = play::packet_nbt_query_response(reader)?;
            Packet::NbtQueryResponse(p)
        }
        (S::Play, D::ServerToClient, 0x61) => {
            let p = play::packet_collect_response(reader)?;
            Packet::CollectResponse(p)
        }
        (S::Play, D::ServerToClient, 0x62) => {
            let p = play::packet_entity_teleport_response(reader)?;
            Packet::EntityTeleportResponse(p)
        }
        (S::Play, D::ServerToClient, 0x63) => {
            let p = play::packet_advancements_response(reader)?;
            Packet::AdvancementsResponse(p)
        }
        (S::Play, D::ServerToClient, 0x64) => {
            let p = play::packet_entity_update_attributes_response(reader)?;
            Packet::EntityUpdateAttributesResponse(p)
        }
        (S::Play, D::ServerToClient, 0x65) => {
            let p = play::packet_entity_effect_response(reader)?;
            Packet::EntityEffectResponse(p)
        }
        (S::Play, D::ServerToClient, 0x66) => {
            let p = play::packet_declare_recipes_response(reader)?;
            Packet::DeclareRecipesResponse(p)
        }
        (S::Play, D::ServerToClient, 0x67) => {
            let p = play::packet_tags_response(reader)?;
            Packet::TagsResponse(p)
        }
        _ => {
            return Err(anyhow!("unknown packet id={}", id));
        }
    };
    Ok(packet)
}
