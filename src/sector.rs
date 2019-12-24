use crate::my_strategy::{
    Rect,
    Vec2,
    normalize_angle,
};

#[derive(Default, Clone, Copy, Debug, PartialOrd)]
pub struct Sector {
    begin: f64,
    length: f64,
}

impl Sector {
    pub fn new(begin: f64, length: f64) -> Self {
        Self { begin, length }
    }

    pub fn from_direction_and_spread(direction: Vec2, spread: f64) -> Self {
        Self { begin: normalize_angle(direction.angle() - spread), length: 2.0 * spread }
    }

    pub fn from_source_and_rect(source: Vec2, rect: &Rect) -> Self {
        let angles = [
            (rect.bottom_left() - source).angle(),
            (rect.top_left() - source).angle(),
            (rect.top_right() - source).angle(),
            (rect.bottom_right() - source).angle(),
        ];
        let mut min_angle = std::f64::consts::PI;
        let mut max_length = 0.0;
        for i in 0 .. 3 {
            for j in 1 .. 4 {
                let mut diff = angles[i] - angles[j];
                if diff >= std::f64::consts::PI {
                    diff = 2.0 * std::f64::consts::PI - diff;
                    if max_length < diff {
                        max_length = diff;
                        min_angle = angles[i];
                    }
                } else if diff <= -std::f64::consts::PI {
                    diff += 2.0 * std::f64::consts::PI;
                    if max_length < diff {
                        max_length = diff;
                        min_angle = angles[j];
                    }
                } else {
                    diff = diff.abs();
                    if max_length < diff {
                        max_length = diff;
                        min_angle = angles[i].min(angles[j]);
                    }
                }
            }
        }
        Self { begin: min_angle, length: max_length }
    }

    pub fn get_intersection_fraction(&self, other: Self) -> f64 {
        let begin_diff = self.begin - other.begin;
        let other_begin = if begin_diff > std::f64::consts::PI {
            other.begin + 2.0 * std::f64::consts::PI
        } else if begin_diff < -std::f64::consts::PI {
            other.begin - 2.0 * std::f64::consts::PI
        } else {
            other.begin
        };
        let (min_begin, min_length, max_begin, max_length, swap) = if self.begin <= other_begin {
            (self.begin, self.length, other_begin, other.length, false)
        } else {
            (other_begin, other.length, self.begin, self.length, true)
        };
        let min_end = min_begin + min_length;
        let to_max_begin = max_begin - min_end;
        if to_max_begin >= 0.0 {
            return 0.0;
        }
        let to_max_end = max_begin + max_length - min_end;
        if to_max_end > 0.0 {
            -to_max_begin / if swap {
                max_length
            } else {
                min_length
            }
        } else {
            if swap {
                1.0
            } else {
                max_length / min_length
            }
        }
    }
}

impl PartialEq for Sector {
    #[inline(always)]
    fn eq(&self, rhs: &Self) -> bool {
        (self.begin, self.length).eq(&(rhs.begin, rhs.length))
    }
}

impl Eq for Sector {}
