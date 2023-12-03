use crate::geometry::point::Point;
use crate::graphics::object::Object;

pub struct Triangle {
    position: Point,
    a: Point,
    b: Point,
    c: Point,
    object: Object,
}

impl Triangle {
    pub fn new(position: Point, a: Point, b: Point, c: Point) -> Self {
        let object = Object::new(
            position,
            &[a.x, a.y, a.z, b.x, b.y, b.z, c.x, c.y, c.z],
            &[0, 1, 2],
        );
        Self {
            position,
            a,
            b,
            c,
            object,
        }
    }

    pub fn display(&mut self) {
        self.object.display();
    }
}
