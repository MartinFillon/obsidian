use crate::errors::Errors;
use crate::shaders::shader::ShaderType::FragmentShader;
use crate::shaders::shader::{Shader, ShaderProgram};
use glfw::{Context, GlfwReceiver};

pub struct Window {
    glfw: glfw::Glfw,
    window_handle: glfw::PWindow,
    events: GlfwReceiver<(f64, glfw::WindowEvent)>,
    shader_program: ShaderProgram,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Result<Self, Errors> {
        let mut glfw = match glfw::init(glfw::fail_on_errors) {
            Ok(glfw) => glfw,
            Err(_) => return Err(Errors::WindowInitError),
        };

        let (mut window_handle, events) =
            match glfw.create_window(width, height, title, glfw::WindowMode::Windowed) {
                Some((window_handle, events)) => (window_handle, events),
                None => return Err(Errors::WindowInitError),
            };

        window_handle.set_framebuffer_size_polling(true);
        window_handle.set_key_polling(true);
        window_handle.make_current();
        gl::load_with(|symbol| window_handle.get_proc_address(symbol) as *const _);

        let mut shader_program = ShaderProgram::new();
        let fragment_shader =
            Shader::new(FragmentShader, include_str!("../shaders/shader.frag")).unwrap();
        let vertex_shader =
            Shader::new(FragmentShader, include_str!("../shaders/shader.vert")).unwrap();

        fragment_shader.compile().unwrap();
        vertex_shader.compile().unwrap();

        shader_program.attach_shader(&vertex_shader);
        shader_program.attach_shader(&fragment_shader);

        shader_program.link().unwrap();

        shader_program.detach_shader(vertex_shader);
        shader_program.detach_shader(fragment_shader);

        Ok(Self {
            glfw,
            window_handle,
            events,
            shader_program,
        })
    }

    pub fn should_close(&mut self) -> bool {
        self.window_handle.should_close()
    }

    fn process_events(&mut self) {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height);
                },
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    self.window_handle.set_should_close(true)
                }
                _ => {}
            }
        }
    }

    pub fn update(&mut self) {
        self.shader_program.use_program();
        self.process_events();
        self.glfw.poll_events();
        self.window_handle.swap_buffers();
    }
}
