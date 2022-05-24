use crate::protocol::de::MinecraftDeserialize;
use crate::protocol::PacketDirection;
use anyhow::Result;
use std::io::{Read, Write};

pub mod events;
mod game;
pub mod player;
mod protocol;
pub mod recorder;

struct DiskPacket {
    pub id: u32,
    pub direction: PacketDirection,
    pub data: Vec<u8>, // todo: make it copy free
}

impl DiskPacket {
    fn write<W: Write>(&self, mut writer: W) -> Result<()> {
        let size = 4 + 1 + self.data.len() as u32;
        writer.write_all(&size.to_be_bytes())?;
        writer.write_all(&self.id.to_be_bytes())?;
        writer.write_all(&[self.direction as u8])?;
        writer.write_all(&self.data)?;

        Ok(())
    }

    fn read<R: Read>(mut reader: R) -> Result<DiskPacket> {
        let size: u32 = MinecraftDeserialize::deserialize(&mut reader)?;
        let id: u32 = MinecraftDeserialize::deserialize(&mut reader)?;
        let direction: u8 = MinecraftDeserialize::deserialize(&mut reader)?;
        let direction = PacketDirection::try_from(direction)?;
        let mut data = vec![0; size as usize - 4 - 1];
        reader.read_exact(&mut data)?;

        Ok(DiskPacket {
            id,
            direction,
            data,
        })
    }

    fn has_enough_bytes(buf: &[u8]) -> bool {
        if buf.len() < 4 {
            return false;
        }
        let size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize;
        size + 4 <= buf.len()
    }
}
