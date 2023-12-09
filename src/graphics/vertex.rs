use crate::geometry::point::Point;
use gl::types::{GLboolean, GLenum, GLfloat, GLsizei, GLsizeiptr};
use std::ffi::c_void;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertices {
    position: [f32; 3],
    texture: [f32; 2],
}

impl Vertices {
    pub fn new(position: [f32; 3], texture: [f32; 2]) -> Self {
        Self { position, texture }
    }

    pub fn empty() -> Self {
        Self {
            position: [0.0; 3],
            texture: [0.0; 2],
        }
    }

    pub fn set_position(&mut self, position: &[f32; 3]) {
        self.position = position.clone();
    }

    pub fn set_texture(&mut self, texture: &[f32; 2]) {
        self.texture = texture.clone();
    }
}

impl From<Point> for Vertices {
    fn from(point: Point) -> Self {
        Self {
            position: [point.x, point.y, point.z],
            texture: [0.0, 0.0],
        }
    }
}

impl From<Vertices> for [f32; 5] {
    fn from(vertices: Vertices) -> Self {
        [
            vertices.position[0],
            vertices.position[1],
            vertices.position[2],
            vertices.texture[0],
            vertices.texture[1],
        ]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vertex {
    vertex: Vec<Vertices>,
}

impl Vertex {
    pub fn new(vertex: Vec<Vertices>) -> Self {
        Self { vertex }
    }

    pub fn empty() -> Self {
        Self { vertex: vec![] }
    }

    pub fn push(&mut self, vertex: Vertices) {
        self.vertex.push(vertex);
    }

    pub fn get_mut(&mut self, index: usize) -> &mut Vertices {
        self.vertex.get_mut(index).unwrap()
    }

    pub fn set(&mut self, index: usize, vertex: Vertices) {
        self.vertex[index] = vertex.clone();
    }

    pub fn change_position(&mut self, position: Point) {
        self.vertex = self
            .vertex
            .iter()
            .map(|v| {
                let mut coords = v.clone();
                coords.position[0] += position.x;
                coords.position[1] += position.y;
                coords.position[2] += position.z;
                coords
            })
            .collect();
        ()
    }
}

impl From<Vertex> for Box<[f32]> {
    fn from(vertex: Vertex) -> Self {
        let mut vertices: Vec<[f32; 5]> = vec![];
        for v in vertex.vertex {
            vertices.push(v.into());
        }
        vertices.concat().as_slice().into()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vao {
    pub id: u32,
}

impl Vao {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Self { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vbo {
    pub id: u32,
}

impl Vbo {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Self { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn buffer_data(&self, data: Box<[f32]>) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                &data[0] as *const f32 as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Ebo {
    pub id: u32,
}

impl Ebo {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Self { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    pub fn buffer_data(&self, data: &[u32]) {
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                &data[0] as *const u32 as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }
}

#[derive(Debug, Clone)]
pub struct VertexAttribute {
    index: u32,
}

impl VertexAttribute {
    pub fn new(
        index: u32,
        size: u32,
        r#type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const c_void,
    ) -> Self {
        unsafe {
            gl::VertexAttribPointer(index, size as i32, r#type, normalized, stride, pointer);
        }
        Self { index }
    }

    pub fn enable(&self) {
        unsafe {
            gl::EnableVertexAttribArray(self.index);
        }
    }

    pub fn disable(&self) {
        unsafe {
            gl::DisableVertexAttribArray(self.index);
        }
    }
}
