use std::ops::{Add, Sub, Mul, Div, Neg};
use model::{
    Vec2F32,
    Vec2F64,
};
use crate::my_strategy::Square;

#[derive(Default, Clone, Copy, Debug, PartialOrd)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    #[inline(always)]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub const fn i() -> Self {
        Self::only_x(1.0)
    }

    pub const fn only_x(x: f64) -> Self {
        Self { x, y: 0.0 }
    }

    #[inline(always)]
    pub const fn from_model(value: &Vec2F64) -> Self {
        Self { x: value.x, y: value.y }
    }

    #[inline(always)]
    pub const fn as_model(&self) -> Vec2F64 {
        Vec2F64 { x: self.x, y: self.y }
    }

    #[cfg(feature = "enable_debug")]
    #[inline(always)]
    pub const fn as_model_f32(&self) -> Vec2F32 {
        Vec2F32 { x: self.x as f32, y: self.y as f32 }
    }

    #[inline(always)]
    pub const fn x(&self) -> f64 {
        self.x
    }

    #[inline(always)]
    pub const fn y(&self) -> f64 {
        self.y
    }

    #[inline(always)]
    pub fn norm(&self) -> f64 {
        (self.x.square() + self.y.square()).sqrt()
    }

    #[inline(always)]
    pub fn distance(&self, other: Self) -> f64 {
        (other - *self).norm()
    }

    #[inline(always)]
    pub fn set_x(&mut self, value: f64) {
        self.x = value;
    }

    #[inline(always)]
    pub fn set_y(&mut self, value: f64) {
        self.y = value;
    }

    #[inline(always)]
    pub fn add_x(&mut self, value: f64) {
        self.x += value;
    }

    #[inline(always)]
    pub fn add_y(&mut self, value: f64) {
        self.y += value;
    }

    #[inline(always)]
    pub fn normalized(&self) -> Self {
        *self / self.norm()
    }

    #[inline(always)]
    pub fn rotated(&self, angle: f64) -> Self {
        let sin = angle.sin();
        let cos = angle.cos();
        Self::new(self.x * cos - self.y * sin, self.y * cos + self.x * sin)
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn mul(self, rhs: f64) -> Vec2 {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f64> for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn div(self, rhs: f64) -> Vec2 {
        Vec2::new(self.x / rhs, self.y / rhs)
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

impl PartialEq for Vec2 {
    #[inline(always)]
    fn eq(&self, rhs: &Vec2) -> bool {
        (self.x, self.y).eq(&(rhs.x, rhs.y))
    }
}

impl Eq for Vec2 {}
