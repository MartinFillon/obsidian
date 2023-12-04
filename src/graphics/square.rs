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

    pub fn set_position(&mut self, position: Point) {
        self.position = position;
        self.a.x += position.x;
        self.a.y += position.y;
        self.a.z += position.z;
        self.b.x += position.x;
        self.b.y += position.y;
        self.b.z += position.z;
        self.c.x += position.x;
        self.c.y += position.y;
        self.c.z += position.z;
        self.d.x += position.x;
        self.d.y += position.y;
        self.d.z += position.z;

        self.object = Object::new(
            position,
            &[
                self.a.x, self.a.y, self.a.z, self.b.x, self.b.y, self.b.z, self.c.x, self.c.y,
                self.c.z, self.d.x, self.d.y, self.d.z,
            ],
            &[0, 1, 2, 0, 2, 3],
        );
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
