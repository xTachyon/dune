mod launchers;

use ansi_term::Color::{Cyan, Green, Purple};
use anyhow::{bail, Result};
use launchers::get_access_token;
use melon::chat::parse_chat;
use melon::events::{EventSubscriber, Position};
use melon::play::play;
use melon::record::record_to_file;
use std::env;
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

impl EventSubscriber for EventHandler {
    fn on_chat(&mut self, message: &str) -> Result<()> {
        // println!("chat: {}", message);
        let c = parse_chat(message)?;
        println!("{}", c.to_string());
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
}

fn main_impl() -> Result<()> {
    const DEFAULT_PACKET_FILE: &str = "packets.dune";

    let _ = ansi_term::enable_ansi_support();

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        bail!("no args supplied");
    }

    match args[1].as_str() {
        "record" => {
            // let server_address = "127.0.0.1:25566";
            let server_address = "play.runic-paradise.com:25565";
            let listen_addr = "0.0.0.0:25565";
            let auth_data_ext = get_access_token()?;

            let online_str = if auth_data_ext.online {
                "online"
            } else {
                "offline"
            };
            println!(
                "{}: {} ({})\n{}: {}\n{}: {}\n",
                Green.paint("minecraft profile"),
                Cyan.paint(auth_data_ext.name),
                Purple.paint(online_str),
                Green.paint("listening address"),
                Cyan.paint(listen_addr),
                Green.paint("server address   "),
                Cyan.paint(server_address)
            );

            record_to_file(
                listen_addr,
                server_address,
                auth_data_ext.data,
                DEFAULT_PACKET_FILE,
            )?;
        }
        "play" => {
            let handler = Box::new(EventHandler::new());
            play(DEFAULT_PACKET_FILE, handler)?;
        }
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
