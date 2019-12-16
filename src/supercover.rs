use crate::my_strategy::Location;

pub struct Supercover {
    location: Location,
    ix: f64,
    iy: f64,
    sign_x: isize,
    sign_y: isize,
    ny: f64,
    nx: f64,
}

impl Supercover {
    pub fn new(start: Location, end: Location) -> Self {
        let (dx, dy) = (end.x() as isize - start.x() as isize, end.y() as isize - start.y() as isize);

        Self {
            location: start,
            ix: 0.0,
            iy: 0.0,
            sign_x: dx.signum(),
            sign_y: dy.signum(),
            nx: dx.abs() as f64,
            ny: dy.abs() as f64,
        }
    }
}

impl Iterator for Supercover {
    type Item = Location;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ix <= self.nx && self.iy <= self.ny {
            let location = self.location;
            let comparison = ((0.5 + self.ix) / self.nx) - ((0.5 + self.iy) / self.ny);

            if comparison == 0.0 {
                self.location.add_x(self.sign_x);
                self.location.add_y(self.sign_y);
                self.ix += 1.0;
                self.iy += 1.0;
            } else if comparison < 0.0 {
                self.location.add_x(self.sign_x);
                self.ix += 1.0;
            } else {
                self.location.add_y(self.sign_y);
                self.iy += 1.0;
            }

            Some(location)
        } else {
            None
        }
    }
}