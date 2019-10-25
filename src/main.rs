mod codec;
mod de;
mod protocol;
mod varint;

use tokio::net::{TcpListener, TcpStream};
use tokio::codec::{Framed, BytesCodec};
use futures_util::stream::StreamExt;
use futures_util::try_stream::TryStreamExt;
use futures_util::sink::SinkExt;
use crate::codec::PacketCodec;

type MyResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

async fn on_connected(client_socket: TcpStream, server_socket: TcpStream) -> MyResult<()> {
  let client_framed = Framed::new(
    client_socket,
    PacketCodec,
  );

  let (mut client_write, mut client_read) = client_framed.split();

  let server_framed = Framed::new(
    server_socket,
    PacketCodec,
  );

  let (mut server_write, mut server_read) = server_framed.split();

  tokio::spawn(async move {
    while let Ok(Some(packet)) = client_read.try_next().await {
      println!("ClientToServer: {:?}", packet);
      server_write.send(packet).await.unwrap();
    }
  });

  tokio::spawn(async move {
    while let Ok(Some(packet)) = server_read.try_next().await {
      println!("ServerToClient: {:?}", packet);
      client_write.send(packet).await.unwrap();
    }
  });


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
