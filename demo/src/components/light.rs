use wme_core::shader::Shader;

use super::component::Component;

extern crate nalgebra_glm as glm;

const POINT_LIGHT_VERTEX_SOURCE: &str = "../resources/shaders/point-light.vert";
const POINT_LIGHT_FRAGMENT_SOURCE: &str = "../resources/shaders/point-light.frag";

pub enum LightType {
    POINT,
}

pub struct Light {
    pub light_type: LightType,
    pub color: glm::Vec3,
    pub intensity: f32,

    shader: Shader,
}

impl Light {

}

impl Component for Light {
}

impl Default for Light {
    fn default() -> Light {
        Light {
            light_type: LightType::POINT,
            color: glm::Vec3::zeros(),
            intensity: 1.0,
            shader: Shader::new(&[POINT_LIGHT_VERTEX_SOURCE, POINT_LIGHT_FRAGMENT_SOURCE])
                .expect("Unable to create shader!"),
        }
    }
}
