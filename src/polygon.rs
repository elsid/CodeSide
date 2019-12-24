use crate::my_strategy::{
    Rect,
    Vec2,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polygon {
    vertices: Vec<Vec2>,
}

impl Polygon {
    pub fn new() -> Self {
        Self { vertices: Vec::new() }
    }

    pub fn from_vertices(vertices: Vec<Vec2>) -> Self {
        Self { vertices }
    }

    pub fn from_rect(rect: &Rect) -> Self {
        let mut vertices = Vec::new();

        vertices.push(rect.bottom_left());
        vertices.push(rect.top_left());
        vertices.push(rect.top_right());
        vertices.push(rect.bottom_right());

        Self { vertices }
    }
}
