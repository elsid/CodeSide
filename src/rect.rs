use crate::my_strategy::Vec2;

#[cfg(feature = "enable_debug")]
use model::{
    ColorF32,
    CustomData,
};

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

    #[cfg(feature = "enable_debug")]
    pub fn as_debug(&self, color: ColorF32) -> CustomData {
        CustomData::Rect {
            pos: (self.center - self.half).as_debug(),
            size: (self.half * 2.0).as_debug(),
            color,
        }
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

    pub fn top_left(&self) -> Vec2 {
        self.center + Vec2::new(-self.half.x(), self.half.y())
    }

    pub fn top_right(&self) -> Vec2 {
        self.center + Vec2::new(self.half.x(), self.half.y())
    }

    pub fn bottom_left(&self) -> Vec2 {
        self.center + Vec2::new(-self.half.x(), -self.half.y())
    }

    pub fn bottom_right(&self) -> Vec2 {
        self.center + Vec2::new(self.half.x(), -self.half.y())
    }

    pub fn has_intersection_with_line(&self, a: Vec2, b: Vec2) -> bool {
        let d = b - a;
        let min = self.min();
        let max = self.max();
        let p = [-d.x(), d.x(), -d.y(), d.y()];
        let q = [a.x() - min.x(), max.x() - a.x(), a.y() - min.y(), max.y() - a.y()];
        let mut u1 = 0.0;
        let mut u2 = 1.0;
        for i in 0 .. 4 {
            if p[i] == 0.0 {
                if q[i] >= 0.0 {
                    continue;
                }
                return false;
            }
            let candidate = q[i] / p[i];
            if p[i] < 0.0 {
                if u1 < candidate {
                    u1 = candidate;
                }
            } else {
                if u2 > candidate {
                    u2 = candidate;
                }
            }
        }
        u1 <= u2
    }

    pub fn get_intersection_with_line(&self, a: Vec2, b: Vec2) -> Option<f64> {
        if self.has_intersection_with_line(a, b) {
            Some(self.center.distance(a))
        } else {
            None
        }
    }
}
