use crate::record::Session;
use anyhow::Result;
use polling::{Event, Poller};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};

pub struct Client {}

pub fn run(addr: SocketAddr) -> Result<()> {
    const SOCKET_KEY: usize = 0;

    let mut socket = TcpStream::connect(addr)?;
    socket.set_nonblocking(true)?;

    let poller = Poller::new()?;
    poller.add(&socket, Event::all(SOCKET_KEY))?;

    let mut session = Session::new();
    let mut events = Vec::new();
    let mut buffer = [0; 4096];
    loop {
        events.clear();
        poller.wait(&mut events, None)?;

        for ev in &events {
            if ev.readable {
                let read = socket.read(&mut buffer)?;
                if read == 0 {
                    return Ok(());
                }
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
