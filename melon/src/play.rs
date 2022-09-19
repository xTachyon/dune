use crate::events::{EventSubscriber, Position};
use crate::protocol::de::Reader;
use crate::protocol::{ConnectionState, Packet};
use crate::{protocol, DiskPacket};
use anyhow::Result;
use flate2::read::ZlibDecoder;
use std::fs::File;
use std::io::Read;

struct TrafficPlayer {
    reader: ZlibDecoder<File>,
    handler: Box<dyn EventSubscriber>,
    state: ConnectionState,
}

impl TrafficPlayer {
    fn new(in_path: &str, handler: Box<dyn EventSubscriber>) -> Result<TrafficPlayer> {
        let reader = File::open(in_path)?;
        let reader = ZlibDecoder::new(reader);

        Ok(TrafficPlayer {
            reader,
            handler,
            state: ConnectionState::Handshaking,
        })
    }

    fn do_packet(&mut self, disk_packet: DiskPacket) -> Result<()> {
        let mut reader = Reader::new(disk_packet.data);
        let packet = protocol::just_deserialize(
            disk_packet.direction,
            self.state,
            disk_packet.id,
            &mut reader,
        )?;

        println!("{:?}", packet);
        match packet {
            Packet::SetProtocolRequest(p) => {
                self.state = match p.next_state {
                    1 => ConnectionState::Status,
                    2 => ConnectionState::Login,
                    _ => unimplemented!(),
                };
            }
            Packet::SuccessResponse(p) => {
                self.state = ConnectionState::Play;
                self.handler
                    .player_info(p.username.get(disk_packet.data), p.uuid)?;
            }
            Packet::ChatResponse(p) => self.handler.on_chat(p.message.get(disk_packet.data))?,
            Packet::PositionRequest(p) => self.handler.position(Position {
                x: p.x,
                y: p.y,
                z: p.z,
            })?,
            Packet::PositionResponse(p) => self.handler.position(Position {
                x: p.x,
                y: p.y,
                z: p.z,
            })?,
            _ => {}
        }
        Ok(())
    }

    fn run(&mut self) -> Result<()> {
        let mut buffer = Vec::with_capacity(64 * 1024);
        let mut tmp = [0; 64 * 1024];
        loop {
            let read = self.reader.read(&mut tmp)?;
            if read == 0 {
                return Ok(());

                // const SLEEP_DURATION: Duration = Duration::from_millis(20);
                // std::thread::sleep(SLEEP_DURATION);
                // continue;
            }
            buffer.extend_from_slice(&tmp[..read]);

            let mut cursor = Reader::new(&buffer);
            while DiskPacket::has_enough_bytes(&buffer[cursor.offset()..]) {
                let disk_packet = DiskPacket::read(&mut cursor)?;
                self.do_packet(disk_packet)?;
            }

            buffer.drain(..cursor.offset());
        }
    }
}

pub fn play(in_path: &str, handler: Box<dyn EventSubscriber>) -> Result<()> {
    let mut player = TrafficPlayer::new(in_path, handler)?;
    player.run()?;
    Ok(())
}
