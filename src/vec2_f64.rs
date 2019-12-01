use model::Vec2F64;

#[derive(Debug)]
pub struct Vec2F64Wrapper(pub Vec2F64);

impl PartialEq for Vec2F64Wrapper {
    fn eq(&self, other: &Vec2F64Wrapper) -> bool {
        let Vec2F64Wrapper(lhs) = self;
        let Vec2F64Wrapper(rhs) = other;
        (lhs.x, lhs.y).eq(&(rhs.x, rhs.y))
    }
}

impl Eq for Vec2F64Wrapper {}
