pub mod events;
mod game;
mod protocol;

use crate::events::EventSubscriber;
use crate::protocol::v1_18_1::login::EncryptionBeginRequest;
use crate::protocol::{ConnectionState, Packet, PacketDirection};
use anyhow::Result;
use byteorder::WriteBytesExt;
use cfb8::cipher::AsyncStreamCipher;
use cfb8::cipher::NewCipher;
use polling::{Event, Poller};
use protocol::de::Reader;
use protocol::varint::write_varint;
use rand::RngCore;
use rsa::pkcs8::DecodePublicKey;
use rsa::{PaddingScheme, PublicKey, RsaPublicKey};
use serde_derive::Serialize;
use sha1::Digest;
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
    fn write(&mut self, buf: &mut [u8]) {
        if let Some(crypt) = &mut self.crypt {
            crypt.enc.encrypt(buf);
        }

        // for i in buf.iter() {
        //     if i.is_ascii() && !i.is_ascii_control() {
        //         print!("{}", *i as char);
        //     }
        // }

        self.write_buf.extend_from_slice(buf);
    }

    fn read(&mut self, buf: &mut [u8]) {
        if let Some(crypt) = &mut self.crypt {
            crypt.dec.decrypt(buf);
        }

        // for i in buf.iter() {
        //     if i.is_ascii() && !i.is_ascii_control() {
        //         print!("{}", *i as char);
        //     }
        // }

        self.read_buf.extend_from_slice(buf);
    }
}

pub struct AuthData {
    pub selected_profile: String,
    pub access_token: String,
}

struct Proxy {
    client: Session,
    server: Session,
    state: ConnectionState,
    compression: bool,
    start_done: bool,
    auth_data: Option<AuthData>,
    tmp: Vec<u8>,
}

