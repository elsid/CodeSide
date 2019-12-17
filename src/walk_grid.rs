use crate::my_strategy::Vec2;

pub struct WalkGrid {
    ax: f64,
    ay: f64,
    nx: f64,
    ny: f64,
    sign_x: f64,
    sign_y: f64,
    avx: f64,
    avy: f64,
    to_border_x: f64,
    to_border_y: f64,
    point: Vec2,
}

impl WalkGrid {
    #[inline(always)]
    pub fn new(begin: Vec2, end: Vec2) -> Self {
        let to = end - begin;
        let av = to.normalized();
        let sign_x = av.x().signum();
        let sign_y = av.y().signum();
        let to_border_x = if sign_x >= 0.0 {
            (begin.x().ceil() - begin.x())
        } else {
            (begin.x() - begin.x().floor())
        };
        let to_border_y = if sign_y >= 0.0 {
            (begin.y().ceil() - begin.y())
        } else {
            (begin.y() - begin.y().floor())
        };
        Self {
            ax: 0.0,
            ay: 0.0,
            nx: to.x().abs(),
            ny: to.y().abs(),
            sign_x,
            sign_y,
            avx: av.x().abs(),
            avy: av.y().abs(),
            to_border_x,
            to_border_y,
            point: begin,
        }
    }
}

impl Iterator for WalkGrid {
    type Item = Vec2;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.avx != 0.0 && self.avy != 0.0 && self.ax <= self.nx && self.ay <= self.ny {
            let point = self.point;
            let dtx = self.to_border_x / self.avx;
            let dty = self.to_border_y / self.avy;

            if dtx < dty {
                self.point.add_x(self.sign_x);
                let dy = self.avy * dtx;
                self.ax += self.to_border_x;
                self.ay += dy;
                self.to_border_x = 1.0;
                self.to_border_y = (self.to_border_y - dy).max(0.0);
            } else {
                self.point.add_y(self.sign_y);
                let dx = self.avx * dty;
                self.ax += dx;
                self.ay += self.to_border_y;
                self.to_border_x = (self.to_border_x - dx).max(0.0);
                self.to_border_y = 1.0;
            }

            Some(point)
        } else if self.avx != 0.0 && self.avy == 0.0 && self.ax <= self.nx {
            let point = self.point;

            self.point.add_x(self.sign_x);
            self.ax += self.to_border_x;
            self.to_border_x = 1.0;

            Some(point)
        } else if self.avx == 0.0 && self.avy != 0.0 && self.ay <= self.ny {
            let point = self.point;

            self.point.add_y(self.sign_y);
            self.ay += self.to_border_y;
            self.to_border_y = 1.0;

            Some(point)
        } else {
            None
        }
    }
}
