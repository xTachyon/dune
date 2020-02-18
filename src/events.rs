use async_trait::async_trait;

struct ChatEvent {

}

#[async_trait]
trait EventHandler {
    async fn on_chat(event: ChatEvent) {}
}