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
