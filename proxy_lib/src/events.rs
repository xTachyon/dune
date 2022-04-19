use anyhow::Result;

#[derive(Debug)]
pub struct ChatEvent {
    pub message: String,
}

pub trait EventSubscriber: Sync {
    fn on_chat(&self, _event: ChatEvent) -> Result<()> {
        Ok(())
    }
}
