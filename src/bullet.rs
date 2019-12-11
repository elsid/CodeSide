use model::Bullet;
use crate::my_strategy::{
    Positionable,
    Vec2,
};

impl Positionable for Bullet {
    fn position(&self) -> Vec2 {
        Vec2::from_model(&self.position)
    }
}
