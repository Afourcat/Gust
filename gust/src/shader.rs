//! Shader module

use gl;
use std;
use std::io;
use std::ptr;
use gl::types::*;
use std::io::Read;
use std::fs::File;
use std::ffi::CString;
use nalgebra::{Vector3, Vector2, Vector4, Matrix4, Matrix2, Matrix3};

lazy_static! {
	pub static ref DEFAULT_SHADER: Shader = Shader::default();
}

/// Shader object that abstract openGl type
#[derive(Debug)]
pub struct Shader {
	id: u32,
	vert: u32,
	frag: u32,
}

static VS: &'static str =
"#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in vec3 aColor;
out vec3 ourColor;
out vec2 TexCoord;
uniform mat4 transform;

void main()
{
   gl_Position = transform * vec4(aPos.xy, 0.0, 1.0);
   ourColor = aColor;
   TexCoord = aTexCoord;
}";

static FS: &'static str =
"#version 330 core
out vec4 FragColor;
in vec3 ourColor;
in vec2 TexCoord;
uniform sampler2D ourTexture;

void main()
{
   FragColor = texture(ourTexture, TexCoord) * vec4(ourColor, 1.0);
}";

/// Return a string from a filename
pub fn file_to_cstring(name: &str) -> Result<CString, io::Error> {
		let mut content = String::new();
		File::open(name)?.read_to_string(&mut content)?;
		Ok(CString::new(content.as_bytes()).unwrap())
}

impl Shader {

// Constructors ---------------------------------------------------------------

	/// Create a new Shader from a filename of vertex and frag
	pub fn new(vert: &str, frag: &str) -> Result<Shader, io::Error> {
		let vert_source = file_to_cstring(vert)?;
		let frag_source = file_to_cstring(frag)?;
		let (vert_id, frag_id, id);

		unsafe {
			let (v, f, i) = Shader::do_shader(vert_source, frag_source)?;
			vert_id = v;
			frag_id = f;
			id = i;
		}

		Ok (Shader { id: id, frag: frag_id, vert: vert_id })
	}

	/// Do all everything needed for shaders
	unsafe fn do_shader(vert_code: CString, frag_code: CString)
	-> Result<(u32, u32, u32), io::Error>
	{
		let id = gl::CreateProgram();
		let vert_id = Shader::compile_shader(vert_code, gl::VERTEX_SHADER);
		let frag_id = Shader::compile_shader(frag_code, gl::FRAGMENT_SHADER);


		gl::AttachShader(id, vert_id);
		gl::AttachShader(id, frag_id);
		gl::LinkProgram(id);

		let mut status: i32 = 0;
		let s: String = std::iter::repeat(' ').take(512).collect();
		let info_log: CString = CString::new(s.as_bytes()).unwrap();
		gl::GetProgramiv(id, gl::LINK_STATUS, &mut status);
		if status == 0 {
			gl::GetProgramInfoLog(id, 512, 0 as *mut _, info_log.as_ptr() as *mut _);
			println!("Could not link shaders {}", info_log.into_string().unwrap());
		}
		gl::DeleteShader(vert_id);
		gl::DeleteShader(frag_id);
		Ok((vert_id, frag_id, id))
	}

	/// Compile all shaders
	unsafe fn compile_shader(code: CString, gl_type: GLenum) -> u32 {
		let id = gl::CreateShader(gl_type);
		gl::ShaderSource(id, 1, &code.as_ptr(), ptr::null());
		gl::CompileShader(id);
		let mut success: i32 = 0;
		let s: String = std::iter::repeat(' ').take(512).collect();
		let info_log: CString = CString::new(s.as_bytes()).unwrap();
		gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
		if success == 0 {
			gl::GetShaderInfoLog(id, 512, 0 as *mut _, info_log.as_ptr() as *mut _);
			println!("Could not compile shaders {}", info_log.into_string().unwrap());
		};
		id
	}

// Usable ---------------------------------------------------------------------

	/// Activate the program
	pub fn activate(&self) {
		unsafe {
			gl::UseProgram(self.id);
		}
	}

// Uniform setter Vector

