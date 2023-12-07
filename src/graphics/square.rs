use crate::geometry::point::Point;
use crate::graphics::colors::Colors;
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
    pub fn new(
        position: Point,
        a: Point,
        b: Point,
        c: Point,
        d: Point,
        color: Option<Colors>,
        texture_path: Option<&str>,
    ) -> Self {
        let object = Object::new(
            position,
            &[
                a.x + position.x,
                a.y + position.y,
                a.z + position.z,
                1.0,
                1.0,
                b.x + position.x,
                b.y + position.y,
                b.z + position.z,
                1.0,
                0.0,
                c.x + position.x,
                c.y + position.y,
                c.z + position.z,
                0.0,
                0.0,
                d.x + position.x,
                d.y + position.y,
                d.z + position.z,
                0.0,
                1.0,
            ],
            &[0, 1, 3, 1, 2, 3],
            color,
            texture_path,
        )
        .unwrap();

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
