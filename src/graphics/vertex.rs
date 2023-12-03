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

    pub fn buffer_data(&self, data: &VertexData) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                data.get_size(),
                data.get_vertices().as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }
}

pub type Vertex = [f32; 3];

pub struct VertexData {
    vertices: [Vertex; 3],
    size: isize,
}

impl VertexData {
    pub fn new(vertices: [Vertex; 3]) -> Self {
        Self {
            vertices,
            size: std::mem::size_of_val(&vertices) as isize,
        }
    }

    pub fn get_vertices(&self) -> &[Vertex; 3] {
        &self.vertices
    }

    pub fn get_size(&self) -> isize {
        self.size
    }
}
