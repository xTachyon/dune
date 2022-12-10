use crate::protocol::de::{Reader, MD};
use crate::protocol::PacketDirection;
use anyhow::bail;
use anyhow::Result;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::io::Write;

pub mod chat;
pub mod client;
mod data;
pub mod events;
mod game;
pub mod nbt;
pub mod play;
pub mod protocol;
pub mod record;
pub mod world;

pub use data::enchantments::Enchantment;
pub use data::items::Item;

struct DiskPacket<'p> {
    pub id: u32,
    pub direction: PacketDirection,
    pub data: &'p [u8],
}

impl<'p> DiskPacket<'p> {
    fn write<W: Write>(&self, mut writer: W) -> Result<()> {
        let size = 4 + 1 + self.data.len() as u32;
        // id + direction + size

        writer.write_all(&size.to_be_bytes())?;
        writer.write_all(&self.id.to_be_bytes())?;
        writer.write_all(&[self.direction as u8])?;
        writer.write_all(self.data)?;

        Ok(())
    }

    fn read(reader: &'p mut Reader) -> Result<DiskPacket<'p>> {
        let size: u32 = MD::deserialize(reader)?;
        let id: u32 = MD::deserialize(reader)?;

        let direction: u8 = MD::deserialize(reader)?;
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
        const SIZEOF_U32: usize = std::mem::size_of::<u32>();

        if buf.len() < SIZEOF_U32 {
            return false;
        }
        let size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize;
        size + SIZEOF_U32 <= buf.len()
    }
}

pub trait HashMapExt<K, V> {
    fn remove_err<Q: ?Sized>(&mut self, key: &Q) -> Result<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + fmt::Debug;
}
impl<K: Eq + Hash, V> HashMapExt<K, V> for HashMap<K, V> {
    fn remove_err<Q: ?Sized>(&mut self, key: &Q) -> Result<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + fmt::Debug,
    {
        let key = key.borrow();
        match self.remove(key) {
            Some(x) => Ok(x),
            None => bail!("unknown key `{:?}`", key),
        }
    }
}
