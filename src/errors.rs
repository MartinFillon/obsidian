use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("Window cannot be inited")]
    WindowInitError,

    #[error("Shader cannot be compiled")]
    ShaderCompileError(String),
}
