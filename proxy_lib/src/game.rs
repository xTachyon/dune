use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Debug, TryFromPrimitive)]
pub enum Gamemode {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
}

#[derive(Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