    pub fn uniform_f4(&mut self, name: &str, value: Vector4<f32>) {
        unsafe {
            let pos = gl::GetUniformLocation(self.id, name.as_ptr() as *const _);
            gl::Uniform4f(pos, value.x, value.y, value.z, value.w);
        }
    }

    pub fn uniform_f3(&mut self, name: &str, value: Vector3<f32>) {
        unsafe {
            let pos = gl::GetUniformLocation(self.id, name.as_ptr() as *const _);
            gl::Uniform3f(pos, value.x, value.y, value.z);
        }
    }

    pub fn uniform_f2(&mut self, name: &str, value: Vector2<f32>) {
        unsafe {
            let pos = gl::GetUniformLocation(self.id, name.as_ptr() as *const _);
            gl::Uniform2f(pos, value.x, value.y);
        }
    }

    pub fn uniform_f(&mut self, name: &str, value: f32) {
        unsafe {
            let pos = gl::GetUniformLocation(self.id, name.as_ptr() as *const _);
            gl::Uniform1f(pos, value);
        }
    }

// Uniform setter integer

    pub fn uniform_bool(&mut self, name: &str, value: bool) {
        self.uniform_int(name, value as i32);
    }

    pub fn uniform_int(&mut self, name: &str, value: i32) {
        unsafe {
            let pos = gl::GetUniformLocation(self.id, name.as_ptr() as *const _);
            gl::Uniform1i(pos, value);
        }
    }

    pub fn uniform_int2(&mut self, name: &str, value: Vector2<i32>) {
        unsafe {
            let pos = gl::GetUniformLocation(self.id, name.as_ptr() as *const _);
            gl::Uniform2i(pos, value.x, value.y);
        }
    }

    pub fn uniform_int3(&mut self, name: &str, value: Vector3<i32>) {
        unsafe {
            let pos = gl::GetUniformLocation(self.id, name.as_ptr() as *const _);
            gl::Uniform3i(pos, value.x, value.y, value.z);
        }
    }

    pub fn uniform_int4(&mut self, name: &str, value: Vector4<i32>) {
        unsafe {
            let pos = gl::GetUniformLocation(self.id, name.as_ptr() as *const _);
            gl::Uniform4i(pos, value.x, value.y, value.z, value.w);
        }
    }

// Uniform setter for matrix

    pub fn uniform_mat4f(&self, name: &str, value: &Matrix4<f32>) {
        unsafe {
            let pos = gl::GetUniformLocation(self.id, CString::new(name.as_bytes()).unwrap().as_ptr());
            gl::UniformMatrix4fv(pos, 1, gl::FALSE, value.as_slice().as_ptr());
        }
    }

    pub fn uniform_mat3f(&mut self, name: &str, value: Matrix3<f32>) {
        unsafe {
            let pos = gl::GetUniformLocation(self.id, name.as_ptr() as *const _);
            gl::UniformMatrix3fv(pos, 1, gl::FALSE, value.as_slice().as_ptr());
        }
    }

    pub fn uniform_mat2f(&mut self, name: &str, value: Matrix2<f32>) {
        unsafe {
            let pos = gl::GetUniformLocation(self.id, name.as_ptr() as *const _);
            gl::UniformMatrix2fv(pos, 1, gl::FALSE, value.as_slice().as_ptr());
        }
    }
}

impl Default for Shader {
    /// Default shader mode
    fn default() -> Shader {
		let vert_source = CString::new(VS.as_bytes()).unwrap();
		let frag_source = CString::new(FS.as_bytes()).unwrap();
		let (vert_id, frag_id, id);

		unsafe {
			let (v, f, i) = Shader::do_shader(vert_source, frag_source).unwrap();
			vert_id = v;
			frag_id = f;
			id = i;
		}

		Shader { id: id, frag: frag_id, vert: vert_id }
    }
}

impl Drop for Shader {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteShader(self.vert);
			gl::DeleteShader(self.frag);
		}
	}
}
