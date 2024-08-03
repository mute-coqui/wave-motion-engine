extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

use std::error::Error;
use std::ffi::CString;
use std::time::Instant;

use glfw::{Action, Context, Key, WindowHint};
use wave_motion_engine::shader::Shader;
use wave_motion_engine::texture::Texture;

fn main() -> Result<(), Box<dyn Error>> {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw
        .create_window(800, 600, "Wave Motion Engine", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|symbol| window.get_proc_address(symbol));

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    window.set_framebuffer_size_callback(frame_buffer_size_callback);

    let basic_cube: BasicCube = BasicCube::new("./resources/textures/container.jpg");

    let shaders: [&str; 2] = [
        "./resources/shaders/basic.vert",
        "./resources/shaders/basic.frag",
    ];

    let shader = Shader::new(&shaders)?;
    shader.use_program();
    let texture_uniform = CString::new("texture1".to_string()).unwrap();
    shader.set_int(&texture_uniform, 0);

    let cube_positions: [glm::Vec3; 10] = [
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(2.0, 5.0, -15.0),
        glm::vec3(-1.5, -2.2, -2.5),
        glm::vec3(-3.8, -2.0, -12.3),
        glm::vec3(2.4, -0.4, -3.5),
        glm::vec3(-1.7, 3.0, -7.5),
        glm::vec3(1.3, -2.0, -2.5),
        glm::vec3(1.5, 2.0, -2.5),
        glm::vec3(1.5, 0.2, -1.5),
        glm::vec3(-1.3, 1.0, -1.5),
    ];

    let fov: f32 = 45.0;
    let aspect: f32 = (800 / 600) as f32;
    let mut frame_time: f32 = 0.0;
    let mut last_draw_time: Instant = Instant::now();

    let model_uniform = CString::new("model".to_string()).unwrap();
    let view_uniform = CString::new("view".to_string()).unwrap();
    let projection_uniform = CString::new("projection".to_string()).unwrap();

    while !window.should_close() {
        let now: Instant = Instant::now();
        let elapsed_time: f32 = now.duration_since(last_draw_time).as_secs_f32();

        if elapsed_time > 0.02 {
            frame_time += elapsed_time;
            last_draw_time = now;
        }
        // process input
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_events(&mut window, event);
        }

        // render
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let view: glm::Mat4 = glm::translate(&glm::Mat4::identity(), &glm::vec3(0.0, 0.0, -3.0));
        let projection: glm::Mat4 = glm::perspective(aspect, fov.to_radians(), 0.1, 100.0);

        shader.set_mat4(&view_uniform, view);
        shader.set_mat4(&projection_uniform, projection);

        for (idx, pos) in cube_positions.iter().enumerate() {
            let mut model: glm::Mat4 = glm::translate(&glm::Mat4::identity(), pos);
            let angle = (idx + 1) as f32 / 3.0 * frame_time;
            model = glm::rotate(&model, angle, &glm::vec3(1.0, 0.3, 0.5));
            shader.set_mat4(&model_uniform, model);
            basic_cube.draw();
        }

        // check events and swap buffers
        glfw.poll_events();
        window.swap_buffers();
    }

    Ok(())
}

fn frame_buffer_size_callback(_window: &mut glfw::Window, width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}

fn handle_window_events(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}

struct BasicCube {
    vao: u32,
    ebo: u32,
    texture: Texture,
}

impl BasicCube {
    fn new(texture_path: &str) -> Self {
        let vertices: [f32; 180] = [
            -0.5, -0.5, -0.5, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5,
            0.5, -0.5, 1.0, 1.0, -0.5, 0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 0.0, -0.5, -0.5,
            0.5, 0.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0,
            1.0, -0.5, 0.5, 0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5, 0.5, 1.0, 0.0,
            -0.5, 0.5, -0.5, 1.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0,
            -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5,
            -0.5, 1.0, 1.0, 0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, 0.5,
            0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, -0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, -0.5, 1.0,
            1.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, -0.5, -0.5, 0.5, 0.0, 0.0,
            -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, 0.5, -0.5, 0.0, 1.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5,
            0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, -0.5, 0.5, 0.5, 0.0, 0.0, -0.5, 0.5, -0.5,
            0.0, 1.0,
        ];
        let indices: [u32; 6] = [0, 1, 3, 1, 2, 3];

        let mut vbo: u32 = 0;
        let mut vao: u32 = 0;
        let mut ebo: u32 = 0;

        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(&vertices) as gl::types::GLsizeiptr,
                vertices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                5 * std::mem::size_of::<f32>() as gl::types::GLint,
                0 as *const gl::types::GLvoid,
            );
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                5 * std::mem::size_of::<f32>() as gl::types::GLint,
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);

            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                std::mem::size_of_val(&indices) as isize,
                indices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        let texture = Texture::new(texture_path, gl::RGB);
        BasicCube {
            vao,
            ebo,
            texture: texture.expect("Failed to create texture"),
        }
    }

    fn draw(self: &Self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture.id);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            gl::BindVertexArray(0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}
