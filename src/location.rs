use std::ops::Add;

#[cfg(feature = "enable_debug")]
use model::Vec2F32;

use crate::my_strategy::Vec2i;

#[derive(Default, Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Location {
    x: usize,
    y: usize,
}

impl Location {
    #[inline(always)]
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub const fn x(&self) -> usize {
        self.x
    }

    #[inline(always)]
    pub const fn y(&self) -> usize {
        self.y
    }

    #[cfg(feature = "enable_debug")]
    #[inline(always)]
    pub const fn as_model_f32(&self) -> Vec2F32 {
        Vec2F32 { x: self.x as f32, y: self.y as f32 }
    }
}

impl Add<Vec2i> for Location {
    type Output = Location;

    #[inline(always)]
    fn add(self, rhs: Vec2i) -> Location {
        Location::new(sum_usize_and_isize(self.x, rhs.x()), sum_usize_and_isize(self.y, rhs.y()))
    }
}

#[inline(always)]
fn sum_usize_and_isize(lhs: usize, rhs: isize) -> usize {
    use std::num::Wrapping;

    if rhs.is_negative() {
        (Wrapping(lhs) - Wrapping(rhs.wrapping_abs() as usize)).0
    } else {
        (Wrapping(lhs) + Wrapping(rhs as usize)).0
    }
}