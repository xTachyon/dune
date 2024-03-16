use anyhow::Result;
use std::io::{Result as IoResult, Write};
use crate::protocol::{de::MD, varint::{read_varint, write_varint}};

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
        fn deserialize(reader: &mut &[u8]) -> Result<LegacyServerListPingRequest> {
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
        fn deserialize(reader: &mut &[u8]) -> Result<PingRequest> {
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
        fn deserialize(reader: &mut &'p [u8]) -> Result<ServerInfoResponse<'p>> {
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
        fn deserialize(reader: &mut &[u8]) -> Result<PingResponse> {
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
        fn deserialize(reader: &mut &'p [u8]) -> Result<LoginStartRequest<'p>> {
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
        fn deserialize(reader: &mut &'p [u8]) -> Result<EncryptionBeginRequest<'p>> {
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
        fn deserialize(reader: &mut &'p [u8]) -> Result<DisconnectResponse<'p>> {
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
        fn deserialize(reader: &mut &'p [u8]) -> Result<EncryptionBeginResponse<'p>> {
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
        fn deserialize(reader: &mut &'p [u8]) -> Result<SuccessResponse<'p>> {
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
