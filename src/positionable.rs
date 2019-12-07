use crate::my_strategy::Vec2;

pub trait Positionable {
    fn position(&self) -> Vec2;
}
