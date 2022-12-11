use crate::protocol::ConnectionState;
use crate::protocol::v1_18_2::handshaking::SetProtocolRequest;
use crate::protocol::v1_18_2::login::LoginStartRequest;
use crate::protocol::varint::{write_varint_with_size, write_varint};
use crate::record::Session;
use anyhow::Result;
use polling::{Event, Poller};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};

struct Client {
    
}
impl Client {
    fn send_packet() -> Result<()> {

        Ok(())
    }
}

fn send_start(mut writer: &mut Vec<u8>) -> Result<()> {
    let mut tmp = Vec::new();

    SetProtocolRequest::write(&mut tmp, 758, "localhost", 25565, ConnectionState::Login as i32)?;
    write_varint(&mut writer, tmp.len() as u32)?;
    writer.extend_from_slice(&tmp);
    tmp.clear();


    LoginStartRequest::write(&mut tmp, "me")?;
    write_varint(&mut writer, tmp.len() as u32)?;
    writer.extend_from_slice(&tmp);
    
    Ok(())
}

pub fn run(addr: SocketAddr) -> Result<()> {
    const SOCKET_KEY: usize = 0;

    let mut socket = TcpStream::connect(addr)?;
    socket.set_nonblocking(true)?;

    let poller = Poller::new()?;
    poller.add(&socket, Event::all(SOCKET_KEY))?;

    let mut session = Session::new();
    let mut events = Vec::new();
    let mut buffer = [0; 4096];

    send_start(&mut session.write_buf)?;
    loop {
        events.clear();
        poller.wait(&mut events, None)?;

        for ev in &events {
            if ev.readable {
                let read = socket.read(&mut buffer)?;
                if read == 0 {
                    return Ok(());
                }
                println!("{:?}", &buffer[..read]);
                session.read(&buffer[..read]);
            }

            if ev.writable {
                let wrote = socket.write(&session.write_buf)?;
                session.write_buf.drain(..wrote);
            }
        }

        poller.modify(
            &socket,
            Event {
                key: SOCKET_KEY,
                readable: true,
                writable: !session.write_buf.is_empty(),
            },
        )?;
    }
}
