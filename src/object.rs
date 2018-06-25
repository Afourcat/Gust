//
//
//
//
//

use gl;
use gl::types::*;
use std::mem;
use std;
use drawable::Drawable;
use window::Window;
use std::ptr;

/// Vertex Buffer structure
#[derive(Debug)]
pub struct VertexBuffer {
	buffer: u32,
	array: u32,
}

impl VertexBuffer {
	/// Create new Vertex Buffer for vertices
	pub fn new<T>(vertice: &[T]) -> VertexBuffer {
		let mut buffer_id: u32 = 0;
		let mut array_id: u32 = 0;
		unsafe {
			gl::GenVertexArrays(1, &mut array_id);
			gl::GenBuffers(1, &mut buffer_id);
			gl::BindVertexArray(array_id);
			gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);
			// TODO REFACTOR
			gl::BufferData(
				gl::ARRAY_BUFFER,
				(std::mem::size_of::<T>() * vertice.len()) as GLsizeiptr,
				mem::transmute(&vertice[0]),
				gl::STATIC_DRAW
			);
			// TODO: Refactor
			gl::VertexAttribPointer(
						0,
						3,
						gl::FLOAT,
						gl::FALSE,
						(3 * mem::size_of::<T>()) as i32,
						0 as *const _
			);
			gl::EnableVertexAttribArray(0);
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
			gl::BindVertexArray(0);
		};
		VertexBuffer {
			buffer: buffer_id,
			array: array_id,
		}
	}
}

impl Drawable for VertexBuffer {
	fn draw(&self, window: &mut Window) {
		window.shaders[0].activate();
		unsafe {
			gl::BindVertexArray(self.array);
			gl::DrawArrays(gl::TRIANGLES, 0, 3);
			//gl::BindVertexArray(0);
		}
	}
}