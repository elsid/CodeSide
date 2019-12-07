use model::Unit;
use crate::my_strategy::{
    Positionable,
    Rect,
    Rectangular,
    Vec2,
};

impl Positionable for Unit {
    fn position(&self) -> Vec2 {
        Vec2::from_model(&self.position)
    }
}

impl Rectangular for Unit {
    fn rect(&self) -> Rect {
        let half = Vec2::from_model(&self.size) * 0.5;
        Rect::new(Vec2::new(self.position.x, self.position.y + half.y()), half)
    }
}
