#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec2i {
    x: isize,
    y: isize,
}

impl Vec2i {
    #[inline(always)]
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub const fn x(&self) -> isize {
        self.x
    }

    #[inline(always)]
    pub const fn y(&self) -> isize {
        self.y
    }

    #[inline(always)]
    pub const fn only_x(x: isize) -> Self {
        Self { x, y: 0 }
    }

    #[inline(always)]
    pub const fn only_y(y: isize) -> Self {
        Self { x: 0, y }
    }
}
