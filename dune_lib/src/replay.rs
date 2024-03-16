use crate::events::{EventSubscriber, Position, UseEntity};
use crate::{Buffer, DiskPacket};
use anyhow::Result;
use dune_data::protocol;
use dune_data::protocol::v1_19_3::Packet;
use dune_data::protocol::ConnectionState;
use flate2::read::ZlibDecoder;
use log::warn;
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
        let mut data = disk_packet.data;
        let packet = protocol::v1_19_3::deserialize(
            self.state,
            disk_packet.direction,
            disk_packet.id,
            &mut data,
        )?;

        // println!("{:?}", packet);
        match packet {
            // Packet::SetProtocolRequest(p) => {
            //     self.state = match p.next_state {
            //         1 => ConnectionState::Status,
            //         2 => ConnectionState::Login,
            //         _ => unimplemented!(),
            //     };
            // }
            // Packet::SuccessResponse(p) => {
            //     self.state = ConnectionState::Play;
            //     self.handler.player_info(p.username, p.uuid)?;
            // }
            Packet::PlayerChatResponse(_) => self.handler.on_chat("who knows")?, // ??
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
            Packet::TradeListResponse(p) => self.handler.trades(p)?,
            Packet::UseEntityRequest(p) => self.handler.interact(UseEntity {
                entity_id: p.entity_id,
                kind: p.kind,
            })?,
            _ => {}
        }
        Ok(())
    }

    fn run(&mut self) -> Result<()> {
        let mut buffer = Buffer::default();
        let mut tmp = [0; 4096];
        let mut packet_count = 0u32;
        loop {
            let read = self.reader.read(&mut tmp)?;
            if read == 0 {
                break;

                // const SLEEP_DURATION: Duration = Duration::from_millis(20);
                // std::thread::sleep(SLEEP_DURATION);
                // continue;
            }
            buffer.extend_from_slice(&tmp[..read]);

            let mut data = buffer.as_slice();
            while DiskPacket::has_enough_bytes(data) {
                let disk_packet = DiskPacket::read(&mut data)?;
                if let Err(err) = self.do_packet(disk_packet) {
                    warn!("packet #{}. {:?}", packet_count, err);
                }
                packet_count += 1;
            }

            buffer.advance(data.as_ptr() as usize - buffer.as_ptr() as usize);
        }

        Ok(())
    }
}

pub fn play(in_path: &str, handler: Box<dyn EventSubscriber>) -> Result<()> {
    let mut player = TrafficPlayer::new(in_path, handler)?;
    player.run()?;

    Ok(())
}
