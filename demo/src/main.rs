extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

use std::error::Error;
use std::ffi::CString;

use demo_russimp::light_cube::LightCube;
use demo_russimp::model::Model;
use glfw::{Action, Context, Key, WindowHint};
use wme_core::camera::{Camera, MovementType};
use wme_core::mouse::Mouse;
use wme_core::shader::Shader;

fn main() -> Result<(), Box<dyn Error>> {
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

    let cube_shaders: [&str; 2] = [
        "../resources/shaders/model-loader-vs.glsl",
        "../resources/shaders/model-loader-fs.glsl",
    ];

    let cube_model: Model =
        Model::new(&"../resources/meshes/cube.obj", &cube_shaders);

    let light_shaders: [&str; 2] = [
        "../resources/shaders/basic-vs.glsl",
        "../resources/shaders/basic-fs.glsl",
    ];

    let light_shader = Shader::new(&light_shaders)?;

    let light_cube_mesh = LightCube::new(glm::Vec3::zeros());

    let u_light_color = CString::new("lightColor".to_string()).unwrap();
    let u_light_pos = CString::new("lightPos".to_string()).unwrap();
    let u_view_pos = CString::new("viewPos".to_string()).unwrap();
    let model_uniform = CString::new("model".to_string()).unwrap();
    let view_uniform = CString::new("view".to_string()).unwrap();
    let projection_uniform = CString::new("projection".to_string()).unwrap();

    let mut camera: Camera = Camera::new(glm::Vec3::new(0.0, 0.0, 5.0));
    camera.mouse_sensitivity = 40.0;
    camera.aspect = SCR_WIDTH as f32 / SCR_HEIGHT as f32;

    let mut delta_time: f32;
    let mut previous_time: f32 = 0.0;

    let mut mouse_data: Mouse = Mouse::new((SCR_WIDTH / 2) as f32, (SCR_HEIGHT / 2) as f32);

    while !window.should_close() {
        let current_time: f32 = glfw.get_time() as f32;
        delta_time = current_time - previous_time;
        previous_time = current_time;

        // process input
        process_inputs(&mut window, &mut camera, delta_time);
        for (_, event) in glfw::flush_messages(&events) {
            process_mouse(event, &mut camera, &mut mouse_data, delta_time);
        }

        // render
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let projection: glm::Mat4 = glm::perspective(
            camera.aspect,
            camera.fov.to_radians(),
            0.1,
            100.0,
        );

        let view: glm::Mat4 = camera.get_view_matrix();

        light_shader.use_program();
        light_shader.set_mat4(&projection_uniform, projection);
        light_shader.set_mat4(&view_uniform, view);

        let mut light_model: glm::Mat4 = glm::Mat4::identity();
        light_model = glm::translate(&light_model, &glm::Vec3::new(1.2, 1.0, 2.0));
        light_model = glm::scale(&light_model, &glm::Vec3::new(0.2, 0.2, 0.2));
        light_shader.set_mat4(&model_uniform, light_model);
        light_cube_mesh.draw(); 


        cube_model.shader.use_program();
        cube_model.shader.set_vec3(&u_light_color, glm::Vec3::new(1.0, 1.0, 1.0));
        cube_model.shader.set_vec3(&u_light_pos, glm::Vec3::new(1.2, 1.0, 2.0));
        cube_model.shader.set_vec3(&u_view_pos, camera.position);
        cube_model.shader.set_mat4(&projection_uniform, projection);
        cube_model.shader.set_mat4(&view_uniform, view);

        let model: glm::Mat4 = glm::Mat4::identity();
        glm::translate(&model, &glm::Vec3::new(0.0, 0.0, 0.0));
        glm::scale(&model, &glm::Vec3::new(1.0, 1.0, 1.0));
        cube_model.shader.set_mat4(&model_uniform, model);
        cube_model.draw_meshes();

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

fn process_inputs(window: &mut glfw::Window, camera: &mut Camera, delta: f32) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
    }
    if window.get_key(Key::Up) == Action::Press {
        camera.dolly_camera(MovementType::FORWARD, delta);
    }
    if window.get_key(Key::Down) == Action::Press {
        camera.dolly_camera(MovementType::BACKWARD, delta);
    }
    if window.get_key(Key::Left) == Action::Press {
        camera.pan_camera(MovementType::LEFT, delta);
    }
    if window.get_key(Key::Right) == Action::Press {
        camera.pan_camera(MovementType::RIGHT, delta);
    }
}

fn process_mouse(
    event: glfw::WindowEvent,
    camera: &mut Camera,
    mouse_data: &mut Mouse,
    delta: f32,
) {
    match event {
        glfw::WindowEvent::CursorPos(xpos, ypos) => {
            let (xpos, ypos) = (xpos as f32, ypos as f32);
            if mouse_data.first_mouse {
                mouse_data.first_mouse = false;
                mouse_data.last_x = xpos;
                mouse_data.last_y = ypos;
            }

            let x_offset: f32 = xpos - mouse_data.last_x;
            let y_offset: f32 = ypos - mouse_data.last_y;

            mouse_data.last_x = xpos;
            mouse_data.last_y = ypos;

            camera.fps_rotate_camera(x_offset, y_offset, delta);
        }
        _ => (),
    }
}
