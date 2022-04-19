mod de;
pub mod events;
mod game;
mod protocol;
mod varint;

use crate::events::{ChatEvent, EventSubscriber};
use crate::protocol::ConnectionState;
use crate::protocol::Packet;
use crate::ConnectionState::Handshake;
use anyhow::Result;
use bytes::{Bytes, BytesMut};
use polling::{Event, Poller};
use std::convert::TryFrom;
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Sender};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PacketDirection {
    ClientToServer,
    ServerToClient,
}

#[derive(Default)]
struct ClientGame {
    buffer: Vec<u8>,
    buffer_offset: usize,
    compression: Option<u32>,
}

impl ClientGame {
    fn on_receive(&mut self, new: &[u8]) {
        if self.buffer_offset > self.buffer.len() - self.buffer_offset {
            self.buffer.drain(..self.buffer_offset);
            self.buffer_offset = 0;
        }
        self.buffer.extend_from_slice(new);
    }

    fn get_packet(
        &mut self,
        direction: PacketDirection,
        state: ConnectionState,
    ) -> Result<Option<Packet>> {
        match protocol::deserialize_with_header(
            direction,
            state,
            &self.buffer[self.buffer_offset..],
            self.compression,
        )? {
            Some((packet, offset)) => {
                self.buffer_offset += offset;
                Ok(Some(packet))
            }
            None => Ok(None),
        }
    }
}

struct GameData {
    client_state: ClientGame,
    server_state: ClientGame,
    connection_state: ConnectionState,
    handler: Box<dyn EventSubscriber + Sync>,
    packet_file: File,
}

impl GameData {
    fn new(handler: Box<dyn EventSubscriber + Sync>) -> Result<GameData> {
        let packet_file = File::create("packet_file.txt")?;
        Ok(GameData {
            client_state: Default::default(),
            server_state: Default::default(),
            connection_state: ConnectionState::Handshake,
            handler,
            packet_file,
        })
    }

    fn get_state(&mut self, direction: PacketDirection) -> &mut ClientGame {
        match direction {
            PacketDirection::ClientToServer => &mut self.server_state,
            PacketDirection::ServerToClient => &mut self.client_state,
        }
    }

    fn on_receive(&mut self, direction: PacketDirection, bytes: &[u8]) -> Result<()> {
        let state = self.get_state(direction);
        state.on_receive(bytes);

        loop {
            let connection_state = self.connection_state;
            let state = self.get_state(direction);
            if let Some(packet) = state.get_packet(direction, connection_state)? {
                match packet {
                    Packet::Unknown(_, _) => {}
                    _ => {
                        let string =
                            format!("{:?} {:?} {:?}\n", direction, connection_state, packet);
                        self.packet_file.write_all(string.as_bytes())?;
                    }
                };
                match packet {
                    Packet::Handshake(x) => self.on_handshake(x)?,
                    Packet::SetCompression(x) => self.on_set_compression(x)?,
                    Packet::LoginSuccess(x) => self.on_login_success(x)?,
                    Packet::ChatResponse(x) => self.on_chat_response(x)?,
                    Packet::SpawnMob(x) => self.on_spawn_mob(x)?,
                    _ => {}
                };
            } else {
                break;
            }
        }
        Ok(())
    }

    fn on_handshake(&mut self, packet: protocol::Handshake) -> Result<()> {
        self.connection_state = ConnectionState::try_from(packet.next_state.get() as u8)?;
        Ok(())
    }

    fn on_set_compression(&mut self, packet: protocol::SetCompression) -> Result<()> {
        let value = packet.value.get() as i32;
        let value = if value < 0 { None } else { Some(value as u32) };
        self.client_state.compression = value;
        self.server_state.compression = value;
        Ok(())
    }

    fn on_login_success(&mut self, _packet: protocol::LoginSuccess) -> Result<()> {
        self.connection_state = ConnectionState::Play;

        Ok(())
    }

