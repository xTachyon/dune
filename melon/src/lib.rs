use crate::protocol::de::{MinecraftDeserialize, Reader};
use crate::protocol::PacketDirection;
use anyhow::Result;
use std::io::Write;

pub mod events;
mod game;
pub mod nbt;
pub mod player;
mod protocol;
pub mod recorder;

struct DiskPacket<'p> {
    pub id: u32,
    pub direction: PacketDirection,
    pub data: &'p [u8],
}

impl<'p> DiskPacket<'p> {
    fn write<W: Write>(&self, mut writer: W) -> Result<()> {
        let size = 4 + 1 + self.data.len() as u32;
        writer.write_all(&size.to_be_bytes())?;
        writer.write_all(&self.id.to_be_bytes())?;
        writer.write_all(&[self.direction as u8])?;
        writer.write_all(self.data)?;

        Ok(())
    }

    fn read(mut reader: &'p mut Reader) -> Result<DiskPacket<'p>> {
        let size: u32 = MinecraftDeserialize::deserialize(&mut reader)?;
        let id: u32 = MinecraftDeserialize::deserialize(&mut reader)?;
       
        let direction: u8 = MinecraftDeserialize::deserialize(&mut reader)?;
        let direction = PacketDirection::try_from(direction)?;
       
        let data = reader.read_range_size(size as usize - 4 - 1)?;
        let data = reader.get_buf_from(data.start as usize..data.end as usize)?;

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
