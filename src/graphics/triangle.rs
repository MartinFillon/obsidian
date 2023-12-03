use crate::geometry::point::Point;

pub struct Triangle {
    pub position: Point,
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn new(position: Point, a: Point, b: Point, c: Point) -> Self {
        Self { position, a, b, c }
    }
}
