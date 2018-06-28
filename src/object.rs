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
use std::os::raw::c_void;
use std::rc::Rc;
use texture::Texture;

/// Vertex Buffer structure
#[derive(Debug)]
pub struct VertexBuffer {
	buffer: u32,
	array: u32,
    texture: Option<u32>,
    primitive: GLenum,
    size: i32,
}

#[derive(Debug)]
pub enum Primitive {
    Triangles,
    Quads,
    Points,
    Lines,
}

impl VertexBuffer {
	/// Create new Vertex Buffer for vertices
	pub fn new(t: Primitive, vertice: &[f32]) -> VertexBuffer {
		let mut buffer_id: u32 = 0;
		let mut array_id: u32 = 0;
		unsafe {
			gl::GenVertexArrays(1, &mut array_id);
			gl::GenBuffers(1, &mut buffer_id);
			gl::BindVertexArray(array_id);
			gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);
			gl::BufferData(
				gl::ARRAY_BUFFER,
				(std::mem::size_of::<GLfloat>() * vertice.len()) as GLsizeiptr,
				&vertice[0] as *const f32 as *const c_void,
				gl::STATIC_DRAW
			);
			gl::VertexAttribPointer(
						0,
						3,
						gl::FLOAT,
						gl::FALSE,
						(3 * mem::size_of::<GLfloat>()) as GLsizei,
						ptr::null()
			);
			gl::EnableVertexAttribArray(0);
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
			gl::BindVertexArray(0);
		};
		VertexBuffer {
			buffer: buffer_id,
			array: array_id,
            texture: None,
            primitive: Self::get_gl_type(&t),
            size: vertice.len() as i32 / 3,
		}
	}

    /// Get primitive type
    fn get_gl_type(prim: &Primitive) -> GLenum {
        match prim {
            Primitive::Quads        => gl::QUADS,
            Primitive::Triangles    => gl::TRIANGLES,
            Primitive::Points       => gl::POINTS,
            Primitive::Lines        => gl::LINES,
        }
    }

    pub fn get_primitive(&self) -> Primitive {
        match self.primitive {
            gl::QUADS       => Primitive::Quads,
            gl::TRIANGLES   => Primitive::Triangles,
            gl::LINES       => Primitive::Lines,
            _               => Primitive::Points,
        }
    }
}

impl Drawable for VertexBuffer {
	fn draw(&self, window: &mut Window) {
		window.shaders.activate();
		unsafe {
            if (self.texture.is_some()) {
                gl::BindTexture(gl::TEXTURE_2D, self.texture.unwrap());
            }
			gl::BindVertexArray(self.array);
			gl::DrawArrays(self.primitive, 0, self.size as i32);
			gl::BindVertexArray(0);
		}
	}

    fn assign_texture(&mut self, texture: &Texture) {
        self.texture = Some(texture.id);
    }
}
