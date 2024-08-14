use crate::components::{Component, Transform};

#[derive(Default)]
pub struct GameObject {
    pub transform: Transform,
    pub components: Vec<Box<dyn Component>>,
}

impl GameObject {
    pub fn add_component(self: &mut Self, component: impl Component + 'static) {
        self.components.push(Box::new(component));
    }
}
