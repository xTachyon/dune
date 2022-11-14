#![feature(core_intrinsics)]

mod launchers;

use ansi_term::Color::{Cyan, Green, Purple};
use anyhow::anyhow;
use anyhow::{bail, Result};
use bumpalo::collections::String as BString;
use bumpalo::collections::Vec as BVec;
use bumpalo::Bump;
use chrono::Local;
use clap::Parser;
use dune_lib::anvil::{Region, CHUNKS_PER_REGION};
use dune_lib::chat::parse_chat;
use dune_lib::events::{EventSubscriber, Position, Trades, UseEntity, UseEntityKind};
use dune_lib::nbt::{RootTag, Tag};
use dune_lib::play::play;
use dune_lib::protocol::{InventorySlot, InventorySlotData};
use dune_lib::record::record_to_file;
use dune_lib::{nbt, Enchantment, Item};
use launchers::{get_access_token, AuthDataExt};
use log::{info, warn, LevelFilter};
use serde_derive::Deserialize;
use simple_logger::SimpleLogger;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::{self, Write as FmtWrite};
use std::fs::{self, File};
use std::hash::Hash;
use std::intrinsics::unlikely;
use std::io::BufWriter;
use std::io::Write;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Instant;

///Tool for replaying saves with game input
#[derive(Parser)]
#[command(author,version,about,long_about=None)]
struct Args {
    #[command(subcommand)]
    action: Action,
}
#[derive(clap::Subcommand)]
enum Action {
    Record { option: Option<String> },
    Replay { option: String },
    Signs { path: String },
}
struct EventHandler {
    player_name: String,
    player_uuid: u128,
    player_position: Position,
    last_entity_interact: Option<Position>,
}

impl EventHandler {
    fn new() -> EventHandler {
        EventHandler {
            player_name: "".to_string(),
            player_uuid: 0,
            player_position: Position {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            last_entity_interact: None,
        }
    }
}

// TODO: remove
#[allow(unused)]
#[derive(Debug)]
struct EnchantmentData {
    enchantment: Enchantment,
    level: u8,
}

// TODO: remove
#[allow(unused)]
#[derive(Debug)]
struct InventorySlotAttrs<'i> {
    enchantments: BVec<'i, EnchantmentData>,
}

// TODO: remove
#[allow(unused)]
#[derive(Debug)]
struct InventorySlotUnpacked<'i> {
    item_id: Item,
    count: u8,
    attrs: Option<InventorySlotAttrs<'i>>,
}

fn get_item_opt(item: Option<InventorySlot>) -> Option<InventorySlotData> {
    item?.data
}
fn check_map_empty(map: HashMap<&str, Tag>) {
    if unlikely(!map.is_empty()) {
        warn!("item nbt map not empty after deserializing: {:?}", map);
    }
}
fn deserialize_item_nbt<'b>(
    bump: &'b Bump,
    mut map: HashMap<&str, Tag>,
) -> Result<InventorySlotAttrs<'b>> {
    let mut enchantments = BVec::new_in(bump);
    if let Some(list) = map.remove("StoredEnchantments") {
        let list = list.list()?;
        enchantments.reserve(list.len());

        for i in list {
            let mut i = i.compound()?;

            let level = i
                .remove("lvl")
                .ok_or_else(|| anyhow!("'lvl' attr not found"))?
                .short()?;
            let id = i
                .remove("id")
                .ok_or_else(|| anyhow!("'id' attr not found"))?
                .string()?;
            let id = Enchantment::from(id)?;

            enchantments.push(EnchantmentData {
                enchantment: id,
                level: level.try_into()?,
            });

            check_map_empty(i);
        }
    }

    check_map_empty(map);
    Ok(InventorySlotAttrs { enchantments })
}
fn get_item<'b>(
    bump: &'b Bump,
    buf: &[u8],
    item: Option<InventorySlot>,
) -> Result<Option<InventorySlotUnpacked<'b>>> {
    let item = match get_item_opt(item) {
        Some(x) => x,
        None => return Ok(None),
    };
    let attrs = match item.nbt {
        Some(buffer) => {
            let buffer = buffer.get(buf);
            let r = nbt::read(buffer, bump)?;
            let attrs = deserialize_item_nbt(bump, r.tag.compound()?)?;
            Some(attrs)
        }
        None => None,
    };

    let id = item.item_id.try_into()?;
    let item_id = Item::from_1_18_2(id)?;
    Ok(Some(InventorySlotUnpacked {
        item_id,
        count: item.count,
        attrs,
    }))
}

fn to_roman(number: u8) -> &'static str {
    match number {
        0 => "",
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        _ => unreachable!(),
    }
}

