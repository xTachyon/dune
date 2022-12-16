use crate::protocol::de::MD;
use crate::protocol::v1_18_2::handshaking::SetProtocolRequest;
use crate::protocol::v1_18_2::login::LoginStartRequest;
use crate::protocol::v1_18_2::play::{ChatRequest, KeepAliveRequest};
use crate::protocol::varint::{write_varint, write_varint_serialize, VarintSerialized};
use crate::protocol::{self, ConnectionState, Packet, PacketDirection};
use crate::{chat, Buffer};
use anyhow::Result;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use log::info;
use polling::{Event, Poller};
use std::borrow::Borrow;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};

#[derive(Default)]
struct ClientWriter {
    buffer: Buffer,
    // crypt: Option<Aes128Cfb8>,
    tmp: Vec<u8>,
    tmp2: Vec<u8>,
    compression_threshold: Option<usize>,
}
#[derive(Default)]
struct ClientReader {
    buffer: Buffer,
    // crypt: Option<Aes128Cfb8>,
    tmp: Vec<u8>,
}
impl ClientWriter {
    fn send_packet<'x, P, Q>(&mut self, packet: P) -> Result<()>
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

    let p = LoginStartRequest {
        username: "TheTachyon",
    };
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

            writer.send_packet(ChatRequest { message: "wow" })?;
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
        match protocol::read_packet_info(&reader.buffer, client.compression, &mut reader.tmp)? {
            Some(x) => x,
            None => return Ok(false),
        };
    let mut data = packet_data.get_data(&reader.buffer, &reader.tmp);
    let packet = protocol::just_deserialize(
        PacketDirection::S2C,
        client.state,
        packet_data.id,
        &mut data,
    )?;
    handle_packet(client, writer, packet)?;
    reader.buffer.advance(packet_data.total_size_original);

    Ok(true)
}

pub fn run(addr: SocketAddr) -> Result<()> {
    const SOCKET_KEY: usize = 0;

    let mut socket = TcpStream::connect(addr)?;
    socket.set_nonblocking(true)?;

    let poller = Poller::new()?;
    poller.add(&socket, Event::all(SOCKET_KEY))?;

    let mut client = Client::new();
    let mut reader = ClientReader::default();
    let mut writer = ClientWriter::default();

    let mut events = Vec::new();
    let mut buffer = [0; 4096];

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
