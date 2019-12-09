use std::ops::Mul;

pub trait Square: Mul + Copy {
    fn square(self) -> Self::Output {
        self * self
    }
}

impl Square for f64 {}

pub trait Clamp1: PartialOrd + Sized {
    fn clamp1(self, min: Self, max: Self) -> Self {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

impl Clamp1 for f32 {}
impl Clamp1 for f64 {}
impl Clamp1 for usize {}

pub trait IsBetween: PartialOrd + Copy {
    fn is_between(self, left: Self, right: Self) -> bool {
        left < self && self < right
    }
}

impl IsBetween for f64 {}

pub struct IdGenerator {
    next: i32,
}

impl IdGenerator {
    pub fn new() -> Self {
        IdGenerator {next: 1}
    }

    pub fn next(&mut self) -> i32 {
        let result = self.next;
        self.next += 1;
        result
    }
}

pub fn as_score(value: f64) -> i32 {
    (value * 1000.0).round() as i32
}

pub fn normalize_angle(value: f64) -> f64 {
    use std::f64::consts::{PI, FRAC_1_PI};
    if value > PI {
        value - (value * 0.5 * FRAC_1_PI).round() * 2.0 * PI
    } else if value < -PI {
        value - (value.abs() * 0.5 * FRAC_1_PI).round() * 2.0 * PI
    } else {
        value
    }
}
