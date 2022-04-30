#![allow(unused_imports)]
#![allow(unused_mut)]
pub mod handshaking {
    use crate::protocol::de::MinecraftDeserialize;
    use crate::protocol::de::Reader;
    use crate::protocol::varint::read_varint;
    use anyhow::Result;

    #[derive(Debug)]
    pub struct SetProtocolRequest<'p> {
        pub protocol_version: i32,
        pub server_host: &'p str,
        pub server_port: u16,
        pub next_state: i32,
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct LegacyServerListPingRequest {
        pub payload: u8,
    }
    pub(super) fn packet_legacy_server_list_ping_request(
        mut reader: &mut Reader,
    ) -> Result<LegacyServerListPingRequest> {
        let payload = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = LegacyServerListPingRequest { payload };
        Ok(result)
    }
}
pub mod status {
    use crate::protocol::de::MinecraftDeserialize;
    use crate::protocol::de::Reader;
    use crate::protocol::varint::read_varint;
    use anyhow::Result;

    #[derive(Debug)]
    pub struct PingStartRequest {}
    pub(super) fn packet_ping_start_request(mut _reader: &mut Reader) -> Result<PingStartRequest> {
        let result = PingStartRequest {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct PingRequest {
        pub time: i64,
    }
    pub(super) fn packet_ping_request(mut reader: &mut Reader) -> Result<PingRequest> {
        let time = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = PingRequest { time };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ServerInfoResponse<'p> {
        pub response: &'p str,
    }
    pub(super) fn packet_server_info_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<ServerInfoResponse<'p>> {
        let response = reader.read_range()?;
        let response = reader.get_str_from(response)?;

        let result = ServerInfoResponse { response };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct PingResponse {
        pub time: i64,
    }
    pub(super) fn packet_ping_response(mut reader: &mut Reader) -> Result<PingResponse> {
        let time = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = PingResponse { time };
        Ok(result)
    }
}
pub mod login {
    use crate::protocol::de::MinecraftDeserialize;
    use crate::protocol::de::Reader;
    use crate::protocol::varint::read_varint;
    use anyhow::Result;

    #[derive(Debug)]
    pub struct LoginStartRequest<'p> {
        pub username: &'p str,
    }
    pub(super) fn packet_login_start_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<LoginStartRequest<'p>> {
        let username = reader.read_range()?;
        let username = reader.get_str_from(username)?;

        let result = LoginStartRequest { username };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EncryptionBeginRequest<'p> {
        pub shared_secret: &'p [u8],
        pub verify_token: &'p [u8],
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct LoginPluginResponse {
        pub message_id: i32,
    }
    pub(super) fn packet_login_plugin_response(
        mut reader: &mut Reader,
    ) -> Result<LoginPluginResponse> {
        let message_id = read_varint(&mut reader)?;

        let result = LoginPluginResponse { message_id };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct DisconnectResponse<'p> {
        pub reason: &'p str,
    }
    pub(super) fn packet_disconnect_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<DisconnectResponse<'p>> {
        let reason = reader.read_range()?;
        let reason = reader.get_str_from(reason)?;

        let result = DisconnectResponse { reason };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EncryptionBeginResponse<'p> {
        pub server_id: &'p str,
        pub public_key: &'p [u8],
        pub verify_token: &'p [u8],
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SuccessResponse<'p> {
        pub uuid: u128,
        pub username: &'p str,
    }
    pub(super) fn packet_success_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<SuccessResponse<'p>> {
        let uuid = MinecraftDeserialize::deserialize(&mut reader)?;
        let username = reader.read_range()?;
        let username = reader.get_str_from(username)?;

        let result = SuccessResponse { uuid, username };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct CompressResponse {
        pub threshold: i32,
    }
    pub(super) fn packet_compress_response(mut reader: &mut Reader) -> Result<CompressResponse> {
        let threshold = read_varint(&mut reader)?;

        let result = CompressResponse { threshold };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct LoginPluginRequest<'p> {
        pub message_id: i32,
        pub channel: &'p str,
    }
    pub(super) fn packet_login_plugin_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<LoginPluginRequest<'p>> {
        let message_id = read_varint(&mut reader)?;
        let channel = reader.read_range()?;
        let channel = reader.get_str_from(channel)?;

        let result = LoginPluginRequest {
            message_id,
            channel,
        };
        Ok(result)
    }
}
pub mod play {
    use crate::protocol::de::MinecraftDeserialize;
    use crate::protocol::de::Reader;
    use crate::protocol::varint::read_varint;
    use anyhow::Result;

