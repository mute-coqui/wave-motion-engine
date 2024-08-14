use crate::model::Model;
use crate::renderer::Renderer;

use super::Component;

pub struct ModelRenderer {
    pub model: Model,
    // pub material: Material,
}

impl ModelRenderer {
    pub fn new(model: Model) -> ModelRenderer {
        ModelRenderer {
            model,
        }
    }
}

impl Component for ModelRenderer {}

impl Renderer for ModelRenderer {
    fn render(self: Self) {
        self.model.draw_meshes();
    }
}
