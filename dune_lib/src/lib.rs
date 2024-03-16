pub mod chat;
pub mod client;
pub mod events;
pub mod record;
pub mod replay;
pub mod world;

use anyhow::bail;
use anyhow::Result;
pub use dune_data::enchantments::Enchantment;
pub use dune_data::items::Item;
use dune_data::protocol::de::MemoryExt;
use dune_data::protocol::de::MD;
use dune_data::protocol::PacketDirection;
use dune_data::protocol::PacketId;
use slice_ring_buffer::SliceRingBuffer;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::io;
use std::io::Write;
use std::ops::Deref;
use std::ops::DerefMut;

struct DiskPacket<'p> {
    pub id: PacketId,
    pub direction: PacketDirection,
    pub data: &'p [u8],
}

impl<'p> DiskPacket<'p> {
    fn write<W: Write>(&self, mut writer: W) -> Result<()> {
        let size = 4 + 1 + self.data.len() as u32;
        // id + direction + size

        writer.write_all(&size.to_be_bytes())?;
        writer.write_all(&self.id.0.to_be_bytes())?;
        writer.write_all(&[self.direction as u8])?;
        writer.write_all(self.data)?;

        Ok(())
    }

    fn read(reader: &'p mut &[u8]) -> Result<DiskPacket<'p>> {
        let size: u32 = MD::deserialize(reader)?;
        let id: u32 = MD::deserialize(reader)?;

        let direction: u8 = MD::deserialize(reader)?;
        let direction = PacketDirection::try_from(direction)?;

        let data = reader.read_mem(size as usize - 4 - 1)?;

        Ok(DiskPacket {
            id: PacketId(id),
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
        Q: Hash + Eq + fmt::Display;
}
impl<K: Eq + Hash, V> HashMapExt<K, V> for HashMap<K, V> {
    fn remove_err<Q: ?Sized>(&mut self, key: &Q) -> Result<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + fmt::Display,
    {
        let key = key.borrow();
        match self.remove(key) {
            Some(x) => Ok(x),
            None => bail!("unknown key `{}`", key),
        }
    }
}

#[derive(Default)]
pub(crate) struct Buffer(SliceRingBuffer<u8>);
impl Buffer {
    fn advance(&mut self, size: usize) {
        let truncate_to = self.len() - size;
        self.truncate_front(truncate_to);
    }
}
impl Write for Buffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
impl Deref for Buffer {
    type Target = SliceRingBuffer<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
