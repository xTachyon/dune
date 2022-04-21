use anyhow::Result;
use proxy_lib::events::{ChatEvent, EventSubscriber};
use proxy_lib::{do_things, AuthData};
use std::fs::File;

struct EventHandler {}

impl EventSubscriber for EventHandler {
    fn on_chat(&self, event: ChatEvent) -> Result<()> {
        println!("chat: {:?}", event);
        Ok(())
    }
}

fn get_access_token() -> Result<AuthData> {
    let path = std::env::var("appdata")? + "/.minecraft/TlauncherProfiles.json";
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
    let auth_data = get_access_token()?;
    let server_address = "127.0.0.1:25565";

    let handler = Box::new(EventHandler {});
    do_things(server_address, auth_data, handler)?;

    Ok(())
}
