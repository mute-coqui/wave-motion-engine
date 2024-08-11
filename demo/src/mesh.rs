use std::{collections::HashSet, ffi::c_void};

use wme_core::{shader::Shader, texture::Texture, vertex::Vertex};

pub struct Mesh {
    pub vao: u32,
    pub ebo: u32,
    pub indices: Vec<u32>,
    pub vertices: Vec<Vertex>,
    pub textures: HashSet<Texture>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: &HashSet<Texture>) -> Mesh {
        let mut vbo: u32 = 0;
        let mut vao: u32 = 0;
        let mut ebo: u32 = 0;

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
                8 * std::mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                8 * std::mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
                (3 * std::mem::size_of::<gl::types::GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE,
                8 * std::mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
                (5 * std::mem::size_of::<gl::types::GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(2);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        Mesh {
            vao,
            ebo,
            indices,
            vertices,
            textures: textures.clone(),
        }
    }

    pub fn draw(self: &Self, shader: &Shader) {
        unsafe {
            for (idx, texture) in self.textures.iter().enumerate() {
                gl::ActiveTexture(gl::TEXTURE0 + idx as u32);
                shader.set_int(&texture.name, idx as i32);
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
