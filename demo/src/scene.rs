use crate::model::Model;

use crate::{components::{Light, ModelRenderer}, game_objects::GameObject};

pub struct Scene {
    pub children: Vec<GameObject>,
}

impl Default for Scene {
    fn default() -> Scene {
        let mut children: Vec<GameObject> = Vec::new();
        let mut light_source: GameObject = GameObject::default();

        let light_component: Light = Light::default();
        light_source.add_component(light_component);

        let cube_shaders: [&str; 2] = [
            "../resources/shaders/phong-shader-vs.glsl",
            "../resources/shaders/phong-shader-fs.glsl",
        ];

        let cube_model: Model = Model::new(&"../resources/meshes/cube.obj", &cube_shaders);
        let light_model_renderer: ModelRenderer = ModelRenderer::new(cube_model);
        light_source.add_component(light_model_renderer);
        children.push(light_source);
        
        Scene {
            children,
        }
    }
}
