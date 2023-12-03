use gl::types::{GLboolean, GLenum, GLfloat, GLsizei, GLsizeiptr};
use std::ffi::c_void;

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

    pub fn buffer_data(&self, data: &[f32]) {
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
