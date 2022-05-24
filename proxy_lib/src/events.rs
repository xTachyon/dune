use anyhow::Result;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub trait EventSubscriber: Sync {
    fn on_chat(&mut self, _message: &str) -> Result<()> {
        Ok(())
    }
    fn player_info(&mut self, _name: &str, _uuid: u128) -> Result<()> {
        Ok(())
    }
    fn position(&mut self, _pos: Position) -> Result<()> {
        Ok(())
    }
}
