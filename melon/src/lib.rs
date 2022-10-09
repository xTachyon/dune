use crate::protocol::de::{Reader, MD};
use crate::protocol::PacketDirection;
use anyhow::Result;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::io::Write;
use anyhow::bail;

pub mod chat;
pub mod events;
mod game;
pub mod nbt;
pub mod play;
pub mod protocol;
pub mod record;

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

#[derive(Clone, Copy)]
pub struct ItemId(u16);

pub struct Item {
    name: &'static str,
    display_name: &'static str,
}

pub struct GameData {
    items_by_id: Vec<Item>,
    items_by_name: HashMap<&'static str, ItemId>,
}

impl GameData {
    pub fn load() -> GameData {
        #[derive(Deserialize)]
        struct JsonItem {
            name: &'static str,
            display_name: &'static str,
        }
        const ITEMS_JSON: &str = include_str!("data/items.json");

        let json_items: Vec<JsonItem> = serde_json::from_str(ITEMS_JSON).unwrap();

        let mut items_by_id = Vec::with_capacity(2048);
        items_by_id.push(Item {
            name: "bad_item",
            display_name: "BadItem",
        });

        for i in json_items {
            let item = Item {
                name: i.name,
                display_name: i.display_name,
            };
            items_by_id.push(item);
        }

        let mut count = 1;
        let mut items_by_name = HashMap::new();
        for i in items_by_id.iter().skip(1) {
            let id = ItemId(count);
            items_by_name.insert(i.name, id);

            count += 1;
        }

        GameData {
            items_by_id,
            items_by_name,
        }
    }

    pub fn item_by_name(&self, mut name: &str) -> Result<ItemId> {
        const PREFIX: &str = "minecraft:";
        if !name.starts_with(PREFIX) {
            bail!("expected item name to start with minecraft:, found {}", name);
        }
        name = &name[PREFIX.len()..];
        match self.items_by_name.get(name) {
            Some(x) => Ok(*x),
            None => bail!("item name {} not found", name)
        }
    }
}
