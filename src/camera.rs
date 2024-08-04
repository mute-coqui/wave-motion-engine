extern crate nalgebra_glm as glm;

pub enum MovementType {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}

pub struct Camera {
    pub position: glm::Vec3,
    pub front: glm::Vec3,
    pub up: glm::Vec3,
    pub right: glm::Vec3,
    pub world_right: glm::Vec3,
    pub pitch: f32,
    pub yaw: f32,
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
}

impl Camera {
    pub fn new(position: glm::Vec3) -> Self {
        let mut camera: Camera = Self::default();
        camera.position = position;
        recalculate_vectors(&mut camera);

        Camera {
            position: camera.position,
            front: camera.front,
            up: camera.up,
            right: camera.right,
            world_right: camera.world_right,
            pitch: camera.pitch,
            yaw: camera.yaw,
            movement_speed: camera.movement_speed,
            mouse_sensitivity: camera.movement_speed,
            zoom: camera.zoom,
        }
    }

    pub fn move_camera(self: &mut Self, movement_type: MovementType, delta: f32) {
        let velocity: f32 = self.movement_speed * delta;
        match movement_type {
            MovementType::FORWARD => {
                self.position -= self.front.scale(velocity);
            }
            MovementType::BACKWARD => {
                self.position += self.front.scale(velocity);
            }
            MovementType::LEFT => {
                self.position -= self.right.scale(velocity);
            }
            MovementType::RIGHT => {
                self.position += self.right.scale(velocity);
            }
        }
    }

    pub fn get_view_matrix(self: &Self) -> glm::Mat4 {
        glm::look_at_lh(&self.position, &(self.position + self.front), &self.up)
    }
}

fn recalculate_vectors(camera: &mut Camera) {
    let mut new_front: glm::Vec3 = glm::Vec3::zeros();
    new_front.x = camera.yaw.to_radians().cos() * camera.pitch.to_radians().cos();
    new_front.y = camera.pitch.to_radians().sin();
    new_front.z = camera.yaw.to_radians().sin() * camera.pitch.to_radians().cos();
    camera.front = glm::normalize(&new_front);
    camera.right = glm::normalize(&glm::cross(&camera.front, &camera.world_right));
    camera.up = glm::normalize(&glm::cross(&camera.right, &camera.front));
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            position: glm::Vec3::zeros(),
            front: glm::Vec3::new(0.0, 0.0, -1.0),
            up: glm::Vec3::new(0.0, 1.0, 0.0),
            right: glm::Vec3::zeros(),
            world_right: glm::Vec3::new(0.0, 1.0, 0.0),
            pitch: 0.0,
            yaw: -90.0,
            movement_speed: 2.5,
            mouse_sensitivity: 0.1,
            zoom: 45.0,
        }
    }
}
