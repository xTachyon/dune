use anyhow::Result;
use async_trait::async_trait;
use proxy_lib::do_things;
use proxy_lib::events::{ChatEvent, EventSubscriber};

async fn println_async(string: String) -> Result<()> {
    tokio::task::spawn_blocking(move || println!("{}", string)).await?;
    Ok(())
}

struct EventHandler {}

#[async_trait]
impl EventSubscriber for EventHandler {
    async fn on_chat(&self, event: ChatEvent) -> Result<()> {
        println_async(format!("{:?}", event)).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let handler = Box::new(EventHandler {});
    do_things(handler).await?;

    Ok(())
}
