use wme_core::shader::Shader;

extern crate nalgebra_glm as glm;

pub struct Material {
    pub ambient: glm::Vec3,
    pub diffuse: glm::Vec3,
    pub specular: glm::Vec3,
    pub shininess: f32,
    pub shader: Shader,
}

impl Default for Material {
    fn default() -> Material {
        let material_shaders: [&str; 2] = [
            "../resources/shaders/material.vert",
            "../resources/shaders/material.frag",
        ];
        let shader: Shader = Shader::new(&material_shaders).expect("Unable to load shaders!");

        Material {
            ambient: glm::Vec3::zeros(),
            diffuse: glm::Vec3::zeros(),
            specular: glm::Vec3::zeros(),
            shininess: 1.0,
            shader,
        }
    }
}
