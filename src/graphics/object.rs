use crate::errors::Errors;
use crate::geometry::point::Point;
use crate::graphics::colors::Colors;
use crate::graphics::texture::Texture2D;
use crate::graphics::vertex::{Ebo, Vao, Vbo, Vertex, VertexAttribute};
use crate::shaders::shader::{Shader, ShaderProgram, ShaderType};
use gl::types::{GLfloat, GLsizei};
use std::ffi::c_void;
use std::{mem, ptr};

#[derive(Debug, Clone, Copy, PartialEq)]
enum DisplayType {
    Color,
    Texture,
}

#[derive(Debug, Clone)]
pub struct Object {
    pub position: Point,
    vao: Vao,
    vbo: Vbo,
    ebo: Ebo,
    vertex: Vertex,
    position_attribute: VertexAttribute,
    texture_attribute: Option<VertexAttribute>,
    shader_program: ShaderProgram,
    display_type: DisplayType,
    texture: Option<Texture2D>,
    color: Colors,
}

impl Object {
    pub fn new(
        position: Point,
        vertex: Vertex,
        indices: &[u32],
        color: Option<Colors>,
        texture_path: Option<&str>,
    ) -> Result<Self, Errors> {
        if color.is_some() && texture_path.is_some() {
            return Err(Errors::ColorAndTextureError);
        }
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
            5 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );

        let texture_attribute = if texture_path.is_some() {
            Some(VertexAttribute::new(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                5 * mem::size_of::<GLfloat>() as GLsizei,
                (3 * mem::size_of::<GLfloat>()) as *const c_void,
            ))
        } else {
            None
        };

        vao.bind();
        vbo.bind();
        vbo.buffer_data(vertex.clone().into());

        ebo.bind();
        ebo.buffer_data(indices);

        position_attribute.enable();
        if texture_path.is_some() {
            texture_attribute.as_ref().unwrap().enable();
        }

        let vertex_shader: Shader = if texture_path.is_some() {
            Shader::new(
                ShaderType::VertexShader,
                include_str!("../shaders/texture_shader.vert"),
            )?
        } else {
            Shader::new(
                ShaderType::VertexShader,
                include_str!("../shaders/color_shader.vert"),
            )?
        };

        let fragment_shader: Shader = if texture_path.is_some() {
            Shader::new(
                ShaderType::FragmentShader,
                include_str!("../shaders/texture_shader.frag"),
            )?
        } else {
            Shader::new(
                ShaderType::FragmentShader,
                include_str!("../shaders/color_shader.frag"),
            )?
        };

        vertex_shader.compile()?;
        fragment_shader.compile()?;

        let mut shader_program = ShaderProgram::new();

        shader_program.attach_shader(&vertex_shader);
        shader_program.attach_shader(&fragment_shader);

        shader_program.link()?;

        shader_program.detach_shader(vertex_shader);
        shader_program.detach_shader(fragment_shader);

        let mut texture = None;

        if texture_path.is_some() {
            let temp_texture = Texture2D::new();
            temp_texture.load(texture_path.unwrap());

            shader_program.use_program();

            let texture_selector = shader_program.get_uniform_location("texture1");
            shader_program.set_uniform_1i(texture_selector, 0);
            texture = Some(temp_texture);
        } else {
            shader_program.use_program();
            let vertex_color_location = shader_program.get_uniform_location("ourColor");
            shader_program.set_uniform_4f(vertex_color_location, (&color.unwrap()).into());
        }

        Ok(Self {
            position,
            vao,
            vbo,
            ebo,
            vertex,
            position_attribute,
            texture_attribute,
            shader_program,
            display_type: DisplayType::Color,
            texture,
            color: color.unwrap_or(Colors::red()),
        })
    }

    pub fn display(&mut self) {
        if self.display_type == DisplayType::Texture {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0);
                self.texture.unwrap().bind();
            }
        }
        self.shader_program.use_program();
        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
    }

    pub fn set_color(&mut self, color: Colors) -> Result<(), Errors> {
        if self.display_type == DisplayType::Texture {
            return Err(Errors::ColorAndTextureError);
        }
        let vertex_color_location = self.shader_program.get_uniform_location("ourColor");
        self.shader_program
            .set_uniform_4f(vertex_color_location, (&color).into());
        self.color = color;
        Ok(())
    }

    pub fn set_position(&mut self, position: Point) {
        self.position = position;
        self.vertex.change_position(position);
        self.vbo.bind();
        self.vbo.buffer_data(self.vertex.clone().into());
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        self.vao.unbind();
        self.vbo.unbind();
        self.ebo.unbind();
        self.position_attribute.disable();
        if self.texture_attribute.is_some() {
            self.texture_attribute.as_ref().unwrap().disable();
        }
        if self.texture.is_some() {
            self.texture.as_ref().unwrap().unbind();
        }
    }
}
