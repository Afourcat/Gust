//! Shader
//
//
//
//

use gl;
use std;
use std::io;
use std::ptr;
use std::mem;
use gl::types::*;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::ffi::CString;

/// Shader object that abstract openGl type
#[derive(Debug)]
pub struct Shader {
	id: u32,
	vert: u32,
	frag: u32,
}

/// Return a string from a filename
pub fn file_to_cstring(name: &str) -> Result<CString, io::Error> {
		let mut content = String::new();
		File::open(name)?.read_to_string(&mut content)?;
		Ok(CString::new(content.as_bytes()).unwrap())
}

impl Shader {
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

	/// Activate the program
	pub fn activate(&self) {
		unsafe {
			gl::UseProgram(self.id);
		}
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