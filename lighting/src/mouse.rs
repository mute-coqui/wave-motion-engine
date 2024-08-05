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
}
