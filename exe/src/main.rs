use anyhow::Result;
use proxy_lib::events::{ChatEvent, EventSubscriber};
use proxy_lib::{play, record_to_file, AuthData};
use std::env;
use std::fs::File;

struct EventHandler {}

impl EventSubscriber for EventHandler {
    fn on_chat(&self, event: ChatEvent) -> Result<()> {
        println!("chat: {:?}", event);
        Ok(())
    }
}

fn get_access_token() -> Result<AuthData> {
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

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("no args supplied");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "record" => {
            let auth_data = get_access_token()?;
            let server_address = "127.0.0.1:25565";

            record_to_file(server_address, auth_data, "packets.dune")?;
        }
        "play" => {
            let handler = Box::new(EventHandler {});
            play("packets.dune", handler)?;
        }
        _ => {}
    }

    Ok(())
}
