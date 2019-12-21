use model::{
    Mine,
    MineState,
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
        let size = if self.state == MineState::Idle {
            Vec2::new(self.trigger_radius, self.trigger_radius)
        } else {
            Vec2::from_model(&self.size) / 2.0
        };
        Rect::new(self.center(), size)
    }

    fn center(&self) -> Vec2 {
        Vec2::from_model(&self.position) + Vec2::only_y(self.size.y / 2.0)
    }
}