impl Proxy {
    fn new(auth_data: AuthData) -> Proxy {
        Proxy {
            client: Session {
                read_buf: vec![],
                write_buf: vec![],
                crypt: None,
            },
            server: Session {
                read_buf: vec![],
                write_buf: vec![],
                crypt: None,
            },
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
        cursor.write_all(&packet.shared_secret)?;

        // Verify Token Length
        let (buf, size) = write_varint(packet.verify_token.len() as u32);
        cursor.write_all(&buf[..size])?;

        // Verify Token
        cursor.write_all(&packet.verify_token)?;

        let mut result = Vec::new();

        // size
        let (buf, size) = write_varint(cursor.get_ref().len() as u32);
        result.extend_from_slice(&buf[..size]);
        result.extend_from_slice(cursor.get_ref());

        Ok(result)
    }

    fn on_start(&mut self, direction: PacketDirection) -> Result<bool> {
        self.tmp.clear();
        match direction {
            PacketDirection::ClientToServer => {
                let session = &mut self.client;
                let mut reader = Reader::new(&session.read_buf);
                let p = protocol::deserialize_with_header(
                    direction,
                    self.state,
                    &mut reader,
                    self.compression,
                    &mut self.tmp,
                )?;
                match p {
                    Some((packet, size)) => {
                        match packet {
                            Packet::SetProtocolRequest(x) => {
                                if x.next_state == 1 {
                                    self.start_done = true;
                                    self.state = ConnectionState::Status;
                                } else if x.next_state == 2 {
                                    self.state = ConnectionState::Login;
                                } else {
                                    unreachable!();
                                }
                            }
                            _ => {}
                        }
                        session.read_buf.drain(..size);
                    }
                    None => return Ok(false),
                }
            }
            PacketDirection::ServerToClient => {
                let session = &mut self.server;
                let mut reader = Reader::new(&session.read_buf);
                let p = protocol::deserialize_with_header(
                    direction,
                    self.state,
                    &mut reader,
                    self.compression,
                    &mut self.tmp,
                )?;
                match p {
                    Some((packet, size)) => {
                        println!("{:?}", packet);
                        match packet {
                            Packet::SuccessResponse(_) => {
                                self.start_done = true;
                                self.state = ConnectionState::Play;
                            }
                            Packet::CompressResponse(x) => {
                                self.compression = x.threshold >= 0;
                            }
                            Packet::EncryptionBeginResponse(packet) => {
                                let aes_key: [u8; 16] = rand::random();

                                let mut buffer = [0; 16];
                                rand::thread_rng().fill_bytes(&mut buffer);

                                let hash = {
                                    let mut sha1 = sha1::Sha1::new();
                                    sha1.update(packet.server_id);
                                    sha1.update(aes_key);
                                    sha1.update(&packet.public_key);
                                    let hash = sha1.finalize();

                                    num_bigint::BigInt::from_signed_bytes_be(&hash).to_str_radix(16)
                                };

                                let auth_data = self.auth_data.take().unwrap();
                                let selected_profile = auth_data.selected_profile.replace("-", "");

                                #[allow(non_snake_case)]
                                #[derive(Serialize)]
                                struct RequestData {
                                    accessToken: String,
                                    selectedProfile: String,
                                    serverId: String,
                                }
                                let req = RequestData {
                                    accessToken: auth_data.access_token,
                                    selectedProfile: selected_profile,
                                    serverId: hash,
                                };
                                let req = serde_json::to_string(&req)?;
                                let response = ureq::post(
                                    "https://sessionserver.mojang.com/session/minecraft/join",
                                )
                                .set("Content-Type", "application/json; charset=utf-8")
                                .send_string(&req)?;

                                // 204 No Content = Ok
                                if response.status() != 204 {
                                    return Err(anyhow::Error::msg("bad mojang auth"));
                                }

                                let response = EncryptionBeginRequest {
                                    shared_secret: &Proxy::rsa_crypt(&packet.public_key, &aes_key)?,
                                    verify_token: &Proxy::rsa_crypt(
                                        &packet.public_key,
                                        &packet.verify_token,
                                    )?,
                                };
                                let mut buf = Proxy::serialize_enc_response(response)?;
                                self.server.write(&mut buf);

                                let enc = Aes128Cfb8::new_from_slices(&aes_key, &aes_key).unwrap();
                                let dec = Aes128Cfb8::new_from_slices(&aes_key, &aes_key).unwrap();

                                self.server.crypt = Some(Encryption { enc, dec });
                            }
                            _ => {}
                        }
                        self.server.read_buf.drain(..size);
                    }
                    None => {
                        return Ok(false);
                    }
                }
            }
        }
        Ok(true)
    }

    fn on_recv(&mut self, buf: &mut [u8], direction: PacketDirection) -> Result<()> {
        match direction {
            PacketDirection::ClientToServer => {
                self.client.read(buf);
                self.server.write(buf);
            }
            PacketDirection::ServerToClient => {
                self.server.read(buf);
                self.client.write(buf);
            }
        }

        while self.on_start(direction)? {}

        Ok(())
    }
}

fn run(
    mut client_socket: TcpStream,
    mut server_socket: TcpStream,
    auth_data: AuthData,
) -> Result<()> {
    const CLIENT_KEY: usize = 0;
    const SERVER_KEY: usize = 1;

    let mut proxy = Proxy::new(auth_data);

    let poller = Poller::new()?;

    client_socket.set_nonblocking(true)?;
    server_socket.set_nonblocking(true)?;

    poller.add(&client_socket, Event::readable(CLIENT_KEY))?;
    poller.add(&server_socket, Event::readable(SERVER_KEY))?;

    let mut events = Vec::new();
    let mut buffer = [0; 16 * 1024];
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
                proxy.on_recv(&mut buffer[..read], direction)?;
            }
            if ev.writable {
                if ev.key == CLIENT_KEY {
                    let wrote = client_socket.write(&proxy.client.write_buf)?;
                    proxy.client.write_buf.drain(..wrote);
                } else {
                    let wrote = server_socket.write(&proxy.server.write_buf)?;
                    proxy.server.write_buf.drain(..wrote);
                }
            }
        }

        poller.modify(
            &client_socket,
            polling::Event {
                key: CLIENT_KEY,
                readable: true,
                writable: !proxy.client.write_buf.is_empty(),
            },
        )?;
        poller.modify(
            &server_socket,
            polling::Event {
                key: SERVER_KEY,
                readable: true,
                writable: !proxy.server.write_buf.is_empty(),
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

    println!("{:?}", run(client, server, auth_data));
    Ok(())
}
