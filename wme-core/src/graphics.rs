use glfw::{Context, Glfw, GlfwReceiver, InitError, PWindow, WindowEvent, WindowHint};

pub struct Graphics {
    pub screen_width: u32,
    pub screen_height: u32,
    pub glfw: Glfw,
    pub window: PWindow, 
    pub events: GlfwReceiver<(f64, WindowEvent)>,
    pub delta_time: f32,
    pub current_time: f32,
    pub previous_time: f32,
}

impl Graphics {
    pub fn new(width: u32, height: u32) -> Result<Graphics, InitError> {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        glfw.window_hint(WindowHint::ContextVersion(3, 3));
        glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        const SCR_WIDTH: u32 = 1280;
        const SCR_HEIGHT: u32 = 720;

        let (mut window, events) = glfw
            .create_window(
                SCR_WIDTH,
                SCR_HEIGHT,
                "Wave Motion Engine",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window");

        window.set_key_polling(true);
        window.make_current();

        gl::load_with(|symbol| window.get_proc_address(symbol));

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        window.set_framebuffer_size_callback(frame_buffer_size_callback);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        Ok(Graphics {
            screen_width: width,
            screen_height: height,
            glfw,
            window,
            events,
            delta_time: 0.0,
            current_time: 0.0,
            previous_time: 0.0,
        })
    }

    pub fn window_should_close(self: &Self) -> bool {
        self.window.should_close()
    }

    pub fn get_time(self: &Self) -> f32 {
        self.glfw.get_time() as f32
    }

    pub fn update_time(self: &mut Self) {
        self.current_time = self.get_time();
        self.delta_time = self.current_time - self.previous_time;
        self.previous_time = self.current_time;
    }

    pub fn check_events(self: &mut Self) {
        self.glfw.poll_events();
    }

    pub fn swap_buffers(self: &mut Self) {
        self.window.swap_buffers();
    }
}

fn frame_buffer_size_callback(_window: &mut glfw::Window, width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}
