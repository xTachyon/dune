use anyhow::Result;
use async_trait::async_trait;
use proxy_lib::do_things;
use proxy_lib::events::{ChatEvent, EventSubscriber};
use tokio::io::AsyncWriteExt;

async fn print_string_async(string: String) -> Result<()> {
    let mut stdout = tokio::io::stdout();
    stdout.write_all(string.as_bytes()).await?;
    Ok(())
}

macro_rules! async_println {
    () => (async_println!(""));
    ($($arg:tt)*) => ({
        let mut text = format!($($arg)*);
        text.push('\n');
        print_string_async(text)
    })
}

struct EventHandler {}

#[async_trait]
impl EventSubscriber for EventHandler {
    async fn on_chat(&self, event: ChatEvent) -> Result<()> {
        async_println!("chat: {:?}", event).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let server_address = "192.168.0.206:25566";

    let handler = Box::new(EventHandler {});
    do_things(server_address, handler).await?;

    Ok(())
}
