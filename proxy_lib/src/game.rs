use enum_primitive_derive::*;

#[derive(Debug, Primitive)]
pub enum Gamemode {
  Survival = 0,
  Creative = 1,
  Adventure = 2
}