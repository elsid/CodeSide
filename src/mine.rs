use model::{
    Mine,
};
use crate::my_strategy::{
    Positionable,
    Rect,
    Rectangular,
    Vec2,
};

impl Positionable for Mine {
    fn position(&self) -> Vec2 {
        Vec2::from_model(&self.position)
    }
}

impl Rectangular for Mine {
    fn rect(&self) -> Rect {
        Rect::new(self.center(), Vec2::from_model(&self.size) / 2.0)
    }

    fn center(&self) -> Vec2 {
        Vec2::from_model(&self.position) + Vec2::only_y(self.size.y / 2.0)
    }
}
