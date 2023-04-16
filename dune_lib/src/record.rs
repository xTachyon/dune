use crate::client::{Aes128Cfb8, ClientReader, ClientWriter};
use crate::DiskPacket;
use aes::cipher::NewCipher;
use anyhow::Result;
use dune_data::protocol;
use dune_data::protocol::v1_18_2::handshaking::SetProtocolRequest;
use dune_data::protocol::v1_18_2::login::{EncryptionBeginRequest, EncryptionBeginResponse};
use dune_data::protocol::{ConnectionState, Packet, PacketData, PacketDirection};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use polling::{Event, Poller};
use rsa::pkcs8::DecodePublicKey;
use rsa::{PaddingScheme, PublicKey, RsaPublicKey};
use serde_derive::Serialize;
use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[derive(Clone)]
pub struct AuthData {
    pub selected_profile: String,
    pub access_token: String,
    pub name: String,
}

struct Proxy<'x> {
    state: ConnectionState,
    compression: bool,
    start_done: bool,
    auth_data: AuthData,
    server_host: (&'x str, u16),
    out_file: ZlibEncoder<File>,
}

struct OnStartResult<'x> {
    skip: bool,
    packet_data: PacketData<'x>,
}

fn rsa_crypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    let public_key = RsaPublicKey::from_public_key_der(key)?;
    let padding = PaddingScheme::new_pkcs1v15_encrypt();

    let res = public_key.encrypt(&mut rand::thread_rng(), padding, data)?;
    Ok(res)
}
pub(crate) fn crypt_reply(
    packet: EncryptionBeginResponse,
    auth_data: &mut AuthData,
    writer: &mut ClientWriter,
) -> Result<(Aes128Cfb8, Aes128Cfb8)> {
    let aes_key: [u8; 16] = rand::random();

    let hash = {
        let mut sha1 = Sha1::new();
        sha1.update(packet.server_id);
        sha1.update(aes_key);
        sha1.update(packet.public_key);
        let hash = sha1.finalize();

        num_bigint::BigInt::from_signed_bytes_be(&hash).to_str_radix(16)
    };

    auth_data.selected_profile.retain(|c| c != '-');

    #[allow(non_snake_case)]
    #[derive(Serialize)]
    struct RequestData<'x> {
        accessToken: &'x str,
        selectedProfile: &'x str,
        serverId: &'x str,
    }
    let req = RequestData {
        accessToken: &auth_data.access_token,
        selectedProfile: &auth_data.selected_profile,
        serverId: &hash,
    };
    let req = serde_json::to_string(&req)?;
    dbg!(&req);
    let response = ureq::post("https://sessionserver.mojang.com/session/minecraft/join")
        .set("Content-Type", "application/json; charset=utf-8")
        .send_string(&req);
    // println!("{:?}", response);
    let response = response?;

    // 204 No Content = Ok
    if response.status() != 204 {
        return Err(anyhow::Error::msg("bad mojang auth"));
    }

    let p = EncryptionBeginRequest {
        shared_secret: &rsa_crypt(packet.public_key, &aes_key)?,
        verify_token: &rsa_crypt(packet.public_key, packet.verify_token)?,
    };
    writer.send_packet(p)?;

    let result = (
        Aes128Cfb8::new_from_slices(&aes_key, &aes_key).unwrap(),
        Aes128Cfb8::new_from_slices(&aes_key, &aes_key).unwrap(),
    );
    Ok(result)
}

impl<'x> Proxy<'x> {
    fn new(auth_data: AuthData, server_host: (&'x str, u16), out_path: &str) -> Result<Proxy<'x>> {
        let file = File::create(out_path)?;
        Ok(Proxy {
            state: ConnectionState::Handshaking,
            compression: false,
            start_done: false,
            auth_data,
            server_host,
            out_file: ZlibEncoder::new(file, Compression::best()),
        })
    }

    fn on_start<'p>(
        &mut self,
        src_reader: &'p mut ClientReader,
        src_writer: &mut ClientWriter,
        dest_writer: &mut ClientWriter,
        direction: PacketDirection,
    ) -> Result<Option<OnStartResult<'p>>> {
        let packet_data = match protocol::read_packet_info(
            &src_reader.buffer,
            &mut src_reader.tmp,
            self.compression,
        )? {
            Some(x) => x,
            None => return Ok(None),
        };
        let mut data = packet_data.data;
        let packet = protocol::deserialize(self.state, direction, packet_data.id, &mut data)?;

        println!("{:?}", packet);
        let mut skip = false;
        match packet {
            Packet::SetProtocolRequest(x) => {
                match x.next_state {
                    1 => {
                        self.start_done = true;
                        self.state = ConnectionState::Status;
                    }
                    2 => {
                        self.state = ConnectionState::Login;
                    }
                    _ => unimplemented!(),
                }
                skip = true;
                let (addr, port) = self.server_host;
                let p = SetProtocolRequest {
                    protocol_version: x.protocol_version,
                    server_host: addr,
                    server_port: port,
                    next_state: x.next_state,
                };
                dest_writer.send_packet(p)?;
            }

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
                let (c1, c2) = crypt_reply(packet, &mut self.auth_data, src_writer)?;
                src_reader.crypt = Some(c1);
                src_writer.crypt = Some(c2);
            }
            _ => {}
        }

