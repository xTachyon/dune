mod codec;
mod de;
mod error;
mod protocol;
mod varint;

use crate::codec::PacketCodec;
use crate::error::MyResult;
use bytes::{Bytes, BytesMut};
use futures_util::future::join;
use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use futures_util::try_future::try_join;
use futures_util::try_stream::TryStreamExt;
use std::marker::Unpin;
use std::sync::mpsc::{channel, Receiver, Sender};
use tokio::codec::{BytesCodec, Framed};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[derive(Copy, Clone, Debug)]
enum PacketDirection {
    ClientToServer,
    ServerToClient,
}

async fn forward_data<R: AsyncReadExt + Unpin, W: AsyncWriteExt + Unpin>(
    mut reader: R,
    mut writer: W,
    channel: Sender<(PacketDirection, Bytes)>,
    direction: PacketDirection,
) -> MyResult<()> {
    let mut bytes = BytesMut::new();
    loop {
        if bytes.capacity() < 1024 {
            bytes = BytesMut::with_capacity(64 * 1024);
            bytes.resize(bytes.capacity(), 0);
        }
        let read = reader.read(&mut bytes).await?;
        if read == 0 {
            break;
        }

        let new = bytes.split_to(read).freeze();
        channel.send((direction, new.clone()))?;
        writer.write_all(&new).await?;
    }

    Ok(())
}

fn process_traffic(receiver: Receiver<(PacketDirection, Bytes)>) -> MyResult {
    loop {
        let data = receiver.recv()?;
        println!("{:?}", data);
    }

    Ok(())
}

async fn on_connected(mut client_socket: TcpStream, mut server_socket: TcpStream) -> MyResult {
    let (mut client_read, mut client_write) = client_socket.split();
    let (mut server_read, mut server_write) = server_socket.split();

    let (channel_send, channel_receive) = channel();

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

    std::thread::spawn(|| {
        println!("{:?}", process_traffic(channel_receive));
    });

    let (result1, result2) = join(first, second).await;
    result1?;
    result2?;

    Ok(())
}

#[tokio::main]
async fn main() -> MyResult {
    let mut incoming = TcpListener::bind("0.0.0.0:25565").await?;

    while let (client, _) = incoming.accept().await? {
        let task = async move {
            let server = TcpStream::connect("playmc.games:25565").await.unwrap();
            on_connected(client, server).await.unwrap();
        };
        tokio::spawn(task);
    }

    Ok(())
}
