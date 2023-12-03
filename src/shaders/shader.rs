use crate::errors::Errors;
use gl::types::GLuint;
use std::fs;

#[derive(Clone, Copy)]
pub enum ShaderType {
    VertexShader,
    FragmentShader,
    GeometryShader,
    ComputeShader,
    TessControlShader,
    TessEvaluationShader,
}

pub struct Shader {
    id: GLuint,
    r#type: ShaderType,
    source: String,
}

impl From<ShaderType> for u32 {
    fn from(r#type: ShaderType) -> Self {
        match r#type {
            ShaderType::VertexShader => gl::VERTEX_SHADER,
            ShaderType::FragmentShader => gl::FRAGMENT_SHADER,
            ShaderType::GeometryShader => gl::GEOMETRY_SHADER,
            ShaderType::ComputeShader => gl::COMPUTE_SHADER,
            ShaderType::TessControlShader => gl::TESS_CONTROL_SHADER,
            ShaderType::TessEvaluationShader => gl::TESS_EVALUATION_SHADER,
        }
    }
}

impl Shader {
    pub fn new(r#type: ShaderType, file: &str) -> Result<Self, Errors> {
        let id = unsafe { gl::CreateShader(r#type.clone().into()) };
        let source = match fs::read_to_string(file) {
            Ok(s) => s,
            Err(_) => return Err(Errors::FileNotFound(file.to_string())),
        };
        Ok(Shader { id, r#type, source })
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }

    pub fn get_type(&self) -> ShaderType {
        self.r#type
    }

    pub fn get_source(&self) -> &String {
        &self.source
    }

    pub fn compile(&self) -> Result<(), Errors> {
        let mut success = 0;
        unsafe {
            gl::ShaderSource(
                self.id,
                1,
                &self.source.as_bytes().as_ptr().cast(),
                &self.source.len().try_into().unwrap(),
            );
            gl::CompileShader(self.id);
            gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut success);
        }
        if success == 0 {
            let mut log_len = 0_i32;
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            unsafe {
                gl::GetShaderInfoLog(self.id, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
            }
            Err(Errors::ShaderCompileError(String::from_utf8(v).unwrap()))
        } else {
            Ok(())
        }
    }
}

pub struct ShaderProgram {
    id: GLuint,
    shaders: Vec<Shader>,
}

impl ShaderProgram {
    pub fn new() -> Self {
        let id = unsafe { gl::CreateProgram() };
        Self {
            id,
            shaders: Vec::new(),
        }
    }

    pub fn attach_shader(&mut self, shader: Shader) {
        unsafe {
            gl::AttachShader(self.id, shader.id);
        }
        self.shaders.push(shader);
    }

    pub fn link(&self) -> Result<(), Errors> {
        let mut success = 0;
        unsafe {
            gl::LinkProgram(self.id);
            gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success);
        }
        if success == 0 {
            let mut log_len = 0_i32;
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            unsafe {
                gl::GetProgramInfoLog(self.id, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
            }
            Err(Errors::ShaderCompileError(String::from_utf8(v).unwrap()))
        } else {
            Ok(())
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}
