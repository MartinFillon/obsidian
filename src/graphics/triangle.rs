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
            ],
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

    pub fn set_position(&mut self, position: Point) {
        self.position = position;
    }

    pub fn get_position(&self) -> Point {
        self.position
    }

    pub fn get_a(&self) -> Point {
        self.a
    }

    pub fn get_b(&self) -> Point {
        self.b
    }

    pub fn get_c(&self) -> Point {
        self.c
    }
}
