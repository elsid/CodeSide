use crate::my_strategy::Rect;

pub trait Rectangular {
    fn rect(&self) -> Rect;
}
