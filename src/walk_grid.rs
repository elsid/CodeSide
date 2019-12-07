use crate::my_strategy::Vec2;

pub struct Steps<T, I> {
    iterator: I,
    prev: Option<T>,
}

impl<T: Copy, I: Iterator<Item = T>> Steps<T, I> {
    #[inline(always)]
    pub fn new(mut iterator: I) -> Self {
        Self {
            prev: iterator.next(),
            iterator,
        }
    }
}

impl<T: Copy, I: Iterator<Item = T>> Iterator for Steps<T, I> {
    type Item = (T, T);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().and_then(|next| {
            self.prev.map(|prev| {
                self.prev = Some(next);
                (prev, next)
            })
        })
    }
}

pub struct WalkGrid {
    point: Vec2,
    ix: f64,
    iy: f64,
    sign_x: f64,
    sign_y: f64,
    ny: f64,
    nx: f64,
}

impl WalkGrid {
    #[inline(always)]
    pub fn new(start: Vec2, end: Vec2) -> Self {
        // Delta values between the points
        let (dx, dy) = (end.x() - start.x(), end.y() - start.y());

        Self {
            point: start,
            ix: 0.0,
            iy: 0.0,
            sign_x: dx.signum(),
            sign_y: dy.signum(),
            nx: dx.abs(),
            ny: dy.abs(),
        }
    }

    #[inline(always)]
    pub fn steps(self) -> Steps<Vec2, WalkGrid> {
        Steps::new(self)
    }
}

impl Iterator for WalkGrid {
    type Item = Vec2;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.ix <= self.nx && self.iy <= self.ny {
            let point = self.point;

            if (0.5 + self.ix) / self.nx < (0.5 + self.iy) / self.ny {
                self.point.add_x(self.sign_x);
                self.ix += 1.0;
            } else {
                self.point.add_y(self.sign_y);
                self.iy += 1.0;
            }

            Some(point)
        } else {
            None
        }
    }
}
