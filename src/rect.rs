use crate::my_strategy::Vec2;

#[derive(Default, Clone, Debug)]
pub struct Rect {
    center: Vec2,
    half: Vec2,
}

impl Rect {
    #[inline(always)]
    pub const fn new(center: Vec2, half: Vec2) -> Self {
        Self { center, half }
    }

    #[inline(always)]
    pub const fn center(&self) -> Vec2 {
        self.center
    }

    #[inline(always)]
    pub const fn half(&self) -> Vec2 {
        self.half
    }

    #[inline(always)]
    pub fn min(&self) -> Vec2 {
        self.center - self.half
    }

    #[inline(always)]
    pub fn max(&self) -> Vec2 {
        self.center + self.half
    }

    #[inline(always)]
    pub fn has_collision(&self, other: &Self) -> bool {
        self.min().x() < other.max().x()
            && self.max().x() > other.min().x()
            && self.min().y() < other.max().y()
            && self.max().y() > other.min().y()
    }
}
