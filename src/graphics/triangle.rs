use crate::geometry::point::Point;
use crate::graphics::colors::Colors;
use crate::graphics::object::Object;
use crate::graphics::vertex::Vertex;

pub struct Triangle {
    position: Point,
    a: Point,
    b: Point,
    c: Point,
    object: Object,
}

impl Triangle {
    pub fn new(position: Point, a: Point, b: Point, c: Point, color: Colors) -> Self {
        let mut vertex = Vertex::empty();

        vertex.push(a.into());
        vertex.push(b.into());
        vertex.push(c.into());

        let object = Object::new(position, vertex, &[0, 1, 2], Some(color), None).unwrap();

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
        self.object.set_position(position);
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
