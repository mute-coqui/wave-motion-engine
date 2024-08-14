use crate::camera::Camera;

pub struct Mouse {
    pub first_mouse: bool,
    pub last_x: f32,
    pub last_y: f32,
}

impl Mouse {
    pub fn new(x: f32, y: f32) -> Self {
        Mouse {
            first_mouse: true,
            last_x: x,
            last_y: y,
        }
    }

    pub fn process_mouse(
        self: &mut Self,
        events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
        camera: &mut Camera,
        delta: f32,
    ) {
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    let (xpos, ypos) = (xpos as f32, ypos as f32);
                    if self.first_mouse {
                        self.first_mouse = false;
                        self.last_x = xpos;
                        self.last_y = ypos;
                    }

                    let x_offset: f32 = xpos - self.last_x;
                    let y_offset: f32 = ypos - self.last_y;

                    self.last_x = xpos;
                    self.last_y = ypos;

                    camera.fps_rotate_camera(x_offset, y_offset, delta);
                }
                _ => (),
            }
        }
    }
}
