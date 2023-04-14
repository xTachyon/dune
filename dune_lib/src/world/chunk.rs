use std::collections::HashMap;

use crate::nbt::Tag;
use crate::Item;
use crate::{chat::parse_chat, events::PositionInt, nbt, HashMapExt};
use anyhow::Result;
use bumpalo::collections::Vec as BVec;
use bumpalo::Bump;

// https://minecraft.fandom.com/wiki/Chunk_format
// Why is Fandom so annoying??

#[derive(Debug)]
pub struct Book<'x> {
    pub title: Option<&'x str>,
    pub author: Option<&'x str>,
    pub pages: Vec<String>,
}

#[derive(Debug)]
pub enum ItemSlotExtra<'x> {
    Book(Book<'x>),
    Unknown(HashMap<&'x str, Tag<'x>>),
}
#[derive(Debug)]
pub struct ItemSlot<'x> {
    pub item: Item,
    pub count: u8,
    pub extra: Option<ItemSlotExtra<'x>>,
}

pub struct Sign {
    pub text: [String; 4],
}
// pub struct BrewingStand {
//     pub fuel: i8,
//     pub brew_time: i16,
// }
pub struct Storage<'x> {
    pub items: Vec<ItemSlot<'x>>,
}

pub enum BlockEntityKind<'x> {
    Sign(Sign),
    // BrewingStand(BrewingStand),
    Storage(Storage<'x>),
    Bed,
    Bell,
}
pub struct BlockEntity<'x> {
    pub position: PositionInt,
    pub kind: BlockEntityKind<'x>,
}

pub struct Chunk<'x> {
    pub block_entities: Vec<BlockEntity<'x>>,
}
fn read_item_extra<'x>(
    mut nbt: HashMap<&'x str, Tag<'x>>,
    item: Item,
) -> Result<ItemSlotExtra<'x>> {
    let r = match item {
        Item::WritableBook | Item::WrittenBook => {
            let pages_raw = nbt.remove_err("pages")?.list()?;
            let mut pages = Vec::with_capacity(pages_raw.len());
            let is_written = item == Item::WrittenBook;

            for i in pages_raw {
                let page = i.string()?;
                let page = if is_written {
                    parse_chat(page)?.to_string()
                } else {
                    page.to_string()
                };
                pages.push(page);
            }

            let (title, author) = if is_written {
                let title = nbt.remove_err("title")?.string()?;
                let author = nbt.remove_err("author")?.string()?;
                (Some(title), Some(author))
            } else {
                (None, None)
            };

            ItemSlotExtra::Book(Book {
                title,
                author,
                pages,
            })
        }
        _ => ItemSlotExtra::Unknown(nbt),
    };
    Ok(r)
}
fn read_item<'x>(mut nbt: HashMap<&str, Tag<'x>>) -> Result<ItemSlot<'x>> {
    let id = nbt.remove_err("id")?.string()?;
    let item = Item::from_str_id(id)?;
    let count = nbt.remove_err("Count")?.byte()?.try_into()?;

    let extra = match nbt.remove("tag") {
        Some(x) => Some(read_item_extra(x.compound()?, item)?),
        None => None,
    };

    Ok(ItemSlot { item, count, extra })
}
fn read_storage<'x>(mut nbt: HashMap<&str, Tag<'x>>, bump: &Bump) -> Result<BlockEntityKind<'x>> {
    let items_in = match nbt.remove("Items") {
        Some(x) => x.list()?,
        None => BVec::new_in(bump),
    };
    let mut items = Vec::with_capacity(items_in.len());
    for i in items_in {
        let i = i.compound()?;
        let item = read_item(i)?;

        items.push(item);
    }

    Ok(BlockEntityKind::Storage(Storage { items }))
}
pub fn read_block_entity<'x>(
    id: &str,
    mut nbt: HashMap<&str, Tag<'x>>,
    bump: &Bump,
) -> Result<Option<BlockEntityKind<'x>>> {
    let id = match id.strip_prefix("minecraft:") {
        Some(x) => x,
        None => return Ok(None),
    };
    let r = match id {
        "sign" => {
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
        // "brewing_stand" => {
        //     // TODO: items ignored
        //     let fuel = nbt.remove_err("Fuel")?.byte()?;
        //     let brew_time = nbt.remove_err("BrewTime")?.short()?;

        //     BlockEntityKind::BrewingStand(BrewingStand { fuel, brew_time })
        // }
        "chest" | "trapped_chest" | "barrel" | "hopper" | "dispenser" | "dropper" | "furnace"
        | "blast_furnace" | "smoker" | "brewing_stand" => read_storage(nbt, bump)?,
        "bed" => BlockEntityKind::Bed,
        "bell" => BlockEntityKind::Bell,
        _ => {
            // warn!("unknown block entity `{}`", id);
            return Ok(None);
        }
    };
    Ok(Some(r))
}

pub fn read_chunk<'x>(buf: &'x [u8], bump: &'x Bump) -> Result<Chunk<'x>> {
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

    Ok(Chunk { block_entities })
}