    fn on_chat_response(&mut self, packet: protocol::ChatResponse) -> Result<()> {
        let event = ChatEvent {
            message: packet.response,
        };
        self.handler.on_chat(event)?;
        Ok(())
    }

    fn on_spawn_mob(&mut self, packet: protocol::SpawnMob) -> Result<()> {
        // println!("{:?}", packet);
        Ok(())
    }
}

// async fn process_traffic(mut receiver: Receiver<(PacketDirection, Bytes)>, handler: Box<dyn EventSubscriber + Sync>) -> Result<()> {
//     let mut game = GameData::new(handler).await?;
//     loop {
//         let (direction, bytes) = match receiver.recv().await {
//             Some(x) => x,
//             None => break,
//         };
//         game.on_receive(direction, &bytes).await?;
//     }
//
//     Ok(())
// }

// async fn forward_data<R: AsyncReadExt + Unpin, W: AsyncWriteExt + Unpin>(
//     mut reader: R,
//     mut writer: W,
//     mut channel: Sender<(PacketDirection, Bytes)>,
//     direction: PacketDirection,
// ) -> Result<()> {
//     let mut bytes = BytesMut::new();
//     loop {
//         if bytes.capacity() == 0 || bytes.is_empty() {
//             bytes = BytesMut::with_capacity(64 * 1024);
//             bytes.resize(bytes.capacity(), 0);
//         }
//         let read = reader.read(&mut bytes).await?;
//         if read == 0 {
//             break;
//         }
//
//         let new = bytes.split_to(read).freeze();
//         channel.send((direction, new.clone())).await?;
//         writer.write_all(&new).await?;
//     }
//
//     Ok(())
// }

fn forward_data<R: Read, W: Write>(
    mut reader: R,
    mut writer: W,
    mut channel: Sender<(PacketDirection, Bytes)>,
    direction: PacketDirection,
) -> Result<()> {
    const SIZE: usize = 64 * 1024;
    let mut bytes = [0; SIZE];
    let mut bytes_send = BytesMut::with_capacity(SIZE);
    loop {
        let read = reader.read(&mut bytes)?;
        if read == 0 {
            break;
        }
        println!("read {} bytes", read);

        bytes_send.extend_from_slice(&bytes[..read]);
        let new = bytes_send.split_to(read).freeze();
        channel.send((direction, new));

        writer.write_all(&bytes[..read])?;
    }

    Ok(())
}

fn read_some(mut socket: &TcpStream, buffer: &mut Vec<u8>) -> Result<()> {
    let mut bytes = [0; 4096];
    let read = socket.read(&mut bytes)?;
    buffer.extend_from_slice(&bytes[..read]);

    Ok(())
}

fn read_packet(
    socket: &TcpStream,
    buffer: &mut Vec<u8>,
    state: ConnectionState,
    direction: PacketDirection,
) -> Result<Packet> {
    loop {
        read_some(socket, buffer)?;
        if let Some((packet, size)) =
            protocol::deserialize_with_header(direction, state, &buffer, None)?
        {
            buffer.drain(..size);
            println!("{:?}", packet);
            return Ok(packet);
        }
    }
}

fn do_login(client_socket: &TcpStream, server_socket: &TcpStream) -> Result<()> {
    let mut c2s_buffer = Vec::new();
    // let mut s2c_buffer = Vec::new();

    read_packet(
        client_socket,
        &mut c2s_buffer,
        ConnectionState::Handshake,
        PacketDirection::ClientToServer,
    )?;

    Ok(())
}

fn on_connected(
    mut client_socket: TcpStream,
    mut server_socket: TcpStream,
    handler: Box<dyn EventSubscriber + Sync>,
) -> Result<()> {
    // do_login(&client_socket, &server_socket)?;

    let (send, recv) = channel();
    let send2 = send.clone();

    let client_socket2 = client_socket.try_clone()?;
    let server_socket2 = server_socket.try_clone()?;
    let t1 = std::thread::spawn(|| {
        println!(
            "{:?}",
            forward_data(
                client_socket,
                server_socket,
                send,
                PacketDirection::ClientToServer,
            )
        );
    });

    let t2 = std::thread::spawn(|| {
        println!(
            "{:?}",
            forward_data(
                server_socket2,
                client_socket2,
                send2,
                PacketDirection::ServerToClient,
            )
        );
    });

    t1.join();
    t2.join();

    Ok(())
}

