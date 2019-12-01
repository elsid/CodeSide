use crate::my_strategy::vec2::Vec2;

#[derive(Default, Clone, Debug)]
pub struct Rect {
    center: Vec2,
    half: Vec2,
}

impl Rect {
    pub const fn new(center: Vec2, half: Vec2) -> Self {
        Rect { center, half }
    }

    pub const fn center(&self) -> Vec2 {
        self.center
    }

    pub const fn half(&self) -> Vec2 {
        self.half
    }

    pub fn collide(&self, other: &Rect) -> Vec2 {
        Vec2::new(
            (self.center.x() - other.center.x()).abs() - (self.half.x() + other.half.x()),
            (self.center.y() - other.center.y()).abs() - (self.half.y() + other.half.y())
        )
    }
}
