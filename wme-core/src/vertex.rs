extern crate nalgebra_glm as glm;

pub struct Vertex {
    pub position: glm::Vec3,
    pub normals: glm::Vec3,
    pub tex_coords: glm::Vec2,
}

impl Vertex {
    pub fn new() -> Vertex {
        Vertex {
            position: glm::Vec3::zeros(),
            normals: glm::Vec3::zeros(),
            tex_coords: glm::Vec2::zeros(),
        }
    }
}
