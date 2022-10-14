#![feature(core_intrinsics)]

mod launchers;

use ansi_term::Color::{Cyan, Green, Purple};
use anyhow::anyhow;
use anyhow::{bail, Result};
use bumpalo::collections::Vec as BVec;
use bumpalo::Bump;
use chrono::Local;
use launchers::{get_access_token, AuthDataExt};
use melon::chat::parse_chat;
use melon::events::{EventSubscriber, Position, Trades};
use melon::nbt::Tag;
use melon::play::play;
use melon::protocol::{InventorySlot, InventorySlotData};
use melon::record::record_to_file;
use melon::{nbt, Enchantment, Item};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::intrinsics::unlikely;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Instant;

struct EventHandler {
    player_name: String,
    player_uuid: u128,
    player_position: Position,
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
        }
    }
}

#[derive(Debug)]
struct EnchantmentData {
    enchantment: Enchantment,
    level: u16,
}
#[derive(Debug)]
struct InventorySlotAttrs<'i> {
    enchantments: BVec<'i, EnchantmentData>,
}

#[derive(Debug)]
struct InventorySlotUnpacked<'i> {
    item_id: Item,
    count: u8,
    attrs: Option<InventorySlotAttrs<'i>>,
}

fn get_item_opt(item: Option<InventorySlot>) -> Option<InventorySlotData> {
    item?.data
}
fn deserialize_item_nbt<'b>(
    bump: &'b Bump,
    mut map: HashMap<&str, Tag>,
) -> Result<InventorySlotAttrs<'b>> {
    let mut enchantments = BVec::new_in(bump);
    if let Some(list) = map.remove("StoredEnchantments") {
        for i in list.list()? {
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
        }
    }

    if unlikely(!map.is_empty()) {
        println!("item nbt map not empty after deserializing: {:?}", map);
    }

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
        // println!("{:?}", trades);

        for i in trades.trades {
            println!("item1: {:?}", get_item(bump, buf, Some(i.input_item1))?);
            println!("item2: {:?}", get_item(bump, buf, i.input_item2)?);
            {
                let out = get_item(bump, buf, Some(i.output_item))?;
                println!("out:   {:?}\n", out);
            }

            bump.reset();
        }
        Ok(())
    }
}

fn record(config: Config, auth_data_ext: AuthDataExt, server: Option<&String>) -> Result<()> {
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
    let _ = ansi_term::enable_ansi_support();

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        bail!("no args supplied");
    }

    let config_text = fs::read_to_string("config.json")?;
    let config_json = serde_json::from_str(&config_text)?;
    let config = parse_config(config_json)?;
    let auth_data_ext = get_access_token(config.servers[config.default_server].profile)?;

    match args[1].as_str() {
        "record" => record(config, auth_data_ext, args.get(2))?,
        "playold" => {
            const DEFAULT_PACKET_FILE: &str = "packets.dune";
            let packet_file = args
                .get(2)
                .map(|x| x.as_str())
                .unwrap_or(DEFAULT_PACKET_FILE);

            let handler = Box::new(EventHandler::new());
            play(packet_file, handler)?;
        }
        "play" => unimplemented!(),
        _ => bail!("unknown command"),
    }

    Ok(())
}

fn main() -> Result<()> {
    let start = Instant::now();
    let result = main_impl();
    println!("execution took {:?}", start.elapsed());
    result
}
