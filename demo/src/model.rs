use std::collections::HashSet;

use nalgebra_glm::{Vec2, Vec3};
use wme_core::{shader::Shader, texture::Texture, vertex::Vertex};

use crate::mesh::Mesh;

pub struct Model {
    pub shader: Shader,
    pub meshes: Vec<Mesh>,
}

impl Model {
    pub fn new(path: &str, shader_paths: &[&str]) -> Model {
        let shader: Shader = Shader::new(&shader_paths).expect("Failed to load shaders");

        let load_options = tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        };

        let (models, materials) =
            tobj::load_obj(path, &load_options).expect("Failed to OBJ load file");

        let materials = materials.expect("Faild to load MTL file");
        let mut meshes: Vec<Mesh> = Vec::new();

        for m in models.iter() {
            let mesh = &m.mesh;
            let mut vertices: Vec<Vertex> = Vec::new();
            for idx in 0..mesh.positions.len() / 3 {
                let vertex = Vertex {
                    position: Vec3::new(
                        mesh.positions[3 * idx + 0],
                        mesh.positions[3 * idx + 1],
                        mesh.positions[3 * idx + 2],
                    ),
                    texcoord: Vec2::new(mesh.texcoords[2 * idx + 0], mesh.texcoords[2 * idx + 1]),
                    normal: Vec3::new(
                        mesh.normals[3 * idx + 0],
                        mesh.normals[3 * idx + 1],
                        mesh.normals[3 * idx + 2],
                    )
                };
                vertices.push(vertex);
            }

            let indices: Vec<u32> = mesh.indices.clone();

            let mut textures: HashSet<Texture> = HashSet::new();
            if let Some(material_id) = mesh.material_id {
                let material = &materials[material_id];
                if let Some(diffuse_texture) = &material.diffuse_texture {
                    textures.insert(
                        Texture::new(diffuse_texture.as_str(), "diffuse_texture")
                            .expect("Failed to load texture"),
                    );
                }
            }
            meshes.push(Mesh::new(vertices, indices, &textures));
        }

        Model { shader, meshes }
    }

    pub fn draw_meshes(self: &Self) {
        for mesh in self.meshes.iter() {
            mesh.draw(&self.shader);
        }
    }
}
