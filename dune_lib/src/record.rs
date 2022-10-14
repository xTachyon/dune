use crate::protocol::de::Reader;
use crate::protocol::varint::write_varint;
use crate::protocol::{ConnectionState, Packet, PacketData, PacketDirection};
use crate::{protocol, DiskPacket};
use anyhow::Result;
use byteorder::WriteBytesExt;
use cfb8::cipher::AsyncStreamCipher;
use cfb8::cipher::NewCipher;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use polling::{Event, Poller};
use rsa::pkcs8::DecodePublicKey;
use rsa::{PaddingScheme, PublicKey, RsaPublicKey};
use serde_derive::Serialize;
use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

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

#[derive(Clone)]
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
    out_file: ZlibEncoder<File>,
}

struct OnStartResult {
    skip: bool,
    packet_data: PacketData,
}

impl Proxy {
    fn new(auth_data: AuthData, out_path: &str) -> Result<Proxy> {
        let file = File::create(out_path)?;
        Ok(Proxy {
            state: ConnectionState::Handshaking,
            compression: false,
            start_done: false,
            auth_data: Some(auth_data),
            tmp: vec![],
            out_file: ZlibEncoder::new(file, Compression::best()),
        })
    }

    fn rsa_crypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
        let public_key = RsaPublicKey::from_public_key_der(key)?;
        let padding = PaddingScheme::new_pkcs1v15_encrypt();

        let res = public_key.encrypt(&mut rand::thread_rng(), padding, data)?;
        Ok(res)
    }

    fn serialize_enc_response(shared_secret: &[u8], verify_token: &[u8]) -> Result<Vec<u8>> {
        let mut cursor = Cursor::new(Vec::new());

        // id = 1
        cursor.write_u8(1)?;

        // Shared Secret Length
        write_varint(&mut cursor, shared_secret.len() as u32)?;

        // Shared Secret
        cursor.write_all(shared_secret)?;

        // Verify Token Length
        write_varint(&mut cursor, verify_token.len() as u32)?;

        // Verify Token
        cursor.write_all(verify_token)?;

        let mut result = Vec::new();

        // size
        write_varint(&mut result, cursor.get_ref().len() as u32)?;
        result.extend_from_slice(cursor.get_ref());

        Ok(result)
    }

    fn on_start<'p>(
        &'p mut self,
        src_session: &'p mut Session,
        offset: usize,
        direction: PacketDirection,
    ) -> Result<Option<OnStartResult>> {
        let src = &src_session.read_buf[offset..];
        let packet_data = match protocol::read_packet_info(src, self.compression, &mut self.tmp)? {
            Some(x) => x,
            None => return Ok(None),
        };
        let data = packet_data.get_data(src, &self.tmp);
        let mut reader = Reader::new(data);
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
                    let mut sha1 = Sha1::new();
                    sha1.update(packet.server_id.get(data));
                    sha1.update(aes_key);
                    sha1.update(packet.public_key.get(data));
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
                        .send_string(&req);
                // println!("{:?}", response);
                let response = response?;

                // 204 No Content = Ok
                if response.status() != 204 {
                    return Err(anyhow::Error::msg("bad mojang auth"));
                }

                let buf = Proxy::serialize_enc_response(
                    &Proxy::rsa_crypt(packet.public_key.get(data), &aes_key)?,
                    &Proxy::rsa_crypt(packet.public_key.get(data), packet.verify_token.get(data))?,
                )?;
                src_session.write(&buf);

                let enc = Aes128Cfb8::new_from_slices(&aes_key, &aes_key).unwrap();
                let dec = Aes128Cfb8::new_from_slices(&aes_key, &aes_key).unwrap();

                src_session.crypt = Some(Encryption { enc, dec });
            }
            _ => {}
        }

        Ok(Some(OnStartResult { skip, packet_data }))
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
            let packet_data = result.packet_data;
            let data = packet_data.get_data(&src.read_buf[offset..], &self.tmp);

            if !result.skip {
                let disk_packet = DiskPacket {
                    id: packet_data.id,
                    direction,
                    data,
                };
                disk_packet.write(&mut self.out_file)?;

                let bytes = &src.read_buf[offset..offset + packet_data.total_size_original];
                dest.write(bytes);
            }
            offset += packet_data.total_size_original;
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
    out_path: &str,
    stats: &mut RunStats,
) -> Result<()> {
    const CLIENT_KEY: usize = 0;
    const SERVER_KEY: usize = 1;

    let mut client = Session::new();
    let mut server = Session::new();
    let mut proxy = Proxy::new(auth_data, out_path)?;

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

pub fn record_to_file(
    listen_addr: &str,
    server_address: SocketAddr,
    auth_data: AuthData,
    out_path: &str,
) -> Result<()> {
    let (client, client_addr) = {
        let incoming = TcpListener::bind(listen_addr)?;
        println!("waiting for connection..");

        incoming.accept()?
    };
    println!("got a connection from {}", client_addr);

    let server = TcpStream::connect(server_address)?;
    println!("connected to server");

    let mut stats = RunStats { read: 0, write: 0 };
    let res = run(client, server, auth_data, out_path, &mut stats);
    println!("{:?}", res);
    println!(
        "total read: {}\ntotal write: {}",
        stats.read / 2,
        stats.write / 2
    );
    Ok(())
}
