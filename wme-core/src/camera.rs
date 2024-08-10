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
    pub invert_y: bool,
    pub fov: f32,
    pub aspect: f32,
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
            invert_y: camera.invert_y,
            fov: camera.fov,
            aspect: camera.aspect,
        }
    }

    pub fn dolly_camera(self: &mut Self, movement_type: MovementType, delta: f32) {
        let velocity: f32 = self.movement_speed * delta;
        match movement_type {
            MovementType::FORWARD => {
                self.position += self.front * velocity;
            }
            MovementType::BACKWARD => {
                self.position -= self.front * velocity;
            }
            _ => (),
        }
    }

    pub fn pan_camera(self: &mut Self, movement_type: MovementType, delta: f32) {
        let velocity: f32 = self.movement_speed * delta;
        match movement_type {
            MovementType::LEFT => {
                self.position -= self.right * velocity;
            }
            MovementType::RIGHT => {
                self.position += self.right * velocity;
            }
            _ => (),
        }
    }

    pub fn fly_rotate_camera(self: &mut Self, x_offset: f32, y_offset: f32, delta: f32) {
        let new_x: f32 = self.mouse_sensitivity * x_offset * delta;
        let new_y: f32 = self.mouse_sensitivity * y_offset * delta;

        if self.invert_y {
            self.pitch -= new_y;
        } else {
            self.pitch += new_y;
        }
        self.yaw += new_x;

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }
        recalculate_vectors(self);
    }

    pub fn fps_rotate_camera(self: &mut Self, x_offset: f32, y_offset: f32, delta: f32) {
        let current_y_pos = self.position.y;
        self.fly_rotate_camera(x_offset, y_offset, delta);
        self.position.y = current_y_pos;
    }

    pub fn get_view_matrix(self: &Self) -> glm::Mat4 {
        glm::look_at_rh(&self.position, &(self.position + self.front), &self.up)
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
            invert_y: true,
            fov: 45.0,
            aspect: (800 / 600) as f32,
        }
    }
}