    #[derive(Debug)]
    pub struct TeleportConfirmRequest {
        pub teleport_id: i32,
    }
    pub(super) fn packet_teleport_confirm_request(
        mut reader: &mut Reader,
    ) -> Result<TeleportConfirmRequest> {
        let teleport_id = read_varint(&mut reader)?;

        let result = TeleportConfirmRequest { teleport_id };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct QueryBlockNbtRequest {
        pub transaction_id: i32,
        pub location: crate::protocol::de::Position,
    }
    pub(super) fn packet_query_block_nbt_request(
        mut reader: &mut Reader,
    ) -> Result<QueryBlockNbtRequest> {
        let transaction_id = read_varint(&mut reader)?;
        let location = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = QueryBlockNbtRequest {
            transaction_id,
            location,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SetDifficultyRequest {
        pub new_difficulty: u8,
    }
    pub(super) fn packet_set_difficulty_request(
        mut reader: &mut Reader,
    ) -> Result<SetDifficultyRequest> {
        let new_difficulty = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = SetDifficultyRequest { new_difficulty };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EditBookRequest {}
    pub(super) fn packet_edit_book_request(mut _reader: &mut Reader) -> Result<EditBookRequest> {
        let result = EditBookRequest {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct QueryEntityNbtRequest {
        pub transaction_id: i32,
        pub entity_id: i32,
    }
    pub(super) fn packet_query_entity_nbt_request(
        mut reader: &mut Reader,
    ) -> Result<QueryEntityNbtRequest> {
        let transaction_id = read_varint(&mut reader)?;
        let entity_id = read_varint(&mut reader)?;

        let result = QueryEntityNbtRequest {
            transaction_id,
            entity_id,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct PickItemRequest {
        pub slot: i32,
    }
    pub(super) fn packet_pick_item_request(mut reader: &mut Reader) -> Result<PickItemRequest> {
        let slot = read_varint(&mut reader)?;

        let result = PickItemRequest { slot };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct NameItemRequest<'p> {
        pub name: &'p str,
    }
    pub(super) fn packet_name_item_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<NameItemRequest<'p>> {
        let name = reader.read_range()?;
        let name = reader.get_str_from(name)?;

        let result = NameItemRequest { name };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SelectTradeRequest {
        pub slot: i32,
    }
    pub(super) fn packet_select_trade_request(
        mut reader: &mut Reader,
    ) -> Result<SelectTradeRequest> {
        let slot = read_varint(&mut reader)?;

        let result = SelectTradeRequest { slot };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SetBeaconEffectRequest {
        pub primary_effect: i32,
        pub secondary_effect: i32,
    }
    pub(super) fn packet_set_beacon_effect_request(
        mut reader: &mut Reader,
    ) -> Result<SetBeaconEffectRequest> {
        let primary_effect = read_varint(&mut reader)?;
        let secondary_effect = read_varint(&mut reader)?;

        let result = SetBeaconEffectRequest {
            primary_effect,
            secondary_effect,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UpdateCommandBlockRequest<'p> {
        pub location: crate::protocol::de::Position,
        pub command: &'p str,
        pub mode: i32,
        pub flags: u8,
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UpdateCommandBlockMinecartRequest<'p> {
        pub entity_id: i32,
        pub command: &'p str,
        pub track_output: bool,
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct TabCompleteRequest<'p> {
        pub transaction_id: i32,
        pub text: &'p str,
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ChatRequest<'p> {
        pub message: &'p str,
    }
    pub(super) fn packet_chat_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<ChatRequest<'p>> {
        let message = reader.read_range()?;
        let message = reader.get_str_from(message)?;

        let result = ChatRequest { message };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ClientCommandRequest {
        pub action_id: i32,
    }
    pub(super) fn packet_client_command_request(
        mut reader: &mut Reader,
    ) -> Result<ClientCommandRequest> {
        let action_id = read_varint(&mut reader)?;

        let result = ClientCommandRequest { action_id };
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EnchantItemRequest {
        pub window_id: i8,
        pub enchantment: i8,
    }
    pub(super) fn packet_enchant_item_request(
        mut reader: &mut Reader,
    ) -> Result<EnchantItemRequest> {
        let window_id = MinecraftDeserialize::deserialize(&mut reader)?;
        let enchantment = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = EnchantItemRequest {
            window_id,
            enchantment,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct WindowClickRequest {}
    pub(super) fn packet_window_click_request(
        mut _reader: &mut Reader,
    ) -> Result<WindowClickRequest> {
        let result = WindowClickRequest {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct CloseWindowRequest {
        pub window_id: u8,
    }
    pub(super) fn packet_close_window_request(
        mut reader: &mut Reader,
    ) -> Result<CloseWindowRequest> {
        let window_id = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = CloseWindowRequest { window_id };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct CustomPayloadRequest<'p> {
        pub channel: &'p str,
    }
    pub(super) fn packet_custom_payload_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<CustomPayloadRequest<'p>> {
        let channel = reader.read_range()?;
        let channel = reader.get_str_from(channel)?;

        let result = CustomPayloadRequest { channel };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UseEntityRequest {}
    pub(super) fn packet_use_entity_request(mut _reader: &mut Reader) -> Result<UseEntityRequest> {
        let result = UseEntityRequest {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct GenerateStructureRequest {
        pub location: crate::protocol::de::Position,
        pub levels: i32,
        pub keep_jigsaws: bool,
    }
    pub(super) fn packet_generate_structure_request(
        mut reader: &mut Reader,
    ) -> Result<GenerateStructureRequest> {
        let location = MinecraftDeserialize::deserialize(&mut reader)?;
        let levels = read_varint(&mut reader)?;
        let keep_jigsaws = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = GenerateStructureRequest {
            location,
            levels,
            keep_jigsaws,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct KeepAliveRequest {
        pub keep_alive_id: i64,
    }
    pub(super) fn packet_keep_alive_request(mut reader: &mut Reader) -> Result<KeepAliveRequest> {
        let keep_alive_id = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = KeepAliveRequest { keep_alive_id };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct LockDifficultyRequest {
        pub locked: bool,
    }
    pub(super) fn packet_lock_difficulty_request(
        mut reader: &mut Reader,
    ) -> Result<LockDifficultyRequest> {
        let locked = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = LockDifficultyRequest { locked };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct PositionRequest {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub on_ground: bool,
    }
    pub(super) fn packet_position_request(mut reader: &mut Reader) -> Result<PositionRequest> {
        let x = MinecraftDeserialize::deserialize(&mut reader)?;
        let y = MinecraftDeserialize::deserialize(&mut reader)?;
        let z = MinecraftDeserialize::deserialize(&mut reader)?;
        let on_ground = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = PositionRequest { x, y, z, on_ground };
        Ok(result)
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
    pub(super) fn packet_position_look_request(
        mut reader: &mut Reader,
    ) -> Result<PositionLookRequest> {
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct LookRequest {
        pub yaw: f32,
        pub pitch: f32,
        pub on_ground: bool,
    }
    pub(super) fn packet_look_request(mut reader: &mut Reader) -> Result<LookRequest> {
        let yaw = MinecraftDeserialize::deserialize(&mut reader)?;
        let pitch = MinecraftDeserialize::deserialize(&mut reader)?;
        let on_ground = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = LookRequest {
            yaw,
            pitch,
            on_ground,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct FlyingRequest {
        pub on_ground: bool,
    }
    pub(super) fn packet_flying_request(mut reader: &mut Reader) -> Result<FlyingRequest> {
        let on_ground = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = FlyingRequest { on_ground };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct VehicleMoveRequest {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub yaw: f32,
        pub pitch: f32,
    }
    pub(super) fn packet_vehicle_move_request(
        mut reader: &mut Reader,
    ) -> Result<VehicleMoveRequest> {
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SteerBoatRequest {
        pub left_paddle: bool,
        pub right_paddle: bool,
    }
    pub(super) fn packet_steer_boat_request(mut reader: &mut Reader) -> Result<SteerBoatRequest> {
        let left_paddle = MinecraftDeserialize::deserialize(&mut reader)?;
        let right_paddle = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = SteerBoatRequest {
            left_paddle,
            right_paddle,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct CraftRecipeRequest<'p> {
        pub window_id: i8,
        pub recipe: &'p str,
        pub make_all: bool,
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct AbilitiesRequest {
        pub flags: i8,
    }
    pub(super) fn packet_abilities_request(mut reader: &mut Reader) -> Result<AbilitiesRequest> {
        let flags = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = AbilitiesRequest { flags };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct BlockDigRequest {
        pub status: i8,
        pub location: crate::protocol::de::Position,
        pub face: i8,
    }
    pub(super) fn packet_block_dig_request(mut reader: &mut Reader) -> Result<BlockDigRequest> {
        let status = MinecraftDeserialize::deserialize(&mut reader)?;
        let location = MinecraftDeserialize::deserialize(&mut reader)?;
        let face = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = BlockDigRequest {
            status,
            location,
            face,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityActionRequest {
        pub entity_id: i32,
        pub action_id: i32,
        pub jump_boost: i32,
    }
    pub(super) fn packet_entity_action_request(
        mut reader: &mut Reader,
    ) -> Result<EntityActionRequest> {
        let entity_id = read_varint(&mut reader)?;
        let action_id = read_varint(&mut reader)?;
        let jump_boost = read_varint(&mut reader)?;

        let result = EntityActionRequest {
            entity_id,
            action_id,
            jump_boost,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SteerVehicleRequest {
        pub sideways: f32,
        pub forward: f32,
        pub jump: u8,
    }
    pub(super) fn packet_steer_vehicle_request(
        mut reader: &mut Reader,
    ) -> Result<SteerVehicleRequest> {
        let sideways = MinecraftDeserialize::deserialize(&mut reader)?;
        let forward = MinecraftDeserialize::deserialize(&mut reader)?;
        let jump = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = SteerVehicleRequest {
            sideways,
            forward,
            jump,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct DisplayedRecipeRequest<'p> {
        pub recipe_id: &'p str,
    }
    pub(super) fn packet_displayed_recipe_request<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<DisplayedRecipeRequest<'p>> {
        let recipe_id = reader.read_range()?;
        let recipe_id = reader.get_str_from(recipe_id)?;

        let result = DisplayedRecipeRequest { recipe_id };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct RecipeBookRequest {
        pub book_id: i32,
        pub book_open: bool,
        pub filter_active: bool,
    }
    pub(super) fn packet_recipe_book_request(mut reader: &mut Reader) -> Result<RecipeBookRequest> {
        let book_id = read_varint(&mut reader)?;
        let book_open = MinecraftDeserialize::deserialize(&mut reader)?;
        let filter_active = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = RecipeBookRequest {
            book_id,
            book_open,
            filter_active,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ResourcePackReceiveRequest {
        pub result: i32,
    }
    pub(super) fn packet_resource_pack_receive_request(
        mut reader: &mut Reader,
    ) -> Result<ResourcePackReceiveRequest> {
        let result = read_varint(&mut reader)?;

        let result = ResourcePackReceiveRequest { result };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct HeldItemSlotRequest {
        pub slot_id: i16,
    }
    pub(super) fn packet_held_item_slot_request(
        mut reader: &mut Reader,
    ) -> Result<HeldItemSlotRequest> {
        let slot_id = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = HeldItemSlotRequest { slot_id };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SetCreativeSlotRequest {}
    pub(super) fn packet_set_creative_slot_request(
        mut _reader: &mut Reader,
    ) -> Result<SetCreativeSlotRequest> {
        let result = SetCreativeSlotRequest {};
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ArmAnimationRequest {
        pub hand: i32,
    }
    pub(super) fn packet_arm_animation_request(
        mut reader: &mut Reader,
    ) -> Result<ArmAnimationRequest> {
        let hand = read_varint(&mut reader)?;

        let result = ArmAnimationRequest { hand };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SpectateRequest {
        pub target: u128,
    }
    pub(super) fn packet_spectate_request(mut reader: &mut Reader) -> Result<SpectateRequest> {
        let target = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = SpectateRequest { target };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct BlockPlaceRequest {
        pub hand: i32,
        pub location: crate::protocol::de::Position,
        pub direction: i32,
        pub cursor_x: f32,
        pub cursor_y: f32,
        pub cursor_z: f32,
        pub inside_block: bool,
    }
    pub(super) fn packet_block_place_request(mut reader: &mut Reader) -> Result<BlockPlaceRequest> {
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UseItemRequest {
        pub hand: i32,
    }
    pub(super) fn packet_use_item_request(mut reader: &mut Reader) -> Result<UseItemRequest> {
        let hand = read_varint(&mut reader)?;

        let result = UseItemRequest { hand };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct AdvancementTabRequest {}
    pub(super) fn packet_advancement_tab_request(
        mut _reader: &mut Reader,
    ) -> Result<AdvancementTabRequest> {
        let result = AdvancementTabRequest {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct PongRequest {
        pub id: i32,
    }
    pub(super) fn packet_pong_request(mut reader: &mut Reader) -> Result<PongRequest> {
        let id = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = PongRequest { id };
        Ok(result)
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
    pub(super) fn packet_spawn_entity_response(
        mut reader: &mut Reader,
    ) -> Result<SpawnEntityResponse> {
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SpawnEntityExperienceOrbResponse {
        pub entity_id: i32,
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub count: i16,
    }
    pub(super) fn packet_spawn_entity_experience_orb_response(
        mut reader: &mut Reader,
    ) -> Result<SpawnEntityExperienceOrbResponse> {
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
        };
        Ok(result)
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
    pub(super) fn packet_spawn_entity_living_response(
        mut reader: &mut Reader,
    ) -> Result<SpawnEntityLivingResponse> {
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SpawnEntityPaintingResponse {
        pub entity_id: i32,
        pub entity_uuid: u128,
        pub title: i32,
        pub location: crate::protocol::de::Position,
        pub direction: u8,
    }
    pub(super) fn packet_spawn_entity_painting_response(
        mut reader: &mut Reader,
    ) -> Result<SpawnEntityPaintingResponse> {
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
        };
        Ok(result)
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
    pub(super) fn packet_named_entity_spawn_response(
        mut reader: &mut Reader,
    ) -> Result<NamedEntitySpawnResponse> {
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct AnimationResponse {
        pub entity_id: i32,
        pub animation: u8,
    }
    pub(super) fn packet_animation_response(mut reader: &mut Reader) -> Result<AnimationResponse> {
        let entity_id = read_varint(&mut reader)?;
        let animation = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = AnimationResponse {
            entity_id,
            animation,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct StatisticsResponse {}
    pub(super) fn packet_statistics_response(
        mut _reader: &mut Reader,
    ) -> Result<StatisticsResponse> {
        let result = StatisticsResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct AdvancementsResponse {}
    pub(super) fn packet_advancements_response(
        mut _reader: &mut Reader,
    ) -> Result<AdvancementsResponse> {
        let result = AdvancementsResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct BlockBreakAnimationResponse {
        pub entity_id: i32,
        pub location: crate::protocol::de::Position,
        pub destroy_stage: i8,
    }
    pub(super) fn packet_block_break_animation_response(
        mut reader: &mut Reader,
    ) -> Result<BlockBreakAnimationResponse> {
        let entity_id = read_varint(&mut reader)?;
        let location = MinecraftDeserialize::deserialize(&mut reader)?;
        let destroy_stage = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = BlockBreakAnimationResponse {
            entity_id,
            location,
            destroy_stage,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct TileEntityDataResponse {}
    pub(super) fn packet_tile_entity_data_response(
        mut _reader: &mut Reader,
    ) -> Result<TileEntityDataResponse> {
        let result = TileEntityDataResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct BlockActionResponse {
        pub location: crate::protocol::de::Position,
        pub byte1: u8,
        pub byte2: u8,
        pub block_id: i32,
    }
    pub(super) fn packet_block_action_response(
        mut reader: &mut Reader,
    ) -> Result<BlockActionResponse> {
        let location = MinecraftDeserialize::deserialize(&mut reader)?;
        let byte1 = MinecraftDeserialize::deserialize(&mut reader)?;
        let byte2 = MinecraftDeserialize::deserialize(&mut reader)?;
        let block_id = read_varint(&mut reader)?;

        let result = BlockActionResponse {
            location,
            byte1,
            byte2,
            block_id,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct BlockChangeResponse {
        pub location: crate::protocol::de::Position,
        pub type_: i32,
    }
    pub(super) fn packet_block_change_response(
        mut reader: &mut Reader,
    ) -> Result<BlockChangeResponse> {
        let location = MinecraftDeserialize::deserialize(&mut reader)?;
        let type_ = read_varint(&mut reader)?;

        let result = BlockChangeResponse { location, type_ };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct BossBarResponse {}
    pub(super) fn packet_boss_bar_response(mut _reader: &mut Reader) -> Result<BossBarResponse> {
        let result = BossBarResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct DifficultyResponse {
        pub difficulty: u8,
        pub difficulty_locked: bool,
    }
    pub(super) fn packet_difficulty_response(
        mut reader: &mut Reader,
    ) -> Result<DifficultyResponse> {
        let difficulty = MinecraftDeserialize::deserialize(&mut reader)?;
        let difficulty_locked = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = DifficultyResponse {
            difficulty,
            difficulty_locked,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct TabCompleteResponse {}
    pub(super) fn packet_tab_complete_response(
        mut _reader: &mut Reader,
    ) -> Result<TabCompleteResponse> {
        let result = TabCompleteResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct DeclareCommandsResponse {}
    pub(super) fn packet_declare_commands_response(
        mut _reader: &mut Reader,
    ) -> Result<DeclareCommandsResponse> {
        let result = DeclareCommandsResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct FacePlayerResponse {}
    pub(super) fn packet_face_player_response(
        mut _reader: &mut Reader,
    ) -> Result<FacePlayerResponse> {
        let result = FacePlayerResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct NbtQueryResponse {}
    pub(super) fn packet_nbt_query_response(mut _reader: &mut Reader) -> Result<NbtQueryResponse> {
        let result = NbtQueryResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ChatResponse<'p> {
        pub message: &'p str,
        pub position: i8,
        pub sender: u128,
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct MultiBlockChangeResponse {}
    pub(super) fn packet_multi_block_change_response(
        mut _reader: &mut Reader,
    ) -> Result<MultiBlockChangeResponse> {
        let result = MultiBlockChangeResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct CloseWindowResponse {
        pub window_id: u8,
    }
    pub(super) fn packet_close_window_response(
        mut reader: &mut Reader,
    ) -> Result<CloseWindowResponse> {
        let window_id = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = CloseWindowResponse { window_id };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct OpenWindowResponse<'p> {
        pub window_id: i32,
        pub inventory_type: i32,
        pub window_title: &'p str,
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct WindowItemsResponse {}
    pub(super) fn packet_window_items_response(
        mut _reader: &mut Reader,
    ) -> Result<WindowItemsResponse> {
        let result = WindowItemsResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct CraftProgressBarResponse {
        pub window_id: u8,
        pub property: i16,
        pub value: i16,
    }
    pub(super) fn packet_craft_progress_bar_response(
        mut reader: &mut Reader,
    ) -> Result<CraftProgressBarResponse> {
        let window_id = MinecraftDeserialize::deserialize(&mut reader)?;
        let property = MinecraftDeserialize::deserialize(&mut reader)?;
        let value = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = CraftProgressBarResponse {
            window_id,
            property,
            value,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SetSlotResponse {}
    pub(super) fn packet_set_slot_response(mut _reader: &mut Reader) -> Result<SetSlotResponse> {
        let result = SetSlotResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SetCooldownResponse {
        pub item_id: i32,
        pub cooldown_ticks: i32,
    }
    pub(super) fn packet_set_cooldown_response(
        mut reader: &mut Reader,
    ) -> Result<SetCooldownResponse> {
        let item_id = read_varint(&mut reader)?;
        let cooldown_ticks = read_varint(&mut reader)?;

        let result = SetCooldownResponse {
            item_id,
            cooldown_ticks,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct CustomPayloadResponse<'p> {
        pub channel: &'p str,
    }
    pub(super) fn packet_custom_payload_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<CustomPayloadResponse<'p>> {
        let channel = reader.read_range()?;
        let channel = reader.get_str_from(channel)?;

        let result = CustomPayloadResponse { channel };
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct KickDisconnectResponse<'p> {
        pub reason: &'p str,
    }
    pub(super) fn packet_kick_disconnect_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<KickDisconnectResponse<'p>> {
        let reason = reader.read_range()?;
        let reason = reader.get_str_from(reason)?;

        let result = KickDisconnectResponse { reason };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityStatusResponse {
        pub entity_id: i32,
        pub entity_status: i8,
    }
    pub(super) fn packet_entity_status_response(
        mut reader: &mut Reader,
    ) -> Result<EntityStatusResponse> {
        let entity_id = MinecraftDeserialize::deserialize(&mut reader)?;
        let entity_status = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = EntityStatusResponse {
            entity_id,
            entity_status,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ExplosionResponse {}
    pub(super) fn packet_explosion_response(mut _reader: &mut Reader) -> Result<ExplosionResponse> {
        let result = ExplosionResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UnloadChunkResponse {
        pub chunk_x: i32,
        pub chunk_z: i32,
    }
    pub(super) fn packet_unload_chunk_response(
        mut reader: &mut Reader,
    ) -> Result<UnloadChunkResponse> {
        let chunk_x = MinecraftDeserialize::deserialize(&mut reader)?;
        let chunk_z = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = UnloadChunkResponse { chunk_x, chunk_z };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct GameStateChangeResponse {
        pub reason: u8,
        pub game_mode: f32,
    }
    pub(super) fn packet_game_state_change_response(
        mut reader: &mut Reader,
    ) -> Result<GameStateChangeResponse> {
        let reason = MinecraftDeserialize::deserialize(&mut reader)?;
        let game_mode = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = GameStateChangeResponse { reason, game_mode };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct OpenHorseWindowResponse {
        pub window_id: u8,
        pub nb_slots: i32,
        pub entity_id: i32,
    }
    pub(super) fn packet_open_horse_window_response(
        mut reader: &mut Reader,
    ) -> Result<OpenHorseWindowResponse> {
        let window_id = MinecraftDeserialize::deserialize(&mut reader)?;
        let nb_slots = read_varint(&mut reader)?;
        let entity_id = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = OpenHorseWindowResponse {
            window_id,
            nb_slots,
            entity_id,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct KeepAliveResponse {
        pub keep_alive_id: i64,
    }
    pub(super) fn packet_keep_alive_response(mut reader: &mut Reader) -> Result<KeepAliveResponse> {
        let keep_alive_id = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = KeepAliveResponse { keep_alive_id };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct MapChunkResponse {}
    pub(super) fn packet_map_chunk_response(mut _reader: &mut Reader) -> Result<MapChunkResponse> {
        let result = MapChunkResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct WorldEventResponse {
        pub effect_id: i32,
        pub location: crate::protocol::de::Position,
        pub data: i32,
        pub global: bool,
    }
    pub(super) fn packet_world_event_response(
        mut reader: &mut Reader,
    ) -> Result<WorldEventResponse> {
        let effect_id = MinecraftDeserialize::deserialize(&mut reader)?;
        let location = MinecraftDeserialize::deserialize(&mut reader)?;
        let data = MinecraftDeserialize::deserialize(&mut reader)?;
        let global = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = WorldEventResponse {
            effect_id,
            location,
            data,
            global,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct WorldParticlesResponse {}
    pub(super) fn packet_world_particles_response(
        mut _reader: &mut Reader,
    ) -> Result<WorldParticlesResponse> {
        let result = WorldParticlesResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UpdateLightResponse {}
    pub(super) fn packet_update_light_response(
        mut _reader: &mut Reader,
    ) -> Result<UpdateLightResponse> {
        let result = UpdateLightResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct LoginResponse {}
    pub(super) fn packet_login_response(mut _reader: &mut Reader) -> Result<LoginResponse> {
        let result = LoginResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct MapResponse {}
    pub(super) fn packet_map_response(mut _reader: &mut Reader) -> Result<MapResponse> {
        let result = MapResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct TradeListResponse {}
    pub(super) fn packet_trade_list_response(
        mut _reader: &mut Reader,
    ) -> Result<TradeListResponse> {
        let result = TradeListResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct RelEntityMoveResponse {
        pub entity_id: i32,
        pub d_x: i16,
        pub d_y: i16,
        pub d_z: i16,
        pub on_ground: bool,
    }
    pub(super) fn packet_rel_entity_move_response(
        mut reader: &mut Reader,
    ) -> Result<RelEntityMoveResponse> {
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
        };
        Ok(result)
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
    pub(super) fn packet_entity_move_look_response(
        mut reader: &mut Reader,
    ) -> Result<EntityMoveLookResponse> {
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityLookResponse {
        pub entity_id: i32,
        pub yaw: i8,
        pub pitch: i8,
        pub on_ground: bool,
    }
    pub(super) fn packet_entity_look_response(
        mut reader: &mut Reader,
    ) -> Result<EntityLookResponse> {
        let entity_id = read_varint(&mut reader)?;
        let yaw = MinecraftDeserialize::deserialize(&mut reader)?;
        let pitch = MinecraftDeserialize::deserialize(&mut reader)?;
        let on_ground = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = EntityLookResponse {
            entity_id,
            yaw,
            pitch,
            on_ground,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct VehicleMoveResponse {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub yaw: f32,
        pub pitch: f32,
    }
    pub(super) fn packet_vehicle_move_response(
        mut reader: &mut Reader,
    ) -> Result<VehicleMoveResponse> {
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct OpenBookResponse {
        pub hand: i32,
    }
    pub(super) fn packet_open_book_response(mut reader: &mut Reader) -> Result<OpenBookResponse> {
        let hand = read_varint(&mut reader)?;

        let result = OpenBookResponse { hand };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct OpenSignEntityResponse {
        pub location: crate::protocol::de::Position,
    }
    pub(super) fn packet_open_sign_entity_response(
        mut reader: &mut Reader,
    ) -> Result<OpenSignEntityResponse> {
        let location = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = OpenSignEntityResponse { location };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct CraftRecipeResponse<'p> {
        pub window_id: i8,
        pub recipe: &'p str,
    }
    pub(super) fn packet_craft_recipe_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<CraftRecipeResponse<'p>> {
        let window_id = MinecraftDeserialize::deserialize(&mut reader)?;
        let recipe = reader.read_range()?;
        let recipe = reader.get_str_from(recipe)?;

        let result = CraftRecipeResponse { window_id, recipe };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct AbilitiesResponse {
        pub flags: i8,
        pub flying_speed: f32,
        pub walking_speed: f32,
    }
    pub(super) fn packet_abilities_response(mut reader: &mut Reader) -> Result<AbilitiesResponse> {
        let flags = MinecraftDeserialize::deserialize(&mut reader)?;
        let flying_speed = MinecraftDeserialize::deserialize(&mut reader)?;
        let walking_speed = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = AbilitiesResponse {
            flags,
            flying_speed,
            walking_speed,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EndCombatEventResponse {
        pub duration: i32,
        pub entity_id: i32,
    }
    pub(super) fn packet_end_combat_event_response(
        mut reader: &mut Reader,
    ) -> Result<EndCombatEventResponse> {
        let duration = read_varint(&mut reader)?;
        let entity_id = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = EndCombatEventResponse {
            duration,
            entity_id,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EnterCombatEventResponse {}
    pub(super) fn packet_enter_combat_event_response(
        mut _reader: &mut Reader,
    ) -> Result<EnterCombatEventResponse> {
        let result = EnterCombatEventResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct DeathCombatEventResponse<'p> {
        pub player_id: i32,
        pub entity_id: i32,
        pub message: &'p str,
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct PlayerInfoResponse {}
    pub(super) fn packet_player_info_response(
        mut _reader: &mut Reader,
    ) -> Result<PlayerInfoResponse> {
        let result = PlayerInfoResponse {};
        Ok(result)
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
    pub(super) fn packet_position_response(mut reader: &mut Reader) -> Result<PositionResponse> {
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UnlockRecipesResponse {}
    pub(super) fn packet_unlock_recipes_response(
        mut _reader: &mut Reader,
    ) -> Result<UnlockRecipesResponse> {
        let result = UnlockRecipesResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityDestroyResponse {}
    pub(super) fn packet_entity_destroy_response(
        mut _reader: &mut Reader,
    ) -> Result<EntityDestroyResponse> {
        let result = EntityDestroyResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct RemoveEntityEffectResponse {
        pub entity_id: i32,
        pub effect_id: i8,
    }
    pub(super) fn packet_remove_entity_effect_response(
        mut reader: &mut Reader,
    ) -> Result<RemoveEntityEffectResponse> {
        let entity_id = read_varint(&mut reader)?;
        let effect_id = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = RemoveEntityEffectResponse {
            entity_id,
            effect_id,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ResourcePackSendResponse {}
    pub(super) fn packet_resource_pack_send_response(
        mut _reader: &mut Reader,
    ) -> Result<ResourcePackSendResponse> {
        let result = ResourcePackSendResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct RespawnResponse {}
    pub(super) fn packet_respawn_response(mut _reader: &mut Reader) -> Result<RespawnResponse> {
        let result = RespawnResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityHeadRotationResponse {
        pub entity_id: i32,
        pub head_yaw: i8,
    }
    pub(super) fn packet_entity_head_rotation_response(
        mut reader: &mut Reader,
    ) -> Result<EntityHeadRotationResponse> {
        let entity_id = read_varint(&mut reader)?;
        let head_yaw = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = EntityHeadRotationResponse {
            entity_id,
            head_yaw,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct CameraResponse {
        pub camera_id: i32,
    }
    pub(super) fn packet_camera_response(mut reader: &mut Reader) -> Result<CameraResponse> {
        let camera_id = read_varint(&mut reader)?;

        let result = CameraResponse { camera_id };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct HeldItemSlotResponse {
        pub slot: i8,
    }
    pub(super) fn packet_held_item_slot_response(
        mut reader: &mut Reader,
    ) -> Result<HeldItemSlotResponse> {
        let slot = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = HeldItemSlotResponse { slot };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UpdateViewPositionResponse {
        pub chunk_x: i32,
        pub chunk_z: i32,
    }
    pub(super) fn packet_update_view_position_response(
        mut reader: &mut Reader,
    ) -> Result<UpdateViewPositionResponse> {
        let chunk_x = read_varint(&mut reader)?;
        let chunk_z = read_varint(&mut reader)?;

        let result = UpdateViewPositionResponse { chunk_x, chunk_z };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UpdateViewDistanceResponse {
        pub view_distance: i32,
    }
    pub(super) fn packet_update_view_distance_response(
        mut reader: &mut Reader,
    ) -> Result<UpdateViewDistanceResponse> {
        let view_distance = read_varint(&mut reader)?;

        let result = UpdateViewDistanceResponse { view_distance };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ScoreboardDisplayObjectiveResponse<'p> {
        pub position: i8,
        pub name: &'p str,
    }
    pub(super) fn packet_scoreboard_display_objective_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<ScoreboardDisplayObjectiveResponse<'p>> {
        let position = MinecraftDeserialize::deserialize(&mut reader)?;
        let name = reader.read_range()?;
        let name = reader.get_str_from(name)?;

        let result = ScoreboardDisplayObjectiveResponse { position, name };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityMetadataResponse {}
    pub(super) fn packet_entity_metadata_response(
        mut _reader: &mut Reader,
    ) -> Result<EntityMetadataResponse> {
        let result = EntityMetadataResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct AttachEntityResponse {
        pub entity_id: i32,
        pub vehicle_id: i32,
    }
    pub(super) fn packet_attach_entity_response(
        mut reader: &mut Reader,
    ) -> Result<AttachEntityResponse> {
        let entity_id = MinecraftDeserialize::deserialize(&mut reader)?;
        let vehicle_id = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = AttachEntityResponse {
            entity_id,
            vehicle_id,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityVelocityResponse {
        pub entity_id: i32,
        pub velocity_x: i16,
        pub velocity_y: i16,
        pub velocity_z: i16,
    }
    pub(super) fn packet_entity_velocity_response(
        mut reader: &mut Reader,
    ) -> Result<EntityVelocityResponse> {
        let entity_id = read_varint(&mut reader)?;
        let velocity_x = MinecraftDeserialize::deserialize(&mut reader)?;
        let velocity_y = MinecraftDeserialize::deserialize(&mut reader)?;
        let velocity_z = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = EntityVelocityResponse {
            entity_id,
            velocity_x,
            velocity_y,
            velocity_z,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityEquipmentResponse {}
    pub(super) fn packet_entity_equipment_response(
        mut _reader: &mut Reader,
    ) -> Result<EntityEquipmentResponse> {
        let result = EntityEquipmentResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ExperienceResponse {
        pub experience_bar: f32,
        pub level: i32,
        pub total_experience: i32,
    }
    pub(super) fn packet_experience_response(
        mut reader: &mut Reader,
    ) -> Result<ExperienceResponse> {
        let experience_bar = MinecraftDeserialize::deserialize(&mut reader)?;
        let level = read_varint(&mut reader)?;
        let total_experience = read_varint(&mut reader)?;

        let result = ExperienceResponse {
            experience_bar,
            level,
            total_experience,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UpdateHealthResponse {
        pub health: f32,
        pub food: i32,
        pub food_saturation: f32,
    }
    pub(super) fn packet_update_health_response(
        mut reader: &mut Reader,
    ) -> Result<UpdateHealthResponse> {
        let health = MinecraftDeserialize::deserialize(&mut reader)?;
        let food = read_varint(&mut reader)?;
        let food_saturation = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = UpdateHealthResponse {
            health,
            food,
            food_saturation,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ScoreboardObjectiveResponse {}
    pub(super) fn packet_scoreboard_objective_response(
        mut _reader: &mut Reader,
    ) -> Result<ScoreboardObjectiveResponse> {
        let result = ScoreboardObjectiveResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SetPassengersResponse {}
    pub(super) fn packet_set_passengers_response(
        mut _reader: &mut Reader,
    ) -> Result<SetPassengersResponse> {
        let result = SetPassengersResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct TeamsResponse {}
    pub(super) fn packet_teams_response(mut _reader: &mut Reader) -> Result<TeamsResponse> {
        let result = TeamsResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ScoreboardScoreResponse {}
    pub(super) fn packet_scoreboard_score_response(
        mut _reader: &mut Reader,
    ) -> Result<ScoreboardScoreResponse> {
        let result = ScoreboardScoreResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SpawnPositionResponse {
        pub location: crate::protocol::de::Position,
        pub angle: f32,
    }
    pub(super) fn packet_spawn_position_response(
        mut reader: &mut Reader,
    ) -> Result<SpawnPositionResponse> {
        let location = MinecraftDeserialize::deserialize(&mut reader)?;
        let angle = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = SpawnPositionResponse { location, angle };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct UpdateTimeResponse {
        pub age: i64,
        pub time: i64,
    }
    pub(super) fn packet_update_time_response(
        mut reader: &mut Reader,
    ) -> Result<UpdateTimeResponse> {
        let age = MinecraftDeserialize::deserialize(&mut reader)?;
        let time = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = UpdateTimeResponse { age, time };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntitySoundEffectResponse {
        pub sound_id: i32,
        pub sound_category: i32,
        pub entity_id: i32,
        pub volume: f32,
        pub pitch: f32,
    }
    pub(super) fn packet_entity_sound_effect_response(
        mut reader: &mut Reader,
    ) -> Result<EntitySoundEffectResponse> {
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct StopSoundResponse {}
    pub(super) fn packet_stop_sound_response(
        mut _reader: &mut Reader,
    ) -> Result<StopSoundResponse> {
        let result = StopSoundResponse {};
        Ok(result)
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
    pub(super) fn packet_sound_effect_response(
        mut reader: &mut Reader,
    ) -> Result<SoundEffectResponse> {
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct PlayerlistHeaderResponse<'p> {
        pub header: &'p str,
        pub footer: &'p str,
    }
    pub(super) fn packet_playerlist_header_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<PlayerlistHeaderResponse<'p>> {
        let header = reader.read_range()?;
        let footer = reader.read_range()?;
        let header = reader.get_str_from(header)?;
        let footer = reader.get_str_from(footer)?;

        let result = PlayerlistHeaderResponse { header, footer };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct CollectResponse {
        pub collected_entity_id: i32,
        pub collector_entity_id: i32,
        pub pickup_item_count: i32,
    }
    pub(super) fn packet_collect_response(mut reader: &mut Reader) -> Result<CollectResponse> {
        let collected_entity_id = read_varint(&mut reader)?;
        let collector_entity_id = read_varint(&mut reader)?;
        let pickup_item_count = read_varint(&mut reader)?;

        let result = CollectResponse {
            collected_entity_id,
            collector_entity_id,
            pickup_item_count,
        };
        Ok(result)
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
    pub(super) fn packet_entity_teleport_response(
        mut reader: &mut Reader,
    ) -> Result<EntityTeleportResponse> {
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityUpdateAttributesResponse {}
    pub(super) fn packet_entity_update_attributes_response(
        mut _reader: &mut Reader,
    ) -> Result<EntityUpdateAttributesResponse> {
        let result = EntityUpdateAttributesResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct EntityEffectResponse {
        pub entity_id: i32,
        pub effect_id: i8,
        pub amplifier: i8,
        pub duration: i32,
        pub hide_particles: i8,
    }
    pub(super) fn packet_entity_effect_response(
        mut reader: &mut Reader,
    ) -> Result<EntityEffectResponse> {
        let entity_id = read_varint(&mut reader)?;
        let effect_id = MinecraftDeserialize::deserialize(&mut reader)?;
        let amplifier = MinecraftDeserialize::deserialize(&mut reader)?;
        let duration = read_varint(&mut reader)?;
        let hide_particles = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = EntityEffectResponse {
            entity_id,
            effect_id,
            amplifier,
            duration,
            hide_particles,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SelectAdvancementTabResponse {}
    pub(super) fn packet_select_advancement_tab_response(
        mut _reader: &mut Reader,
    ) -> Result<SelectAdvancementTabResponse> {
        let result = SelectAdvancementTabResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct DeclareRecipesResponse {}
    pub(super) fn packet_declare_recipes_response(
        mut _reader: &mut Reader,
    ) -> Result<DeclareRecipesResponse> {
        let result = DeclareRecipesResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct TagsResponse {}
    pub(super) fn packet_tags_response(mut _reader: &mut Reader) -> Result<TagsResponse> {
        let result = TagsResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct AcknowledgePlayerDiggingResponse {
        pub location: crate::protocol::de::Position,
        pub block: i32,
        pub status: i32,
        pub successful: bool,
    }
    pub(super) fn packet_acknowledge_player_digging_response(
        mut reader: &mut Reader,
    ) -> Result<AcknowledgePlayerDiggingResponse> {
        let location = MinecraftDeserialize::deserialize(&mut reader)?;
        let block = read_varint(&mut reader)?;
        let status = read_varint(&mut reader)?;
        let successful = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = AcknowledgePlayerDiggingResponse {
            location,
            block,
            status,
            successful,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SculkVibrationSignalResponse {}
    pub(super) fn packet_sculk_vibration_signal_response(
        mut _reader: &mut Reader,
    ) -> Result<SculkVibrationSignalResponse> {
        let result = SculkVibrationSignalResponse {};
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ClearTitlesResponse {
        pub reset: bool,
    }
    pub(super) fn packet_clear_titles_response(
        mut reader: &mut Reader,
    ) -> Result<ClearTitlesResponse> {
        let reset = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = ClearTitlesResponse { reset };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct InitializeWorldBorderResponse {
        pub x: f64,
        pub z: f64,
        pub old_diameter: f64,
        pub new_diameter: f64,
        pub speed: i32,
        pub portal_teleport_boundary: i32,
        pub warning_blocks: i32,
        pub warning_time: i32,
    }
    pub(super) fn packet_initialize_world_border_response(
        mut reader: &mut Reader,
    ) -> Result<InitializeWorldBorderResponse> {
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
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct ActionBarResponse<'p> {
        pub text: &'p str,
    }
    pub(super) fn packet_action_bar_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<ActionBarResponse<'p>> {
        let text = reader.read_range()?;
        let text = reader.get_str_from(text)?;

        let result = ActionBarResponse { text };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct WorldBorderCenterResponse {
        pub x: f64,
        pub z: f64,
    }
    pub(super) fn packet_world_border_center_response(
        mut reader: &mut Reader,
    ) -> Result<WorldBorderCenterResponse> {
        let x = MinecraftDeserialize::deserialize(&mut reader)?;
        let z = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = WorldBorderCenterResponse { x, z };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct WorldBorderLerpSizeResponse {
        pub old_diameter: f64,
        pub new_diameter: f64,
        pub speed: i32,
    }
    pub(super) fn packet_world_border_lerp_size_response(
        mut reader: &mut Reader,
    ) -> Result<WorldBorderLerpSizeResponse> {
        let old_diameter = MinecraftDeserialize::deserialize(&mut reader)?;
        let new_diameter = MinecraftDeserialize::deserialize(&mut reader)?;
        let speed = read_varint(&mut reader)?;

        let result = WorldBorderLerpSizeResponse {
            old_diameter,
            new_diameter,
            speed,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct WorldBorderSizeResponse {
        pub diameter: f64,
    }
    pub(super) fn packet_world_border_size_response(
        mut reader: &mut Reader,
    ) -> Result<WorldBorderSizeResponse> {
        let diameter = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = WorldBorderSizeResponse { diameter };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct WorldBorderWarningDelayResponse {
        pub warning_time: i32,
    }
    pub(super) fn packet_world_border_warning_delay_response(
        mut reader: &mut Reader,
    ) -> Result<WorldBorderWarningDelayResponse> {
        let warning_time = read_varint(&mut reader)?;

        let result = WorldBorderWarningDelayResponse { warning_time };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct WorldBorderWarningReachResponse {
        pub warning_blocks: i32,
    }
    pub(super) fn packet_world_border_warning_reach_response(
        mut reader: &mut Reader,
    ) -> Result<WorldBorderWarningReachResponse> {
        let warning_blocks = read_varint(&mut reader)?;

        let result = WorldBorderWarningReachResponse { warning_blocks };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct PlayPingResponse {
        pub id: i32,
    }
    pub(super) fn packet_play_ping_response(mut reader: &mut Reader) -> Result<PlayPingResponse> {
        let id = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = PlayPingResponse { id };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SetTitleSubtitleResponse<'p> {
        pub text: &'p str,
    }
    pub(super) fn packet_set_title_subtitle_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<SetTitleSubtitleResponse<'p>> {
        let text = reader.read_range()?;
        let text = reader.get_str_from(text)?;

        let result = SetTitleSubtitleResponse { text };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SetTitleTextResponse<'p> {
        pub text: &'p str,
    }
    pub(super) fn packet_set_title_text_response<'p>(
        mut reader: &'p mut Reader<'p>,
    ) -> Result<SetTitleTextResponse<'p>> {
        let text = reader.read_range()?;
        let text = reader.get_str_from(text)?;

        let result = SetTitleTextResponse { text };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SetTitleTimeResponse {
        pub fade_in: i32,
        pub stay: i32,
        pub fade_out: i32,
    }
    pub(super) fn packet_set_title_time_response(
        mut reader: &mut Reader,
    ) -> Result<SetTitleTimeResponse> {
        let fade_in = MinecraftDeserialize::deserialize(&mut reader)?;
        let stay = MinecraftDeserialize::deserialize(&mut reader)?;
        let fade_out = MinecraftDeserialize::deserialize(&mut reader)?;

        let result = SetTitleTimeResponse {
            fade_in,
            stay,
            fade_out,
        };
        Ok(result)
    }
    #[derive(Debug)]
    pub struct SimulationDistanceResponse {
        pub distance: i32,
    }
    pub(super) fn packet_simulation_distance_response(
        mut reader: &mut Reader,
    ) -> Result<SimulationDistanceResponse> {
        let distance = read_varint(&mut reader)?;

        let result = SimulationDistanceResponse { distance };
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
    LegacyServerListPingRequest(handshaking::LegacyServerListPingRequest),
    PingStartRequest(status::PingStartRequest),
    PingRequest(status::PingRequest),
    ServerInfoResponse(status::ServerInfoResponse<'p>),
    PingResponse(status::PingResponse),
    LoginStartRequest(login::LoginStartRequest<'p>),
    EncryptionBeginRequest(login::EncryptionBeginRequest<'p>),
    LoginPluginResponse(login::LoginPluginResponse),
    DisconnectResponse(login::DisconnectResponse<'p>),
    EncryptionBeginResponse(login::EncryptionBeginResponse<'p>),
    SuccessResponse(login::SuccessResponse<'p>),
    CompressResponse(login::CompressResponse),
    LoginPluginRequest(login::LoginPluginRequest<'p>),
    TeleportConfirmRequest(play::TeleportConfirmRequest),
    QueryBlockNbtRequest(play::QueryBlockNbtRequest),
    SetDifficultyRequest(play::SetDifficultyRequest),
    EditBookRequest(play::EditBookRequest),
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
    WindowClickRequest(play::WindowClickRequest),
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
    SetCreativeSlotRequest(play::SetCreativeSlotRequest),
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
    TileEntityDataResponse(play::TileEntityDataResponse),
    BlockActionResponse(play::BlockActionResponse),
    BlockChangeResponse(play::BlockChangeResponse),
    BossBarResponse(play::BossBarResponse),
    DifficultyResponse(play::DifficultyResponse),
    TabCompleteResponse(play::TabCompleteResponse),
    DeclareCommandsResponse(play::DeclareCommandsResponse),
    FacePlayerResponse(play::FacePlayerResponse),
    NbtQueryResponse(play::NbtQueryResponse),
    ChatResponse(play::ChatResponse<'p>),
    MultiBlockChangeResponse(play::MultiBlockChangeResponse),
    CloseWindowResponse(play::CloseWindowResponse),
    OpenWindowResponse(play::OpenWindowResponse<'p>),
    WindowItemsResponse(play::WindowItemsResponse),
    CraftProgressBarResponse(play::CraftProgressBarResponse),
    SetSlotResponse(play::SetSlotResponse),
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
    ResourcePackSendResponse(play::ResourcePackSendResponse),
    RespawnResponse(play::RespawnResponse),
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
    EntityUpdateAttributesResponse(play::EntityUpdateAttributesResponse),
    EntityEffectResponse(play::EntityEffectResponse),
    SelectAdvancementTabResponse(play::SelectAdvancementTabResponse),
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
