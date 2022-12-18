use crate::protocol::de::MD;
use crate::protocol::v1_18_2::handshaking::SetProtocolRequest;
use crate::protocol::v1_18_2::login::LoginStartRequest;
use crate::protocol::v1_18_2::play::{ChatRequest, KeepAliveRequest};
use crate::protocol::varint::{write_varint, write_varint_serialize, VarintSerialized};
use crate::protocol::{self, ConnectionState, Packet, PacketDirection};
use crate::{chat, Buffer};
use aes::cipher::AsyncStreamCipher;
use anyhow::Result;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use log::{info, warn};
use polling::{Event, Poller};
use std::borrow::Borrow;
use std::io::{stdin, BufRead, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;

pub(crate) type Aes128Cfb8 = cfb8::Cfb8<aes::Aes128>;

#[derive(Default)]
pub(crate) struct ClientReader {
    pub(crate) buffer: Buffer,
    pub(crate) crypt: Option<Aes128Cfb8>,
    pub(crate) tmp: Vec<u8>,
}
impl ClientReader {
    pub(crate) fn add(&mut self, buf: &[u8]) {
        let offset = self.buffer.len();
        self.buffer.extend_from_slice(buf);

        if let Some(crypt) = &mut self.crypt {
            crypt.decrypt(&mut self.buffer[offset..]);
        }
    }
}

#[derive(Default)]
pub(crate) struct ClientWriter {
    pub(crate) buffer: Buffer,
    pub(crate) crypt: Option<Aes128Cfb8>,
    tmp: Vec<u8>,
    tmp2: Vec<u8>,
    compression_threshold: Option<usize>,
}
impl ClientWriter {
    pub(crate) fn add(&mut self, buf: &[u8]) {
        let offset = self.buffer.len();
        self.buffer.extend_from_slice(buf);

        if let Some(crypt) = &mut self.crypt {
            crypt.encrypt(&mut self.buffer[offset..]);
        }
    }

    pub(crate) fn send_packet<'x, P, Q>(&mut self, packet: P) -> Result<()>
    where
        Q: MD<'x>,
        P: Borrow<Q>,
    {
        self.tmp.clear();
        self.tmp2.clear();
        // can we do this with only one tmp? mojang :squint:

        packet.borrow().serialize(&mut self.tmp)?;

        let (packet_buffer, data_length) = match self.compression_threshold {
            Some(threshold) if self.tmp.len() >= threshold => {
                // if at least as big as threshold, compress it in tmp2

                let mut compressor = ZlibEncoder::new(&mut self.tmp2, Compression::fast());
                compressor.write_all(&self.tmp)?;
                compressor.finish()?;

                let data_length_buffer = write_varint_serialize(self.tmp2.len() as u32);
                (&self.tmp2, data_length_buffer)
            }
            Some(_) => (&self.tmp, write_varint_serialize(0)),
            None => (&self.tmp, VarintSerialized::default()),
        };
        // now the compressed or uncompressed data is in tmp or tmp2

        let total_size = packet_buffer.len() + data_length.size;
        write_varint(&mut self.buffer, total_size as u32)?;

        if data_length.size != 0 {
            self.buffer
                .extend_from_slice(&data_length.buffer[..data_length.size]);
        }
        self.buffer.extend_from_slice(packet_buffer);

        // and you're wondering why minecraft is slow
        Ok(())
    }
}

struct Client {
    pub(crate) compression: bool,
    pub(crate) state: ConnectionState,
}
impl Client {
    fn new() -> Client {
        Client {
            compression: false,
            state: ConnectionState::Login,
        }
    }
}

fn send_start(client: &mut ClientWriter) -> Result<()> {
    let p = SetProtocolRequest {
        protocol_version: 758,
        server_host: "localhost",
        server_port: 25565,
        next_state: ConnectionState::Login as i32,
    };
    client.send_packet(p)?;

    let p = LoginStartRequest { username: "grape" };
    client.send_packet(p)?;

    Ok(())
}

fn handle_packet(client: &mut Client, writer: &mut ClientWriter, packet: Packet) -> Result<()> {
    // match packet {
    //     Packet::MapChunkResponse(_) | Packet::DeclareRecipesResponse(_) | Packet::LoginResponse(_) => {},
    //     _ => println!("{:?}", packet)
    // }
    match packet {
        Packet::KeepAliveResponse(x) => {
            let p = KeepAliveRequest {
                keep_alive_id: x.keep_alive_id,
            };
            info!("ping!");
            writer.send_packet(p)?;
        }
        Packet::ChatResponse(x) => {
            println!("{}", chat::parse_chat(x.message)?);
        }
        Packet::SuccessResponse(_) => {
            client.state = ConnectionState::Play;
        }
        Packet::CompressResponse(x) => {
            client.compression = x.threshold >= 0;
            writer.compression_threshold = Some(x.threshold.try_into()?);
        }
        _ => {}
    }

    Ok(())
}

fn read_packet(
    client: &mut Client,
    reader: &mut ClientReader,
    writer: &mut ClientWriter,
) -> Result<bool> {
    let packet_data =
        match protocol::read_packet_info(&reader.buffer, &mut reader.tmp, client.compression)? {
            Some(x) => x,
            None => return Ok(false),
        };
    let mut data = packet_data.data;
    let packet = protocol::just_deserialize(
        client.state,
        PacketDirection::S2C,
        packet_data.id,
        &mut data,
    )?;
    handle_packet(client, writer, packet)?;
    reader.buffer.advance(packet_data.total_size_original);

    Ok(true)
}

fn on_stdin_line(writer: &mut ClientWriter, line: String) -> Result<()> {
    let line = line.trim();
    writer.send_packet(ChatRequest { message: line })?;

    Ok(())
}

fn read_from_stdin(sender: Sender<String>, poller: &Poller) -> Result<()> {
    let mut stdin = stdin().lock();
    loop {
        let mut line = String::new();
        stdin.read_line(&mut line)?;
        sender.send(line)?;

        poller.notify()?;
    }
}
fn spawn_stdin_thread(poller: Arc<Poller>) -> Receiver<String> {
    let (sender, receiver) = channel();
    thread::spawn(move || {
        if let Err(e) = read_from_stdin(sender, &poller) {
            warn!("stdin reading died: {}", e);
        }
    });

    receiver
}

pub fn run(addr: SocketAddr) -> Result<()> {
    const SOCKET_KEY: usize = 0;

    let mut socket = TcpStream::connect(addr)?;
    socket.set_nonblocking(true)?;

    let poller = Arc::new(Poller::new()?);
    poller.add(&socket, Event::all(SOCKET_KEY))?;

    let mut client = Client::new();
    let mut reader = ClientReader::default();
    let mut writer = ClientWriter::default();

    let mut events = Vec::new();
    let mut buffer = [0; 4096];

    let receiver = spawn_stdin_thread(poller.clone());
    send_start(&mut writer)?;

    loop {
        events.clear();
        poller.wait(&mut events, None)?;

        for ev in &events {
            if ev.readable {
                let read = socket.read(&mut buffer)?;
                if read == 0 {
                    return Ok(());
                }
                reader.buffer.extend_from_slice(&buffer[..read]);

                while read_packet(&mut client, &mut reader, &mut writer)? {}
            }
            if ev.writable {
                let wrote = socket.write(&writer.buffer)?;
                writer.buffer.advance(wrote);
            }
        }

        while let Ok(line) = receiver.try_recv() {
            on_stdin_line(&mut writer, line)?;
        }

        poller.modify(
            &socket,
            Event {
                key: SOCKET_KEY,
                readable: true,
                writable: !writer.buffer.is_empty(),
            },
        )?;
    }
}
