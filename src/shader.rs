use std::{error::Error, io::Read};

pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(shaders: &[&str]) -> Result<Self, Box<dyn Error>> {
        let vertex_shader = Self::create_shader(shaders[0], gl::VERTEX_SHADER)?;
        let fragment_shader = Self::create_shader(shaders[1], gl::FRAGMENT_SHADER)?;
        let id = Self::create_program(vertex_shader, fragment_shader);

        Ok(Shader { id })
    }

    pub fn use_program(self: &Self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_bool(self: &Self, name: &str, value: bool) {
        self.set_int(name, value as i32)
    }

    pub fn set_int(self: &Self, name: &str, value: i32) {
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.id, name.as_ptr().cast()),
                value as i32,
            );
        }
    }

    pub fn set_float(self: &Self, name: &str, value: f32) {
        unsafe {
            gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr().cast()), value);
        }
    }

    fn create_shader(path: &str, shader_type: gl::types::GLenum) -> Result<u32, Box<dyn Error>> {
        let mut shader_file = std::fs::File::open(path)?;
        let mut contents = String::new();
        shader_file.read_to_string(&mut contents)?;

        let shader_id = unsafe { gl::CreateShader(shader_type) };
        unsafe {
            gl::ShaderSource(
                shader_id,
                1,
                &contents.as_bytes().as_ptr().cast(),
                &contents.len().try_into().unwrap(),
            );
            gl::CompileShader(shader_id);

            let mut success = 0;
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut log_len = 0_i32;
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                gl::GetShaderInfoLog(shader_id, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Shader compile error: {}", String::from_utf8_lossy(&v));
            }
        }

        Ok(shader_id)
    }

    fn create_program(vertex_shader: u32, fragment_shader: u32) -> u32 {
        let id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(id, vertex_shader);
            gl::AttachShader(id, fragment_shader);
            gl::LinkProgram(id);

            let mut success = 0;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut log_len = 0_i32;
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                gl::GetProgramInfoLog(id, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Program linking error: {}", String::from_utf8_lossy(&v));
            }

            gl::DetachShader(id, vertex_shader);
            gl::DetachShader(id, fragment_shader);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        id
    }
}
