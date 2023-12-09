use crate::errors::Errors;
use crate::graphics::colors::Colors;
use glfw::{Context, GlfwReceiver, WindowEvent};

pub struct Window {
    glfw: glfw::Glfw,
    window_handle: glfw::PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str, fullscreen: bool) -> Result<Self, Errors> {
        let mut glfw = match glfw::init(glfw::fail_on_errors) {
            Ok(glfw) => glfw,
            Err(_) => return Err(Errors::WindowInitError),
        };

        let monitor = glfw::Monitor::from_primary();

        let mode = if fullscreen {
            glfw::WindowMode::FullScreen(&monitor)
        } else {
            glfw::WindowMode::Windowed
        };

        let (mut window_handle, events) = match glfw.create_window(width, height, title, mode) {
            Some((window_handle, events)) => (window_handle, events),
            None => return Err(Errors::WindowInitError),
        };

        window_handle.set_framebuffer_size_polling(true);
        window_handle.set_key_polling(true);
        window_handle.make_current();
        gl::load_with(|symbol| window_handle.get_proc_address(symbol) as *const _);

        Ok(Self {
            glfw,
            window_handle,
            events,
        })
    }

    pub fn should_close(&mut self) -> bool {
        self.window_handle.should_close()
    }

    pub fn set_should_close(&mut self, value: bool) {
        self.window_handle.set_should_close(value);
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }

    pub fn update(&mut self) -> Vec<WindowEvent> {
        self.window_handle.swap_buffers();
        self.glfw.poll_events();
        glfw::flush_messages(&self.events)
            .map(|(_, event)| event)
            .collect()
    }

    pub fn clear(&mut self, color: Colors) {
        unsafe {
            gl::ClearColor(color.red, color.green, color.blue, color.alpha);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
