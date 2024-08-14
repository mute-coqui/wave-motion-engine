use crate::game_objects::GameObject;

use super::component::Component;

extern crate nalgebra_glm as glm;

pub struct Transform {
    pub position: glm::Vec3,
    pub rotation: glm::Vec3,
    pub scale: glm::Vec3,
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            position: glm::Vec3::zeros(),
            rotation: glm::Vec3::zeros(),
            scale: glm::Vec3::identity(),
        }
    }
}

impl Component for Transform {
}
