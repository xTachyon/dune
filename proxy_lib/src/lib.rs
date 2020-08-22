mod de;
pub mod events;
mod game;
mod protocol;
mod varint;

use anyhow::Result;
use crate::protocol::ConnectionState;
use crate::protocol::Packet;
use bytes::{Bytes, BytesMut};
use futures::future::join3;
use std::marker::Unpin;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use crate::events::{EventSubscriber, ChatEvent};
use tokio::fs::File;
use std::convert::TryFrom;

#[derive(Copy, Clone, Debug)]
pub enum PacketDirection {
    ClientToServer,
    ServerToClient,
}

async fn forward_data<R: AsyncReadExt + Unpin, W: AsyncWriteExt + Unpin>(
    mut reader: R,
    mut writer: W,
    mut channel: Sender<(PacketDirection, Bytes)>,
    direction: PacketDirection,
) -> Result<()> {
    let mut bytes = BytesMut::new();
    loop {
        if bytes.capacity() == 0 || bytes.is_empty() {
            bytes = BytesMut::with_capacity(64 * 1024);
            bytes.resize(bytes.capacity(), 0);
        }
        let read = reader.read(&mut bytes).await?;
        if read == 0 {
            break;
        }

        let new = bytes.split_to(read).freeze();
        channel.send((direction, new.clone())).await?;
        writer.write_all(&new).await?;
    }

    Ok(())
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
    packet_file: File
}

impl GameData {
    async fn new(handler: Box<dyn EventSubscriber + Sync>) -> Result<GameData> {
        let packet_file = File::create("packet_file.txt").await?;
        Ok(GameData {
            client_state: Default::default(),
            server_state: Default::default(),
            connection_state: ConnectionState::Handshake,
            handler,
            packet_file
        })
    }

    fn get_state(&mut self, direction: PacketDirection) -> &mut ClientGame {
        match direction {
            PacketDirection::ClientToServer => &mut self.server_state,
            PacketDirection::ServerToClient => &mut self.client_state,
        }
    }

    async fn on_receive(&mut self, direction: PacketDirection, bytes: &[u8]) -> Result<()> {
        let state = self.get_state(direction);
        state.on_receive(bytes);

        loop {
            let connection_state = self.connection_state;
            let state = self.get_state(direction);
            if let Some(packet) = state.get_packet(direction, connection_state)? {
                match packet {
                    Packet::Unknown(_, _) => {}
                    _ => {
                        let string = format!("{:?} {:?} {:?}\n", direction, connection_state, packet);
                        self.packet_file.write_all(string.as_bytes()).await?;
                    },
                };
                match packet {
                    Packet::Handshake(x) => self.on_handshake(x)?,
                    Packet::SetCompression(x) => self.on_set_compression(x)?,
                    Packet::LoginSuccess(x) => self.on_login_success(x)?,
                    Packet::ChatResponse(x) => self.on_chat_response(x).await?,
                    Packet::SpawnMob(x) => self.on_spawn_mob(x).await?,
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

    async fn on_chat_response(&mut self, packet: protocol::ChatResponse) -> Result<()> {
        let event = ChatEvent {
            message: packet.response
        };
        self.handler.on_chat(event).await?;
        Ok(())
    }

    async fn on_spawn_mob(&mut self, packet: protocol::SpawnMob) -> Result<()> {
        // println!("{:?}", packet);
        Ok(())
    }
}

async fn process_traffic(mut receiver: Receiver<(PacketDirection, Bytes)>, handler: Box<dyn EventSubscriber + Sync>) -> Result<()> {
    let mut game = GameData::new(handler).await?;
    loop {
        let (direction, bytes) = match receiver.recv().await {
            Some(x) => x,
            None => break,
        };
        game.on_receive(direction, &bytes).await?;
    }

    Ok(())
}

async fn on_connected(mut client_socket: TcpStream, mut server_socket: TcpStream, handler: Box<dyn EventSubscriber + Sync>) -> Result<()> {
    let (client_read, client_write) = client_socket.split();
    let (server_read, server_write) = server_socket.split();

    let (channel_send, channel_receive) = channel(1024);

    let first = forward_data(
        client_read,
        server_write,
        channel_send.clone(),
        PacketDirection::ClientToServer,
    );
    let second = forward_data(
        server_read,
        client_write,
        channel_send,
        PacketDirection::ServerToClient,
    );

    let third = process_traffic(channel_receive, handler);

    let (result1, result2, result3) = join3(third, first, second).await;
    result1?;
    result2?;
    result3?;

    Ok(())
}

pub async fn do_things(server_address: &str, handler: Box<dyn EventSubscriber + Sync>) -> Result<()> {
    let mut incoming = TcpListener::bind("0.0.0.0:25565").await?;

    let (client, _) = incoming.accept().await?;
    let server = TcpStream::connect(server_address).await.unwrap();
    println!("{:?}", on_connected(client, server, handler).await);
    Ok(())
}