macro_rules! some_or_return {
    ($e:expr) => {
        match $e {
            Some(x) => x,
            None => return Ok(()),
        }
    };
}
fn print_item(out: &mut BString, name: &str, item: Option<InventorySlotUnpacked>) -> Result<()> {
    let x = some_or_return!(item);
    write!(out, "{}: {:>2}x {:?}", name, x.count, x.item_id)?;

    if let Some(attrs) = x.attrs {
        write!(out, "(")?;
        for i in attrs.enchantments {
            write!(out, "{:?}", i.enchantment)?;
            if i.level != 0 {
                write!(out, " {}", to_roman(i.level))?;
            }
        }
        write!(out, ")")?;
    }

    writeln!(out)?;

    Ok(())
}

impl EventSubscriber for EventHandler {
    fn on_chat(&mut self, message: &str) -> Result<()> {
        // println!("chat: {}", message);
        let c = parse_chat(message)?;
        println!("{}", c);
        Ok(())
    }
    fn player_info(&mut self, name: &str, uuid: u128) -> Result<()> {
        self.player_name = name.to_string();
        self.player_uuid = uuid;
        Ok(())
    }
    fn position(&mut self, pos: Position) -> Result<()> {
        self.player_position = pos;
        Ok(())
    }
    fn trades(&mut self, trades: Trades) -> Result<()> {
        let buf = trades.buffer;
        let trades = trades.packet;
        let bump = &mut Bump::with_capacity(4096);

        let last_entity = self
            .last_entity_interact
            .ok_or_else(|| anyhow!("use entity wasn't set before using it"))?;

        let out = &mut BString::with_capacity_in(1024, bump);
        writeln!(out, "trades at {:?}:", last_entity)?;
        for i in trades.trades {
            let in1 = get_item(bump, buf, Some(i.input_item1))?;
            print_item(out, "in1", in1)?;

            let in2 = get_item(bump, buf, i.input_item2)?;
            print_item(out, "in2", in2)?;

            let out_item = get_item(bump, buf, Some(i.output_item))?;
            print_item(out, "out", out_item)?;

            writeln!(out)?;
        }
        writeln!(out, "------------------------------------------------------------------------------------------------------------")?;

        info!("{}", out);
        Ok(())
    }
    fn interact(&mut self, use_entity: UseEntity) -> Result<()> {
        if let UseEntityKind::InteractAt(coords) = use_entity.kind {
            let position = Position {
                x: coords.x as f64 + self.player_position.x,
                y: coords.y as f64 + self.player_position.y,
                z: coords.z as f64 + self.player_position.z,
            };
            self.last_entity_interact = Some(position);
        }
        Ok(())
    }
}
// /summon minecraft:villager ~ ~ ~ {VillagerData:{type:"minecraft:plains",profession:"minecraft:mason",level:2}}

fn record(config: Config, auth_data_ext: AuthDataExt, server: Option<String>) -> Result<()> {
    let server = match server {
        Some(name) => match config.servers.iter().find(|x| x.name == name) {
            Some(x) => x,
            None => bail!("unknown server {}", name),
        },
        None => &config.servers[config.default_server],
    };
    loop {
        let listen_addr = "0.0.0.0:25565";

        let online_str = if auth_data_ext.online {
            "online"
        } else {
            "offline"
        };
        println!(
            "{}: {} ({})\n{}: {}\n{}: {} ({})\n",
            Green.paint("minecraft profile"),
            Cyan.paint(&auth_data_ext.name),
            Purple.paint(online_str),
            Green.paint("listening address"),
            Cyan.paint(listen_addr),
            Green.paint("server           "),
            Cyan.paint(server.name),
            Purple.paint(server.address.to_string())
        );

        let packet_file = format!(
            "saves/{}_{}.dune",
            server.name,
            Local::now().format("%Y-%m-%d_%H-%M-%S")
        );
        fs::write("saves/last.txt", &packet_file)?;

        record_to_file(
            listen_addr,
            server.address,
            auth_data_ext.data.clone(),
            &packet_file,
        )?;

        println!("saved to {}", packet_file);
    }
}

