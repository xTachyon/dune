use anyhow::Result;
use async_trait::async_trait;
use proxy_lib::do_things;
use proxy_lib::events::{ChatEvent, EventSubscriber};

struct EventHandler {}

#[async_trait]
impl EventSubscriber for EventHandler {
    async fn on_chat(event: ChatEvent) {}
}

#[tokio::main]
async fn main() -> Result<()> {
    let handler = Box::new(EventHandler {});
    do_things().await?;

    Ok(())
}
