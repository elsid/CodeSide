use std::ops::Mul;

macro_rules! log {
    ($tick_index:expr, $message:tt) => {
        if cfg!(feature = "enable_log") {
            let f = || {
                use std::io::{stdout, Write};
                write!(&mut stdout(), "[{}] {}\n", $tick_index, $message).unwrap();
            };
            f();
        }
    };
    ($tick_index:expr, $format:tt, $($value:expr),*) => {
        if cfg!(feature = "enable_log") {
            let f = || {
                use std::io::{stdout, Write};
                write!(&mut stdout(), "[{}] {}\n", $tick_index, format!($format, $($value),*)).unwrap();
            };
            f();
        }
    };
}

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
    (value * 100000.0).round() as i32
}

pub fn normalize_angle(value: f64) -> f64 {
    use std::f64::consts::{PI, FRAC_1_PI};
    if value > PI {
        value - (value * 0.5 * FRAC_1_PI).round() * 2.0 * PI
    } else if value < -PI {
        value + (value.abs() * 0.5 * FRAC_1_PI).round() * 2.0 * PI
    } else {
        value
    }
}

pub fn remove_if<T: Clone, F>(vec: &mut Vec<T>, predicate: F)
        where F: Fn(&T) -> bool {
    let end = vec.len();
    let mut i = 0;
    let mut shrink = 0;
    while i + shrink < end {
        if predicate(&vec[i]) {
            if i + 1 + shrink < end {
                let (left, right) = vec.split_at_mut(i + 1);
                std::mem::swap(&mut left[i], &mut right[right.len() - 1 - shrink]);
            }
            shrink += 1;
        } else {
            i += 1;
        }
    }
    vec.truncate(end - shrink);
}
