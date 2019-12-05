use model::Vec2F64;

#[derive(Debug)]
pub struct Vec2F64Wrapper(pub Vec2F64);

impl PartialEq for Vec2F64Wrapper {
    fn eq(&self, other: &Self) -> bool {
        let Self(lhs) = self;
        let Self(rhs) = other;
        (lhs.x, lhs.y).eq(&(rhs.x, rhs.y))
    }
}

impl Eq for Vec2F64Wrapper {}
