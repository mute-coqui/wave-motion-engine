extern crate nalgebra_glm as glm;

use std::error::Error;

use demo::light_cube::LightCube;
use demo::model::Model;
use wme_core::camera::Camera;
use wme_core::constants::ShaderStrings;
use wme_core::graphics::Graphics;
use wme_core::mouse::Mouse;
use wme_core::shader::Shader;

fn main() -> Result<(), Box<dyn Error>> {
    let mut graphics: Graphics = Graphics::new(1280, 720).expect("Failed to init glfw");

    let shader_strings: ShaderStrings = ShaderStrings::default();

    let cube_shaders: [&str; 2] = [
        "../resources/shaders/phong-shader-vs.glsl",
        "../resources/shaders/phong-shader-fs.glsl",
    ];

    let cube_model: Model = Model::new(&"../resources/meshes/cube.obj", &cube_shaders);

    let light_shaders: [&str; 2] = [
        "../resources/shaders/point-light-vs.glsl",
        "../resources/shaders/point-light-fs.glsl",
    ];

    let light_shader = Shader::new(&light_shaders)?;

    let light_cube_mesh = LightCube::new(glm::Vec3::zeros());

    let mut camera: Camera = Camera::new(glm::Vec3::new(0.0, 0.0, 5.0));
    camera.mouse_sensitivity = 40.0;
    camera.aspect = graphics.screen_width as f32 / graphics.screen_height as f32;

    let mut mouse_data: Mouse = Mouse::new(
        (graphics.screen_width / 2) as f32,
        (graphics.screen_height / 2) as f32,
    );

    while !graphics.window_should_close() {
        graphics.update_time();

        // process input
        wme_core::keyboard::process_inputs(&mut graphics.window, &mut camera, graphics.delta_time);
        mouse_data.process_mouse(&graphics.events, &mut camera, graphics.delta_time);

        // render
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let projection: glm::Mat4 = camera.get_projection_matrix();
        let view: glm::Mat4 = camera.get_view_matrix();

        // Render point light
        let mut light_model: glm::Mat4 = glm::Mat4::identity();
        light_model = glm::translate(&light_model, &glm::Vec3::new(1.2, 1.0, 2.0));
        light_model = glm::scale(&light_model, &glm::Vec3::new(0.2, 0.2, 0.2));

        light_shader.use_program();
        light_shader.set_mat4(&shader_strings.projection_uniform, projection);
        light_shader.set_mat4(&shader_strings.view_uniform, view);
        light_shader.set_mat4(&shader_strings.model_uniform, light_model);
        light_cube_mesh.draw();

        // Render cube
        let model: glm::Mat4 = glm::Mat4::identity();
        glm::translate(&model, &glm::Vec3::new(0.0, 0.0, 0.0));
        glm::scale(&model, &glm::Vec3::new(1.0, 1.0, 1.0));

        cube_model.shader.use_program();
        cube_model
            .shader
            .set_vec3(&shader_strings.u_light_color, glm::Vec3::new(1.0, 1.0, 1.0));
        cube_model
            .shader
            .set_vec3(&shader_strings.u_light_pos, glm::Vec3::new(1.2, 1.0, 2.0));
        cube_model
            .shader
            .set_vec3(&shader_strings.u_view_pos, camera.position);
        cube_model
            .shader
            .set_mat4(&shader_strings.projection_uniform, projection);
        cube_model
            .shader
            .set_mat4(&shader_strings.view_uniform, view);
        cube_model
            .shader
            .set_mat4(&shader_strings.model_uniform, model);
        cube_model.draw_meshes();

        // check events and swap buffers
        graphics.check_events();
        graphics.swap_buffers();
    }

    Ok(())
}
