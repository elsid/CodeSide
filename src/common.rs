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

impl Clamp1 for f64 {}
