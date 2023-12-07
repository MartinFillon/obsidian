use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("Window cannot be inited")]
    WindowInitError,

    #[error("Shader cannot be compiled")]
    ShaderCompileError(String),

    #[error("File not found")]
    FileNotFound(String),

    #[error("Shader Program cannot be linked")]
    ShaderProgramLinkError(String),

    #[error("Cannot set a color and a texture at the same time")]
    ColorAndTextureError,

    #[error("Cannot set a color on a textured object")]
    ColorOnTexturedObjectError,
}
