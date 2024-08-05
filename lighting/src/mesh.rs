extern crate gl;
extern crate nalgebra_glm as glm;

pub struct Mesh {
    pub position: glm::Vec3,
    pub vao: u32,
    pub light_vao: u32,

    vbo: u32,
}

impl Mesh {
    pub fn new(position: glm::Vec3) -> Mesh {
        let mut vao: u32 = 0;
        let mut light_vao: u32 = 0;
        let mut vbo: u32 = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(&VERTICES) as gl::types::GLsizeiptr,
                VERTICES.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
            gl::BindVertexArray(vao);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                6 * std::mem::size_of::<f32>() as gl::types::GLint,
                0 as *const gl::types::GLvoid,
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                6 * std::mem::size_of::<f32>() as gl::types::GLint,
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            gl::EnableVertexAttribArray(1);

            gl::GenVertexArrays(1, &mut light_vao);
            gl::BindVertexArray(light_vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                6 * std::mem::size_of::<f32>() as gl::types::GLint,
                0 as *const gl::types::GLvoid,
            );
            gl::EnableVertexAttribArray(0);
        }

        Mesh {
            position,
            vao,
            light_vao,
            vbo,
        }
    }

    pub fn draw(vao: u32) {
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            gl::BindVertexArray(0);
        }
    }

    pub fn destroy(self: &Self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteVertexArrays(1, &self.light_vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

static VERTICES: [f32; 216] = [
    -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.5, 0.5, -0.5, 0.0, 0.0,
    -1.0, 0.5, 0.5, -0.5, 0.0, 0.0, -1.0, -0.5, 0.5, -0.5, 0.0, 0.0, -1.0, -0.5, -0.5, -0.5, 0.0,
    0.0, -1.0, -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.5, 0.5, 0.5, 0.0,
    0.0, 1.0, 0.5, 0.5, 0.5, 0.0, 0.0, 1.0, -0.5, 0.5, 0.5, 0.0, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0,
    0.0, 1.0, -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, -0.5, 0.5, -0.5, -1.0, 0.0, 0.0, -0.5, -0.5, -0.5,
    -1.0, 0.0, 0.0, -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, -0.5, -0.5, 0.5, -1.0, 0.0, 0.0, -0.5, 0.5,
    0.5, -1.0, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 0.5, 0.5, -0.5, 1.0, 0.0, 0.0, 0.5, -0.5,
    -0.5, 1.0, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.0, 0.5, 0.5,
    0.5, 1.0, 0.0, 0.0, -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 0.5,
    -0.5, 0.5, 0.0, -1.0, 0.0, 0.5, -0.5, 0.5, 0.0, -1.0, 0.0, -0.5, -0.5, 0.5, 0.0, -1.0, 0.0,
    -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.5, 0.5, -0.5, 0.0, 1.0,
    0.0, 0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 0.5, 0.5, 0.5, 0.0, 1.0, 0.0, -0.5, 0.5, 0.5, 0.0, 1.0, 0.0,
    -0.5, 0.5, -0.5, 0.0, 1.0, 0.0,
];
