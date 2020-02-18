use enum_primitive_derive::*;

#[derive(Debug, Primitive)]
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