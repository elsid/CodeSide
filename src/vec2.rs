use std::ops::{Add, Sub, Mul, Div, Neg};
use model::Vec2F64;
use crate::my_strategy::Square;

#[derive(Default, Clone, Copy, Debug, PartialOrd)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub const fn from_model(value: &Vec2F64) -> Self {
        Self { x: value.x, y: value.y }
    }

    pub const fn as_model(&self) -> Vec2F64 {
        Vec2F64 { x: self.x, y: self.y }
    }

    pub const fn x(&self) -> f64 {
        self.x
    }

    pub const fn y(&self) -> f64 {
        self.y
    }

    pub fn norm(&self) -> f64 {
        (self.x.square() + self.y.square()).sqrt()
    }

    pub fn distance(&self, other: Self) -> f64 {
        (other - *self).norm()
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

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Vec2 {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Vec2 {
        Vec2::new(self.x / rhs, self.y / rhs)
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, rhs: &Vec2) -> bool {
        (self.x, self.y).eq(&(rhs.x, rhs.y))
    }
}

impl Eq for Vec2 {}
