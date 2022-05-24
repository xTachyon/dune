pub mod events;
mod game;
mod protocol;

use crate::events::EventSubscriber;
use crate::protocol::de::MinecraftDeserialize;
use crate::protocol::v1_18_2::login::EncryptionBeginRequest;
use crate::protocol::{ConnectionState, Packet, PacketDirection};
use anyhow::Result;
use byteorder::WriteBytesExt;
use cfb8::cipher::AsyncStreamCipher;
use cfb8::cipher::NewCipher;
use polling::{Event, Poller};
use protocol::de::Reader;
use protocol::varint::write_varint;
use rsa::pkcs8::DecodePublicKey;
use rsa::{PaddingScheme, PublicKey, RsaPublicKey};
use serde_derive::Serialize;
use sha1::Digest;
use std::convert::TryFrom;
use std::io::{Cursor, Read, Write};
use std::net::{TcpListener, TcpStream};

type Aes128Cfb8 = cfb8::Cfb8<aes::Aes128>;

struct Encryption {
    enc: Aes128Cfb8,
    dec: Aes128Cfb8,
}

struct Session {
    read_buf: Vec<u8>,
    write_buf: Vec<u8>,
    crypt: Option<Encryption>,
}

impl Session {
    fn new() -> Session {
        Session {
            read_buf: vec![],
            write_buf: vec![],
            crypt: None,
        }
    }
}

impl Session {
    fn write(&mut self, buf: &[u8]) {
        let offset = self.write_buf.len();
        self.write_buf.extend_from_slice(buf);

        if let Some(crypt) = &mut self.crypt {
            crypt.enc.encrypt(&mut self.write_buf[offset..]);
        }
    }

    fn read(&mut self, buf: &[u8]) {
        let offset = self.read_buf.len();
        self.read_buf.extend_from_slice(buf);

        if let Some(crypt) = &mut self.crypt {
            crypt.dec.decrypt(&mut self.read_buf[offset..]);
        }
    }
}

pub struct AuthData {
    pub selected_profile: String,
    pub access_token: String,
}

struct Proxy {
    state: ConnectionState,
    compression: bool,
    start_done: bool,
    auth_data: Option<AuthData>,
    tmp: Vec<u8>,
}

struct OnStartResult {
    skip: bool,
    total_size: usize,
}

struct DiskPacket {
    pub id: u32,
    pub direction: PacketDirection,
    pub data: Vec<u8>, // todo: make it copy free
}

impl DiskPacket {
    fn write<W: Write>(&self, mut writer: W) -> Result<()> {
        let size = 4 + 1 + self.data.len() as u32;
        writer.write_all(&size.to_le_bytes())?;
        writer.write_all(&[self.direction as u8])?;
        writer.write_all(&self.data)?;

        Ok(())
    }

    fn read<R: Read>(mut reader: R) -> Result<DiskPacket> {
        let size: u32 = MinecraftDeserialize::deserialize(&mut reader)?;
        let id: u32 = MinecraftDeserialize::deserialize(&mut reader)?;
        let direction: u8 = MinecraftDeserialize::deserialize(&mut reader)?;
        let direction = PacketDirection::try_from(direction)?;
        let mut data = vec![0; size as usize - 4 - 1];
        reader.read_exact(&mut data)?;

        Ok(DiskPacket {
            id,
            direction,
            data,
        })
    }

    fn has_enough_bytes(buf: &[u8]) -> bool {
        if buf.len() < 4 {
            return false;
        }
        let size = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize;
        size + 4 <= buf.len()
    }
}

impl Proxy {
    fn new(auth_data: AuthData) -> Proxy {
        Proxy {
            state: ConnectionState::Handshaking,
            compression: false,
            start_done: false,
            auth_data: Some(auth_data),
            tmp: vec![],
        }
    }

    fn rsa_crypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
        let public_key = RsaPublicKey::from_public_key_der(key)?;
        let padding = PaddingScheme::new_pkcs1v15_encrypt();

