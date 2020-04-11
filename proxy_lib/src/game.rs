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
    x: f64,
    y: f64,
    z: f64
}