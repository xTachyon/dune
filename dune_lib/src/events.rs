use anyhow::Result;

use crate::protocol::v1_18_2::play::TradeListResponse;
pub use crate::protocol::v1_18_2::play::UseEntityKind;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PositionInt {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
pub struct Trades<'b> {
    pub packet: TradeListResponse,
    pub buffer: &'b [u8],
}
pub struct UseEntity {
    pub entity_id: i32,
    pub kind: UseEntityKind,
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
    fn trades(&mut self, _trades: Trades) -> Result<()> {
        Ok(())
    }
    fn interact(&mut self, _use_entity: UseEntity) -> Result<()> {
        Ok(())
    }
}
