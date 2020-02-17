mod de;
mod error;
mod protocol;
mod varint;
mod game;

use crate::error::{MyError, MyResult};
use crate::protocol::ConnectionState;
use crate::protocol::Packet;
use bytes::{Bytes, BytesMut};
use futures::future::join3;
use num_traits::cast::FromPrimitive;
use std::marker::Unpin;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

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
) -> MyResult<()> {
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
    ) -> MyResult<Option<Packet>> {
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
}

impl GameData {
    fn new() -> GameData {
        GameData {
            client_state: Default::default(),
            server_state: Default::default(),
            connection_state: ConnectionState::Handshake,
        }
    }

    fn get_state(&mut self, direction: PacketDirection) -> &mut ClientGame {
        match direction {
            PacketDirection::ClientToServer => &mut self.server_state,
            PacketDirection::ServerToClient => &mut self.client_state,
        }
    }

    fn on_receive(&mut self, direction: PacketDirection, bytes: &[u8]) -> MyResult {
        let state = self.get_state(direction);
        state.on_receive(bytes);

        loop {
            let connection_state = self.connection_state;
            let state = self.get_state(direction);
            if let Some(packet) = state.get_packet(direction, connection_state)? {
                match packet {
                    Packet::Unknown(_, _) => {},
                    _ => println!("{:?} {:?} {:?}", direction, connection_state, packet)
                };
                match packet {
                    Packet::Handshake(x) => self.on_handshake(x)?,
                    Packet::SetCompression(x) => self.on_set_compression(x)?,
                    Packet::LoginSuccess(x) => self.on_login_success(x)?,
                    Packet::ChatResponse(x) => self.on_chat_response(x)?,
                    _ => {}
                };
            } else {
                break;
            }
        }
        Ok(())
    }

    fn on_handshake(&mut self, packet: protocol::Handshake) -> MyResult {
        self.connection_state = match ConnectionState::from_u32(packet.next_state.get()) {
            Some(x) => x,
            None => return Err(MyError::IntegerToEnum),
        };
        Ok(())
    }

    fn on_set_compression(&mut self, packet: protocol::SetCompression) -> MyResult {
        let value = packet.value.get() as i32;
        let value = if value < 0 { None } else { Some(value as u32) };
        self.client_state.compression = value;
        self.server_state.compression = value;
        Ok(())
    }

    fn on_login_success(&mut self, _packet: protocol::LoginSuccess) -> MyResult {
        self.connection_state = ConnectionState::Play;

        Ok(())
    }

    fn on_chat_response(&mut self, packet: protocol::ChatResponse) -> MyResult {
        println!("{}", packet.response);
        Ok(())
    }
}

async fn process_traffic(mut receiver: Receiver<(PacketDirection, Bytes)>) -> MyResult {
    let mut game = GameData::new();
    loop {
        let (direction, bytes) = match receiver.recv().await {
            Some(x) => x,
            None => break,
        };
        game.on_receive(direction, &bytes)?;
    }

    Ok(())
}

async fn on_connected(mut client_socket: TcpStream, mut server_socket: TcpStream) -> MyResult {
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

    let third = process_traffic(channel_receive);

    let (result1, result2, result3) = join3(third, first, second).await;
    result1?;
    result2?;
    result3?;

    Ok(())
}

#[tokio::main]
async fn main() -> MyResult {
    let mut incoming = TcpListener::bind("0.0.0.0:25565").await?;

    loop {
        let (client, _) = incoming.accept().await?;
        let task = async move {
            let server = TcpStream::connect("127.0.0.1:25566").await.unwrap();
            println!("{:?}", on_connected(client, server).await);
        };
        tokio::spawn(task);
    }
}
