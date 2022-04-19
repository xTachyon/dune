use anyhow::Result;
use proxy_lib::do_things;
use proxy_lib::events::{ChatEvent, EventSubscriber};

struct EventHandler {}

impl EventSubscriber for EventHandler {
    fn on_chat(&self, event: ChatEvent) -> Result<()> {
        println!("chat: {:?}", event);
        Ok(())
    }
}

fn main() -> Result<()> {
    let server_address = "127.0.0.1:25565";

    let handler = Box::new(EventHandler {});
    do_things(server_address, handler)?;

    Ok(())
}
