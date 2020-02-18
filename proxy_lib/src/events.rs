use async_trait::async_trait;

pub struct ChatEvent {

}

#[async_trait]
pub trait EventSubscriber {
    async fn on_chat(event: ChatEvent) {}
}