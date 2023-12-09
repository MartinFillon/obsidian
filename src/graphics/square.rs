use crate::geometry::point::Point;
use crate::graphics::colors::Colors;
use crate::graphics::object::Object;
use crate::graphics::vertex::Vertex;
use log::debug;

#[allow(dead_code)]
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
        let mut vertex = Vertex::empty();

        vertex.push(a.into());
        vertex.push(b.into());
        vertex.push(c.into());
        vertex.push(d.into());

        if texture_path.is_some() {
            vertex.get_mut(0).set_texture(&[1.0, 1.0]);
            vertex.get_mut(1).set_texture(&[1.0, 0.0]);
            vertex.get_mut(2).set_texture(&[0.0, 0.0]);
            vertex.get_mut(3).set_texture(&[0.0, 1.0]);
        }

        debug!("Vertex: {:?}", vertex);

        let object =
            Object::new(position, vertex, &[0, 1, 3, 1, 2, 3], color, texture_path).unwrap();

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
        self.object.set_position(position);
    }

    pub fn get_position(&self) -> Point {
        self.position
    }
}
