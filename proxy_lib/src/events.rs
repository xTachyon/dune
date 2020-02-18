use async_trait::async_trait;
use anyhow::Result;

#[derive(Debug)]
pub struct ChatEvent {
    pub message: String
}

#[async_trait]
pub trait EventSubscriber : Sync {
    async fn on_chat(&self, _event: ChatEvent) -> Result<()> { Ok(()) }
}
