use std::ffi::{c_void, CString};

use nalgebra_glm::{Vec2, Vec3};
use wme_core::{shader::Shader, texture::Texture};

use crate::vertex::Vertex;

pub struct Mesh {
    pub vao: u32,
    pub ebo: u32,
    pub indices: Vec<u32>,
    pub vertices: Vec<Vertex>,
    pub textures: Vec<Texture>,
}

impl Mesh {
    pub fn new(mesh: &tobj::Mesh, materials: &Vec<tobj::Material>) -> Mesh {
        let mut vbo: u32 = 0;
        let mut vao: u32 = 0;
        let mut ebo: u32 = 0;

        let mut vertices: Vec<Vertex> = Vec::new();
        for idx in 0..mesh.positions.len() / 3 {
            let vertex = Vertex {
                position: Vec3::new(
                    mesh.positions[3 * idx + 0],
                    mesh.positions[3 * idx + 1],
                    mesh.positions[3 * idx + 2],
                ),
                texcoord: Vec2::new(mesh.texcoords[2 * idx + 0], mesh.texcoords[2 * idx + 1]),
            };
            vertices.push(vertex);
        }

        let indices: Vec<u32> = mesh.indices.clone();

        let mut textures: Vec<Texture> = Vec::new();
        for material in materials.iter() {
            if let Some(texture_path) = &material.diffuse_texture {
                textures.push(
                    Texture::new(texture_path.as_str(), gl::RGB).expect("Failed to load texture"),
                );
            }
        }

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
                &vertices[0] as *const Vertex as *const c_void,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<gl::types::GLint>()) as gl::types::GLsizeiptr,
                &indices[0] as *const u32 as *const c_void,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                5 * std::mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                5 * std::mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
                (3 * std::mem::size_of::<gl::types::GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        Mesh {
            vao,
            ebo,
            indices,
            vertices,
            textures,
        }
    }

    pub fn draw(self: &Self, shader: &Shader) {
        unsafe {
            for (idx, texture) in self.textures.iter().enumerate() {
                gl::ActiveTexture(gl::TEXTURE0 + idx as u32);
                let diffuse_texture = CString::new("diffuse_texture".to_string()).unwrap();
                shader.set_int(&diffuse_texture, idx as i32);
                gl::BindTexture(gl::TEXTURE_2D, texture.id);
            }
            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );

            gl::BindVertexArray(0);
            gl::ActiveTexture(gl::TEXTURE0);
        }
    }
}
