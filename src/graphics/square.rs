use crate::geometry::point::Point;
use crate::graphics::object::Object;

pub struct Square {
    position: Point,
    a: Point,
    b: Point,
    c: Point,
    d: Point,
    object: Object,
}

impl Square {
    pub fn new(position: Point, a: Point, b: Point, c: Point, d: Point) -> Self {
        let object = Object::new(
            position,
            &[
                a.x + position.x,
                a.y + position.y,
                a.z + position.z,
                b.x + position.x,
                b.y + position.y,
                b.z + position.z,
                c.x + position.x,
                c.y + position.y,
                c.z + position.z,
                d.x + position.x,
                d.y + position.y,
                d.z + position.z,
            ],
            &[0, 1, 2, 0, 2, 3],
        );
        Self {
            position,
            a,
            b,
            c,
            d,
            object,
        }
    }

    pub fn display(&mut self) {
        self.object.display();
    }
}
