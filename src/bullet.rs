use model::Bullet;
use crate::my_strategy::{
    Positionable,
    Rect,
    Rectangular,
    Vec2,
};

impl Positionable for Bullet {
    fn position(&self) -> Vec2 {
        Vec2::from_model(&self.position)
    }
}

impl Rectangular for Bullet {
    fn rect(&self) -> Rect {
        let half = self.size * 0.5;
        Rect::new(Vec2::from_model(&self.position), Vec2::new(half, half))
    }
}