trait HashMapExt<K, V> {
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

fn print_signs_impl(
    root: RootTag,
    max: &mut usize,
    signs_count: &mut usize,
    out: &mut BufWriter<File>,
) -> Result<()> {
    let mut root = root.tag.compound()?;
    let data_version = root.remove_err("DataVersion")?.int()?;
    let block_entities = if data_version >= 2975 {
        // 2975 - 1.18.2
        // no clue when the format changed actually
        root.remove_err("block_entities")?.list()?
    } else {
        let mut level = root.remove_err("Level")?.compound()?;
        match level.remove("TileEntities") {
            Some(x) => x.list()?,
            None => return Ok(()),
        }
    };

    for i in block_entities {
        let mut i = i.compound()?;
        let id = i.remove_err("id")?.string()?;
        if id != "minecraft:sign" {
            continue;
        }

        let x = i.remove_err("x")?.int()?;
        let y = i.remove_err("y")?.int()?;
        let z = i.remove_err("z")?.int()?;

        let mut get_text = |key: &str| -> Result<String> {
            let json = i.remove_err(key)?.string()?;
            let r = parse_chat(json)?.to_string();
            Ok(r)
        };
        let text = [
            get_text("Text1")?,
            get_text("Text2")?,
            get_text("Text3")?,
            get_text("Text4")?,
        ];

        if text.iter().all(String::is_empty) {
            continue;
        }
        *max = text.iter().map(String::len).max().unwrap_or(*max);

        let dashes80 =
            "--------------------------------------------------------------------------------";
        writeln!(
            out,
            "{},{},{}\n{:^80}\n{:^80}\n{:^80}\n{:^80}\n{}\n",
            x, y, z, text[0], text[1], text[2], text[3], dashes80
        )?;
        *signs_count += 1;
    }

    Ok(())
}

fn print_signs(path: String) -> Result<()> {
    let mut tmp = Vec::new();
    let bump = &mut Bump::with_capacity(128 * 1024); // 128kb based on experimentation

    let mut max = 0;
    let files: Vec<_> = fs::read_dir(path)?.collect();
    let mut out = BufWriter::with_capacity(64 * 1024, File::create("out.txt")?);
    let files_count = files.len();
    for (index, file) in files.into_iter().enumerate() {
        let time = Instant::now();
        let file = file?;

        let p = file.path();
        let mut region = Region::load(&p, false)?;
        let mut signs_count = 0;
        for i in 0..CHUNKS_PER_REGION {
            let data = region.get_chunk(&mut tmp, i)?;
            if data.is_empty() {
                continue;
            }

            let root = nbt::read(data, bump)?;
            print_signs_impl(root, &mut max, &mut signs_count, &mut out)?;
        }

        println!(
            "{:<15} => {:>4}/{}, {:>4} signs, {:?}",
            p.file_name().unwrap_or(p.as_os_str()).to_string_lossy(),
            index + 1,
            files_count,
            signs_count,
            time.elapsed(),
        );

        bump.reset();
    }
    println!("max={}", max);

    Ok(())
}

#[derive(Deserialize)]
struct ConfigJsonServer<'x> {
    name: &'x str,
    profile: &'x str,
    address: &'x str,
}

#[derive(Deserialize)]
struct ConfigJson<'x> {
    default_server: &'x str,
    #[serde(borrow)]
    servers: Vec<ConfigJsonServer<'x>>,
}

struct ConfigServer<'x> {
    name: &'x str,
    profile: &'x str,
    address: SocketAddr,
}

struct Config<'x> {
    servers: Vec<ConfigServer<'x>>,
    default_server: usize,
}

fn parse_config(input: ConfigJson) -> Result<Config> {
    let mut servers: Vec<ConfigServer> = Vec::new();
    let mut default_server_index = None;

    for (index, s) in input.servers.into_iter().enumerate() {
        let addr = if s.address.contains(':') {
            s.address.to_string()
        } else {
            format!("{}:25565", s.address)
        };
        let addr = addr.to_socket_addrs()?.into_iter().next().unwrap();

        if s.name == input.default_server {
            default_server_index = Some(index);
        }

        if servers.iter().any(|x| x.name == s.name) {
            bail!("duplicated server name: {}", s.name);
        }
        servers.push(ConfigServer {
            name: s.name,
            profile: s.profile,
            address: addr,
        });
    }
    let default_server = match default_server_index {
        Some(x) => x,
        None => bail!("default server was not set"),
    };
    Ok(Config {
        servers,
        default_server,
    })
}

fn main_impl() -> Result<()> {
    let arguments = Args::parse();

    let config_text = fs::read_to_string("config.json")?;
    let config_json = serde_json::from_str(&config_text)?;
    let config = parse_config(config_json)?;
    let auth_data_ext = get_access_token(config.servers[config.default_server].profile)?;

    match arguments.action {
        Action::Record { option } => record(config, auth_data_ext, option),
        Action::Replay { option } => {
            let handler = Box::new(EventHandler::new());
            play(&option, handler)
        }
        Action::Signs { path } => print_signs(path),
    }
}

fn main() -> Result<()> {
    let _ = SimpleLogger::new().with_level(LevelFilter::Debug).init();
    #[cfg(windows)]
    let _ = ansi_term::enable_ansi_support();

    // std::thread::spawn(|| {
    //     let s = &mut String::new();
    //     let stdin = &mut stdin().lock();
    //     loop {
    //         stdin.read_line(s).unwrap();
    //         println!("{}", s);
    //         s.clear();
    //     }
    // });

    let start = Instant::now();
    let result = main_impl();
    println!("execution took {:?}", start.elapsed());
    result
}
