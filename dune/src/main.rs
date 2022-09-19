mod launchers;

use anyhow::Result;
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
        let c = parse_chat(message)?;
        println!("chat: {} => \n{:?}\n{}\n", message, c, c.to_string());
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
    // let file = std::fs::read(r#"C:\Users\andre\Downloads\bigtest.nbt"#).unwrap();
    // let mut data = file.as_slice();
    // let p = melon::nbt::read(&mut data).unwrap();
    // println!("{}", melon::nbt::pretty_print(&p)?);
    // return Ok(());
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("no args supplied");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "record" => {
            let auth_data = get_access_token()?;
            let server_address = "127.0.0.1:25566";

            record_to_file(server_address, auth_data, "packets.dune")?;
        }
        "play" => {
            let handler = Box::new(EventHandler::new());
            play("packets.dune", handler)?;
        }
        _ => eprintln!("unknown command"),
    }

    Ok(())
}

fn main() -> Result<()> {
    let start = Instant::now();
    let result = main_impl();
    println!("execution took {:?}", start.elapsed());
    result
}
