use crate::errors::Errors;
use crate::geometry::point::Point;
use crate::graphics::colors::Colors;
use crate::graphics::texture::Texture2D;
use crate::graphics::vertex::{Ebo, Vao, Vbo, VertexAttribute};
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
    position_attribute: VertexAttribute,
    shader_program: ShaderProgram,
    display_type: DisplayType,
    texture: Option<Texture2D>,
    color: Colors,
}

impl Object {
    pub fn new(
        position: Point,
        vertex: &[f32],
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

        let texture_attribute = VertexAttribute::new(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            5 * mem::size_of::<GLfloat>() as GLsizei,
            (3 * mem::size_of::<GLfloat>()) as *const c_void,
        );

        vao.bind();
        vbo.bind();
        vbo.buffer_data(vertex);

        ebo.bind();
        ebo.buffer_data(indices);

        position_attribute.enable();
        texture_attribute.enable();

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
        }

        Ok(Self {
            position,
            vao,
            vbo,
            ebo,
            position_attribute,
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

    pub fn get_color(&self) -> Colors {
        self.color
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        self.vao.unbind();
        self.vbo.unbind();
        self.ebo.unbind();
        self.position_attribute.disable();
    }
}
