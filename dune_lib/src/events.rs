use anyhow::Result;
pub use dune_data::protocol::v1_20_2::play::{TradeListResponse, UseEntityKind};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    fn trades(&mut self, _trades: TradeListResponse) -> Result<()> {
        Ok(())
    }
    fn interact(&mut self, _use_entity: UseEntity) -> Result<()> {
        Ok(())
    }
}
