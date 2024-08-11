use nalgebra_glm as glm;

/* 
 * packing allows for an array like layout
 * make sure vertex shader in vars match
 * vertex struct order.
 * Ex: layout (location = 0) in vec3 aPos;
 *     will correspond to Vertex.position
 */
#[repr(C, packed)]
pub struct Vertex {
    pub position: glm::Vec3,
    pub texcoord: glm::Vec2,
}
