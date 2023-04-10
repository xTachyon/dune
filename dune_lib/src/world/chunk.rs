use std::collections::HashMap;

use crate::nbt::Tag;
use crate::Item;
use crate::{chat::parse_chat, events::PositionInt, nbt, HashMapExt};
use anyhow::Result;
use bumpalo::collections::Vec as BVec;
use bumpalo::Bump;

// https://minecraft.fandom.com/wiki/Chunk_format
// Why is Fandom so annoying??

pub struct Sign {
    pub text: [String; 4],
}
pub struct BrewingStand {
    pub fuel: i8,
    pub brew_time: i16,
}
#[derive(Debug)]

pub struct ItemSlot {
    pub item: Item,
    pub count: u8,
}
pub struct Chest {
    pub items: Vec<ItemSlot>,
}

pub enum BlockEntityKind {
    Sign(Sign),
    BrewingStand(BrewingStand),
    Chest(Chest),
    Bed,
    Bell,
}
pub struct BlockEntity {
    pub position: PositionInt,
    pub kind: BlockEntityKind,
}

pub struct Chunk {
    pub version: i32,
    pub block_entities: Vec<BlockEntity>,
}

pub fn read_block_entity(
    id: &str,
    mut nbt: HashMap<&str, Tag>,
    bump: &Bump,
) -> Result<Option<BlockEntityKind>> {
    let r = match id {
        "minecraft:sign" => {
            let mut get_text = |key: &str| -> Result<String> {
                let json = nbt.remove_err(key)?.string()?;
                let r = parse_chat(json)?.to_string();
                Ok(r)
            };
            let text = [
                get_text("Text1")?,
                get_text("Text2")?,
                get_text("Text3")?,
                get_text("Text4")?,
            ];

            BlockEntityKind::Sign(Sign { text })
        }
        "minecraft:brewing_stand" => {
            // TODO: items ignored
            let fuel = nbt.remove_err("Fuel")?.byte()?;
            let brew_time = nbt.remove_err("BrewTime")?.short()?;

            BlockEntityKind::BrewingStand(BrewingStand { fuel, brew_time })
        }
        "minecraft:chest" => {
            let items_in = match nbt.remove("Items") {
                Some(x) => x.list()?,
                None => BVec::new_in(bump),
            };
            let mut items = Vec::with_capacity(items_in.len());
            for i in items_in {
                let mut i = i.compound()?;

                let id = i.remove_err("id")?.string()?;
                let item = Item::from_str_id(id)?;
                let count = i.remove_err("Count")?.byte()?.try_into()?;

                items.push(ItemSlot { item, count });
            }

            BlockEntityKind::Chest(Chest { items })
        }
        "minecraft:bed" => BlockEntityKind::Bed,
        "minecraft:bell" => BlockEntityKind::Bell,
        _ => {
            // warn!("unknown block entity `{}`", id);
            return Ok(None);
        }
    };
    Ok(Some(r))
}

pub fn read_chunk(buf: &[u8], bump: &Bump) -> Result<Chunk> {
    let root = nbt::read(buf, bump)?;
    let mut root = root.tag.compound()?;
    let version = root.remove_err("DataVersion")?.int()?;

    let block_entities_nbt = if version >= 2860 {
        // 2860 - 1.18
        // no clue when the format changed actually
        root.remove_err("block_entities")?.list()?
    } else {
        let mut level = root.remove_err("Level")?.compound()?;
        match level.remove("TileEntities") {
            Some(x) => x.list()?,
            None => BVec::new_in(bump),
        }
    };

    let mut block_entities = Vec::with_capacity(block_entities_nbt.len());
    for i in block_entities_nbt {
        let mut i = i.compound()?;
        let id = i.remove_err("id")?.string()?;

        let x = i.remove_err("x")?.int()?;
        let y = i.remove_err("y")?.int()?;
        let z = i.remove_err("z")?.int()?;

        let kind = match read_block_entity(id, i, bump)? {
            Some(x) => x,
            None => continue,
        };

        let position = PositionInt { x, y, z };
        block_entities.push(BlockEntity { position, kind });
    }

    Ok(Chunk {
        version,
        block_entities,
    })
}
