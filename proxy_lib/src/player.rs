use crate::events::{ChatEvent, EventSubscriber};
use crate::protocol::de::Reader;
use crate::protocol::v1_18_2::play::ChatResponse;
use crate::protocol::{ConnectionState, Packet};
use crate::{protocol, DiskPacket};
use anyhow::Result;
use std::fs::File;
use std::io::Read;

struct TrafficPlayer {
    reader: File,
    handler: Box<dyn EventSubscriber>,
    state: ConnectionState,
}

impl TrafficPlayer {
    fn new(in_path: &str, handler: Box<dyn EventSubscriber>) -> Result<TrafficPlayer> {
        let reader = File::open(in_path)?;

        Ok(TrafficPlayer {
            reader,
            handler,
            state: ConnectionState::Handshaking,
        })
    }

    fn do_chat(&mut self, chat: ChatResponse) -> Result<()> {
        let event = ChatEvent {
            message: chat.message.to_string(),
        };
        self.handler.on_chat(event)?;
        Ok(())
    }

    fn do_packet(&mut self, disk_packet: DiskPacket) -> Result<()> {
        let mut reader = Reader::new(&disk_packet.data);
        let packet = protocol::just_deserialize(
            disk_packet.direction,
            self.state,
            disk_packet.id,
            &mut reader,
        )?;

        // println!("{:?}", packet);
        match packet {
            Packet::SetProtocolRequest(x) => {
                self.state = match x.next_state {
                    1 => ConnectionState::Status,
                    2 => ConnectionState::Login,
                    _ => unimplemented!(),
                };
            }
            Packet::SuccessResponse(_) => {
                self.state = ConnectionState::Play;
            }
            Packet::ChatResponse(x) => self.do_chat(x)?,
            _ => {}
        }
        Ok(())
    }

    fn run(&mut self) -> Result<()> {
        let mut buffer = Vec::new();
        let mut tmp = [0; 64 * 1024];
        loop {
            let read = self.reader.read(&mut tmp)?;
            if read == 0 {
                break;
            }
            buffer.extend_from_slice(&tmp[..read]);

            let mut cursor = Reader::new(&buffer);
            while DiskPacket::has_enough_bytes(&buffer[cursor.offset()..]) {
                let disk_packet = DiskPacket::read(&mut cursor)?;
                self.do_packet(disk_packet)?;
            }

            buffer.drain(..cursor.offset());
        }
        Ok(())
    }
}

pub fn play(in_path: &str, handler: Box<dyn EventSubscriber>) -> Result<()> {
    let mut player = TrafficPlayer::new(in_path, handler)?;
    player.run()?;
    Ok(())
}
