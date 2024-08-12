extern crate glfw;

use glfw::{Action, Key};

use crate::camera::{Camera, MovementType};

pub fn process_inputs(window: &mut glfw::Window, camera: &mut Camera, delta: f32) {
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
