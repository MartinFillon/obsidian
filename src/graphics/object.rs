use crate::geometry::point::Point;
use crate::graphics::vertex::{Ebo, Vao, Vbo, VertexAttribute};
use gl::types::{GLfloat, GLsizei};
use std::{mem, ptr};

#[derive(Debug, Clone)]
pub struct Object {
    pub position: Point,
    vao: Vao,
    vbo: Vbo,
    ebo: Ebo,
    position_attribute: VertexAttribute,
    index_attribute: VertexAttribute,
}

impl Object {
    pub fn new(position: Point, vertex: &[f32], indices: &[u32]) -> Self {
        let vao = Vao::new();
        vao.bind();

        let vbo = Vbo::new();
        vbo.bind();

        let ebo = Ebo::new();
        ebo.bind();

        let position_attribute = VertexAttribute::new(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );

        let index_attribute = VertexAttribute::new(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );

        vao.bind();
        vbo.bind();
        vbo.buffer_data(vertex);

        ebo.bind();
        ebo.buffer_data(indices);

        position_attribute.enable();
        index_attribute.enable();
        Self {
            position,
            vao,
            vbo,
            ebo,
            position_attribute,
            index_attribute,
        }
    }

    pub fn display(&mut self) {
        log::info!("Object: {:?}", self);
        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        self.vao.unbind();
        self.vbo.unbind();
        self.ebo.unbind();
        self.position_attribute.disable();
        self.index_attribute.disable();
    }
}