        let res = public_key.encrypt(&mut rand::thread_rng(), padding, data)?;
        Ok(res)
    }

    fn serialize_enc_response(packet: EncryptionBeginRequest) -> Result<Vec<u8>> {
        let mut cursor = Cursor::new(Vec::new());

        // id = 1
        cursor.write_u8(1)?;

        // Shared Secret Length
        let (buf, size) = write_varint(packet.shared_secret.len() as u32);
        cursor.write_all(&buf[..size])?;

        // Shared Secret
        cursor.write_all(packet.shared_secret)?;

        // Verify Token Length
        let (buf, size) = write_varint(packet.verify_token.len() as u32);
        cursor.write_all(&buf[..size])?;

        // Verify Token
        cursor.write_all(packet.verify_token)?;

        let mut result = Vec::new();

        // size
        let (buf, size) = write_varint(cursor.get_ref().len() as u32);
        result.extend_from_slice(&buf[..size]);
        result.extend_from_slice(cursor.get_ref());

        Ok(result)
    }

    fn on_start(
        &mut self,
        src_session: &mut Session,
        offset: usize,
        direction: PacketDirection,
    ) -> Result<Option<OnStartResult>> {
        let src = &src_session.read_buf[offset..];
        let packet_data = match protocol::read_packet_info(src, self.compression, &mut self.tmp)? {
            Some(x) => x,
            None => return Ok(None),
        };
        let total_size = packet_data.total_size;

        let mut reader = Reader::new(packet_data.data);
        let packet =
            protocol::just_deserialize(direction, self.state, packet_data.id, &mut reader)?;

        println!("{:?}", packet);
        let mut skip = false;
        match packet {
            Packet::SetProtocolRequest(x) => match x.next_state {
                1 => {
                    self.start_done = true;
                    self.state = ConnectionState::Status;
                }
                2 => {
                    self.state = ConnectionState::Login;
                }
                _ => unimplemented!(),
            },
            // ---------------------------------------------------
            Packet::SuccessResponse(_) => {
                self.start_done = true;
                self.state = ConnectionState::Play;
            }
            Packet::CompressResponse(x) => {
                self.compression = x.threshold >= 0;
            }
            Packet::EncryptionBeginResponse(packet) => {
                skip = true;
                let aes_key: [u8; 16] = rand::random();

                let hash = {
                    let mut sha1 = sha1::Sha1::new();
                    sha1.update(packet.server_id);
                    sha1.update(aes_key);
                    sha1.update(&packet.public_key);
                    let hash = sha1.finalize();

                    num_bigint::BigInt::from_signed_bytes_be(&hash).to_str_radix(16)
                };

                let mut auth_data = self.auth_data.take().unwrap();
                auth_data.selected_profile.retain(|c| c != '-');

                #[allow(non_snake_case)]
                #[derive(Serialize)]
                struct RequestData {
                    accessToken: String,
                    selectedProfile: String,
                    serverId: String,
                }
                let req = RequestData {
                    accessToken: auth_data.access_token,
                    selectedProfile: auth_data.selected_profile,
                    serverId: hash,
                };
                let req = serde_json::to_string(&req)?;
                let response =
                    ureq::post("https://sessionserver.mojang.com/session/minecraft/join")
                        .set("Content-Type", "application/json; charset=utf-8")
                        .send_string(&req)?;

                // 204 No Content = Ok
                if response.status() != 204 {
                    return Err(anyhow::Error::msg("bad mojang auth"));
                }

                let response = EncryptionBeginRequest {
                    shared_secret: &Proxy::rsa_crypt(packet.public_key, &aes_key)?,
                    verify_token: &Proxy::rsa_crypt(packet.public_key, packet.verify_token)?,
                };
                let buf = Proxy::serialize_enc_response(response)?;
                src_session.write(&buf);

                let enc = Aes128Cfb8::new_from_slices(&aes_key, &aes_key).unwrap();
                let dec = Aes128Cfb8::new_from_slices(&aes_key, &aes_key).unwrap();

                src_session.crypt = Some(Encryption { enc, dec });
            }
            _ => {}
        }

        Ok(Some(OnStartResult { skip, total_size }))
    }

    fn forward(
        &mut self,
        src: &mut Session,
        dest: &mut Session,
        buf: &[u8],
        direction: PacketDirection,
    ) -> Result<()> {
        src.read(buf);

        let mut offset = 0;
        while let Some(result) = self.on_start(src, offset, direction)? {
            if !result.skip {
                dest.write(&src.read_buf[offset..offset + result.total_size]);
            }
            offset += result.total_size;
        }
        src.read_buf.drain(..offset);

        Ok(())
    }

    fn on_recv(
        &mut self,
        buf: &[u8],
        client: &mut Session,
        server: &mut Session,
        direction: PacketDirection,
    ) -> Result<()> {
        match direction {
            PacketDirection::ClientToServer => self.forward(client, server, buf, direction),
            PacketDirection::ServerToClient => self.forward(server, client, buf, direction),
        }
    }
}