        Ok(Some(OnStartResult { skip, packet_data }))
    }

    fn forward(
        &mut self,
        src_reader: &mut ClientReader,
        src_writer: &mut ClientWriter,
        dest_writer: &mut ClientWriter,
        buf: &[u8],
        direction: PacketDirection,
    ) -> Result<()> {
        src_reader.add(buf);

        while let Some(result) = self.on_start(src_reader, src_writer, dest_writer, direction)? {
            let packet_data = result.packet_data;
            let data = packet_data.data;
            let total_size_original = packet_data.total_size;

            if !result.skip {
                let disk_packet = DiskPacket {
                    id: packet_data.id,
                    direction,
                    data,
                };
                disk_packet.write(&mut self.out_file)?;

                let bytes = &src_reader.buffer[..total_size_original];
                dest_writer.add(bytes);
            }
            src_reader.buffer.advance(total_size_original);
        }

        Ok(())
    }

    fn on_recv(
        &mut self,
        buf: &[u8],
        client_reader: &mut ClientReader,
        client_writer: &mut ClientWriter,
        server_reader: &mut ClientReader,
        server_writer: &mut ClientWriter,
        direction: PacketDirection,
    ) -> Result<()> {
        match direction {
            PacketDirection::C2S => {
                self.forward(client_reader, client_writer, server_writer, buf, direction)
            }
            PacketDirection::S2C => {
                self.forward(server_reader, server_writer, client_writer, buf, direction)
            }
        }
    }
}

fn run(
    mut client_socket: TcpStream,
    mut server_socket: TcpStream,
    auth_data: AuthData,
    server_host: (&str, u16),
    out_path: &str,
) -> Result<()> {
    const CLIENT_KEY: usize = 0;
    const SERVER_KEY: usize = 1;

    let mut client_reader = ClientReader::default();
    let mut client_writer = ClientWriter::default();
    let mut server_reader = ClientReader::default();
    let mut server_writer = ClientWriter::default();

    let mut proxy = Proxy::new(auth_data, server_host, out_path)?;

    let poller = Poller::new()?;

    client_socket.set_nonblocking(true)?;
    server_socket.set_nonblocking(true)?;

    poller.add(&client_socket, Event::readable(CLIENT_KEY))?;
    poller.add(&server_socket, Event::readable(SERVER_KEY))?;

    let mut events = Vec::new();
    let mut buffer = [0; 4096];
    loop {
        events.clear();
        poller.wait(&mut events, None)?;

        for ev in &events {
            if ev.readable {
                let (read, direction) = if ev.key == CLIENT_KEY {
                    (client_socket.read(&mut buffer)?, PacketDirection::C2S)
                } else {
                    (server_socket.read(&mut buffer)?, PacketDirection::S2C)
                };
                // println!("{:?}: {}", direction, read);
                if read == 0 {
                    return Ok(());
                }
                proxy.on_recv(
                    &buffer[..read],
                    &mut client_reader,
                    &mut client_writer,
                    &mut server_reader,
                    &mut server_writer,
                    direction,
                )?;
            }
            if ev.writable {
                let (socket, buffer) = if ev.key == CLIENT_KEY {
                    (&mut client_socket, &mut client_writer.buffer)
                } else {
                    (&mut server_socket, &mut server_writer.buffer)
                };
                let wrote = socket.write(buffer)?;
                buffer.advance(wrote);
            }
        }

        poller.modify(
            &client_socket,
            Event {
                key: CLIENT_KEY,
                readable: true,
                writable: !client_writer.buffer.is_empty(),
            },
        )?;
        poller.modify(
            &server_socket,
            Event {
                key: SERVER_KEY,
                readable: true,
                writable: !server_writer.buffer.is_empty(),
            },
        )?;
    }
}

pub fn record_to_file(
    listen_addr: (&str, u16),
    auth_data: AuthData,
    server_host: (&str, u16),
    out_path: &str,
) -> Result<()> {
    let (client, client_addr) = {
        let incoming = TcpListener::bind(listen_addr)?;
        println!("waiting for connection..");

        incoming.accept()?
    };
    println!("got a connection from {}", client_addr);

    let server = TcpStream::connect(server_host)?;
    println!("connected to server");

    let res = run(client, server, auth_data, server_host, out_path);
    println!("{:?}", res);
    Ok(())
}