struct Session {
    read_buffer: Vec<u8>,
    write_buffer: Vec<u8>,
}

struct Proxy {
    client: Session,
    server: Session,
    state: ConnectionState,
    compression: Option<u32>,
    start_done: bool,
}

impl Proxy {
    fn new() -> Proxy {
        Proxy {
            client: Session {
                read_buffer: vec![],
                write_buffer: vec![],
            },
            server: Session {
                read_buffer: vec![],
                write_buffer: vec![],
            },
            state: ConnectionState::Handshake,
            compression: None,
            start_done: false,
        }
    }

    fn on_start(&mut self, direction: PacketDirection) -> Result<()> {
        let session = if direction == PacketDirection::ClientToServer {
            &mut self.client
        } else {
            &mut self.server
        };
        while let Some((packet, size)) = protocol::deserialize_with_header(
            direction,
            self.state,
            &session.read_buffer,
            self.compression,
        )? {
            println!("{:?}", packet);
            session.read_buffer.drain(..size);
            match packet {
                Packet::Handshake(x) => {
                    if *x.next_state == 1 {
                        self.start_done = true;
                        return Ok(());
                    }
                    assert_eq!(*x.next_state, 2);
                    self.state = ConnectionState::Login;
                }
                Packet::LoginStart(_) => {}
                Packet::LoginSuccess(x) => {
                    self.start_done = true;
                    self.state = ConnectionState::Play;
                }
                Packet::SetCompression(x) => {
                    self.compression = Some(*x.value);
                }
                _ => unimplemented!(),
            }
        }

        Ok(())
    }

    fn on_recv(&mut self, buf: &[u8], direction: PacketDirection) -> Result<()> {
        if direction == PacketDirection::ClientToServer {
            self.client.read_buffer.extend_from_slice(buf);
            self.server.write_buffer.extend_from_slice(buf);
        } else {
            self.server.read_buffer.extend_from_slice(buf);
            self.client.write_buffer.extend_from_slice(buf);
        }

        if !self.start_done {
            self.on_start(direction)?;
        }

        Ok(())
    }
}

fn run(mut client_socket: TcpStream, mut server_socket: TcpStream) -> Result<()> {
    const CLIENT_KEY: usize = 0;
    const SERVER_KEY: usize = 1;

    let mut proxy = Proxy::new();

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
                println!("{:?}: {}", direction, read);
                if read == 0 {
                    return Ok(());
                }
                proxy.on_recv(&buffer[..read], direction)?;
            }
            if ev.writable {
                if ev.key == CLIENT_KEY {
                    let wrote = client_socket.write(&proxy.client.write_buffer)?;
                    proxy.client.write_buffer.drain(..wrote);
                } else {
                    let wrote = server_socket.write(&proxy.server.write_buffer)?;
                    proxy.server.write_buffer.drain(..wrote);
                }
            }
        }

        poller.modify(
            &client_socket,
            polling::Event {
                key: CLIENT_KEY,
                readable: true,
                writable: !proxy.client.write_buffer.is_empty(),
            },
        )?;
        poller.modify(
            &server_socket,
            polling::Event {
                key: SERVER_KEY,
                readable: true,
                writable: !proxy.server.write_buffer.is_empty(),
            },
        )?;
    }

    Ok(())
}

pub fn do_things(server_address: &str, handler: Box<dyn EventSubscriber + Sync>) -> Result<()> {
    let mut incoming = TcpListener::bind("0.0.0.0:25566")?;

    let (client, _) = incoming.accept()?;
    let server = TcpStream::connect(server_address)?;

    println!("{:?}", run(client, server));
    // println!("{:?}", on_connected(client, server, handler));
    Ok(())
}