struct RunStats {
    read: usize,
    write: usize,
}

fn run(
    mut client_socket: TcpStream,
    mut server_socket: TcpStream,
    auth_data: AuthData,
    stats: &mut RunStats,
) -> Result<()> {
    const CLIENT_KEY: usize = 0;
    const SERVER_KEY: usize = 1;

    let mut client = Session::new();
    let mut server = Session::new();
    let mut proxy = Proxy::new(auth_data);

    let poller = Poller::new()?;

    client_socket.set_nonblocking(true)?;
    server_socket.set_nonblocking(true)?;

    poller.add(&client_socket, Event::readable(CLIENT_KEY))?;
    poller.add(&server_socket, Event::readable(SERVER_KEY))?;

    let mut events = Vec::new();
    let mut buffer = [0; 64 * 1024];
    loop {
        events.clear();
        poller.wait(&mut events, None)?;

        for ev in &events {
            if ev.readable {
                let (read, direction) = if ev.key == CLIENT_KEY {
                    (
                        client_socket.read(&mut buffer)?,
                        PacketDirection::ClientToServer,
                    )
                } else {
                    (
                        server_socket.read(&mut buffer)?,
                        PacketDirection::ServerToClient,
                    )
                };
                // println!("{:?}: {}", direction, read);
                if read == 0 {
                    return Ok(());
                }
                stats.read += read;
                proxy.on_recv(&buffer[..read], &mut client, &mut server, direction)?;
            }
            if ev.writable {
                if ev.key == CLIENT_KEY {
                    let wrote = client_socket.write(&client.write_buf)?;
                    stats.write += wrote;
                    client.write_buf.drain(..wrote);
                } else {
                    let wrote = server_socket.write(&server.write_buf)?;
                    stats.write += wrote;
                    server.write_buf.drain(..wrote);
                }
            }
        }

        poller.modify(
            &client_socket,
            Event {
                key: CLIENT_KEY,
                readable: true,
                writable: !client.write_buf.is_empty(),
            },
        )?;
        poller.modify(
            &server_socket,
            Event {
                key: SERVER_KEY,
                readable: true,
                writable: !server.write_buf.is_empty(),
            },
        )?;
    }
}

pub fn do_things(
    server_address: &str,
    auth_data: AuthData,
    _handler: Box<dyn EventSubscriber + Sync>,
) -> Result<()> {
    let addr = "0.0.0.0:25566";
    let (client, client_addr) = {
        let incoming = TcpListener::bind(addr)?;
        println!("listening on {}", addr);

        incoming.accept()?
    };
    println!("got connection from {}", client_addr);

    let server = TcpStream::connect(server_address)?;
    println!("connected to {}", server_address);

    let mut stats = RunStats { read: 0, write: 0 };
    println!("{:?}", run(client, server, auth_data, &mut stats));
    println!("total read: {}\ntotal write: {}", stats.read, stats.write);
    Ok(())
}
