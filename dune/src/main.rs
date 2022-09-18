use anyhow::Result;
use melon::events::{EventSubscriber, Position};
use melon::player::play;
use melon::recorder::{record_to_file, AuthData};
use serde_derive::Deserialize;
use std::env;
use std::fs::File;

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
        println!("chat: {}", message);
        Ok(())
    }
    fn player_info(&mut self, name: &str, uuid: u128) -> Result<()> {
        self.player_name = name.to_string();
        self.player_uuid = uuid;
        Ok(())
    }
    fn position(&mut self, pos: Position) -> Result<()> {
        if pos != self.player_position {
            println!("{:?}", pos);
        }
        self.player_position = pos;
        Ok(())
    }
}

fn get_access_token_tlauncher() -> Result<AuthData> {
    let path = env::var("appdata")? + "/.minecraft/TlauncherProfiles.json";
    let file = File::open(path)?;
    let value: serde_json::Value = serde_json::from_reader(file)?;

    let selected_acc = value.get("selectedAccountUUID").unwrap().as_str().unwrap();
    let accounts = value.get("accounts").unwrap().as_object().unwrap();
    let acc = accounts.get(selected_acc).unwrap().as_object().unwrap();
    let token = acc.get("accessToken").unwrap().as_str().unwrap();

    Ok(AuthData {
        selected_profile: selected_acc.to_string(),
        access_token: token.to_string(),
    })
}

#[derive(Deserialize)]
struct PolyProfile<'x> {
    id: &'x str,
}
#[derive(Deserialize)]
struct PolyYgg<'x> {
    token: &'x str,
}
#[derive(Deserialize)]
struct PolyAccount<'x> {
    #[serde(borrow)]
    profile: PolyProfile<'x>,
    #[serde(borrow)]
    ygg: PolyYgg<'x>,
}
#[derive(Deserialize)]
struct PolyJson<'x> {
    #[serde(borrow)]
    accounts: Vec<PolyAccount<'x>>,
}

fn get_access_token_polymc() -> Result<AuthData> {
    let path = env::var("appdata")? + "/PolyMC/accounts.json";
    let content = std::fs::read_to_string(path)?;
    let value: PolyJson = serde_json::from_str(&content)?;
    let acc = &value.accounts[0];

    Ok(AuthData {
        selected_profile: acc.profile.id.to_string(),
        access_token: acc.ygg.token.to_string(),
    })
}

fn get_access_token() -> Result<AuthData> {
    if let Ok(x) = get_access_token_polymc() {
        return Ok(x);
    }
    get_access_token_tlauncher()
}

fn main() -> Result<()> {
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
