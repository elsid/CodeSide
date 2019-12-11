use crate::my_strategy::{
    Vec2,
    Location,
};

pub trait Positionable {
    fn position(&self) -> Vec2;

    fn location(&self) -> Location {
        self.position().as_location()
    }
}
