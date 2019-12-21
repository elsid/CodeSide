use crate::my_strategy::{Rect, Vec2};

pub trait Rectangular {
    fn rect(&self) -> Rect;
    fn center(&self) -> Vec2;
}
