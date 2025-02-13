use gl::types::*;
use std::ffi::CString;
use std::fs;
use std::ptr;
use crate::matrix::Matrix;

pub struct Shader {
    pub program: GLuint,
}

impl Shader {
    pub fn new(vert_path: &str, frag_path: &str) -> Result<Self, String> {
        let vertex_shader = Self::compile_shader(vert_path, gl::VERTEX_SHADER)?;
        let fragment_shader = Self::compile_shader(frag_path, gl::FRAGMENT_SHADER)?;

        let program = unsafe { gl::CreateProgram() };

        unsafe {
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            gl::DeleteProgram(vertex_shader);
            gl::DeleteProgram(fragment_shader);
        }

        let mut success: GLint = 0;
        unsafe {
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut info_log = vec![0; 512];
                gl::GetProgramInfoLog(
                    program,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut _
                );
                return Err(format!(
                    "Erreur de linkage du program : {}",
                    String::from_utf8_lossy(&info_log)
                ));
            }
        }

        Ok(Self { program })
    }

    fn compile_shader(path: &str, shader_type: GLenum) -> Result<GLuint, String> {
        let source = fs::read_to_string(path)
            .map_err(|_| format!("Impossible de lire le fichier {}", path))?;
        let c_source = CString::new(source.as_bytes()).unwrap();

        let shader = unsafe { gl::CreateShader(shader_type) };
        unsafe {
            gl::ShaderSource(shader, 1, &c_source.as_ptr(), ptr::null());
            gl::CompileShader(shader);
        }

        let mut success: GLint = 0;
        unsafe {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut info_log = vec![0; 512];
                gl::GetShaderInfoLog(
                    shader,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut _
                );
                return Err(format!(
                    "Erreur de compilation du shader {} : {}",
                    path,
                    String::from_utf8_lossy(&info_log)
                ));
            }
        }

        Ok(shader)
    }

    pub fn find_uniform(&self, name: &str) -> i32 {
        let c_name = CString::new(name).unwrap();
        unsafe {
            gl::GetUniformLocation(self.program, c_name.as_ptr())
        }
    }

    pub fn uniform_matrix(&self, location: i32, matrix: &Matrix) {
        let flat_matrix: [f64; 16] = [
            matrix.data[0][0], matrix.data[0][1], matrix.data[0][2], matrix.data[0][3],
            matrix.data[1][0], matrix.data[1][1], matrix.data[1][2], matrix.data[1][3],
            matrix.data[2][0], matrix.data[2][1], matrix.data[2][2], matrix.data[2][3],
            matrix.data[3][0], matrix.data[3][1], matrix.data[3][2], matrix.data[3][3],
        ];

        unsafe {
            gl::UniformMatrix4fv(location, 1, gl::FALSE, flat_matrix.as_ptr() as *const GLfloat);
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }
    }
}