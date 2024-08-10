use std::ffi::c_void;

use nalgebra_glm::Vec3;

use crate::vertex::Vertex;

pub struct Mesh {
    pub vao: u32,
    pub ebo: u32,
    pub indices: Vec<u32>,
    pub vertices: Vec<Vertex>,
}

impl Mesh {
    pub fn new(mesh: &tobj::Mesh, _materials: &Vec<tobj::Material>) -> Mesh {
        assert!(
            mesh.positions.len() % 3 == 0,
            "Mesh must be representable in 3d space"
        );

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
            };
            vertices.push(vertex);
        }

        let indices: Vec<u32> = mesh.indices.clone();

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
                3 * std::mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        Mesh {
            vao,
            ebo,
            indices,
            vertices,
        }
    }

    pub fn draw(self: &Self) {
        unsafe {
            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );

            gl::BindVertexArray(0);
        }
    }
}
