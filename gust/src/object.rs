//! This module encapsulate the system of vertexBuffer
//! Here you can create a drawable object easily with a VertexArray

use gl;
use gl::types::*;
use std::mem;
use std;
use draw::{Drawable,Drawer};
use std::ptr;
use std::os::raw::c_void;
use std::rc::Rc;
use texture::Texture;
use vertex::Vertex;
use nalgebra::Matrix4;

lazy_static! {
	static ref PROJECTION: Matrix4<f32> = Matrix4::new_orthographic(0.0, 900.0, 0.0, 1600.0, -1.0, 1.0);
}

/// Vertex Buffer structure
#[derive(Debug,Clone,PartialEq)]
pub struct VertexBuffer {
	buffer: u32,
	array: u32,
    texture: Option<Rc<Texture>>,
    primitive: GLenum,
    size: usize,
}

#[derive(Debug,Clone,PartialEq,Copy)]
pub enum Primitive {
    Triangles,
    Quads,
	TrianglesStrip,
    Points,
    Lines,
}

impl VertexBuffer {
	/// Create new Vertex Buffer for vertices
	pub fn new(t: Primitive, vertice: &[f32]) -> VertexBuffer {
		let mut buffer_id: u32 = 0;
		let mut array_id: u32 = 0;
		unsafe {
			// --------------------------------
			// Buffers generations heere
			// we create a vertexArray and a buffer.
			// Then we Bind the VertexArray the buffer
			// to the openGl state machine.
			// After we put data inside the buffer.
			// Then we cut the data inside the buffer in 3
			// { 1.0, 1.0, 0.0, 1.0, 3.0, 3.0 }
			// |   pos  | texCoord |  color  |
			// |        |          |         |
			// With the 3 VertexAttribPointer
			// --------------------------------
			gl::GenVertexArrays(1, &mut array_id);
			gl::GenBuffers(1, &mut buffer_id);
			gl::BindVertexArray(array_id);
			gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);
			// Put data inside
			gl::BufferData(
				gl::ARRAY_BUFFER,
				(std::mem::size_of::<GLfloat>() * vertice.len()) as GLsizeiptr,
				&vertice[0] as *const f32 as *const c_void,
				gl::STATIC_DRAW
			);
			// Attrib to position to vertexBuffer
			gl::VertexAttribPointer(
						0,
						2,
						gl::FLOAT,
						gl::FALSE,
						(8 * mem::size_of::<GLfloat>()) as GLsizei,
						ptr::null()
			);
            gl::EnableVertexAttribArray(0);
			// Attrib texCoord to VertexBuffer
			gl::VertexAttribPointer(
						1,
						2,
						gl::FLOAT,
						gl::FALSE,
						(8 * mem::size_of::<GLfloat>()) as GLsizei,
						(2 * mem::size_of::<GLfloat>()) as *const _,
			);
			gl::EnableVertexAttribArray(1);
			// Attrib color to VertexBuffer
            gl::VertexAttribPointer(
						2,
						4,
						gl::FLOAT,
						gl::FALSE,
						(8 * mem::size_of::<GLfloat>()) as GLsizei,
						(4 * mem::size_of::<GLfloat>()) as *const _,
			);
			gl::EnableVertexAttribArray(2);
			// Clear openGl state machine
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
			gl::BindVertexArray(0);
		};
		VertexBuffer {
			buffer: buffer_id,
			array: array_id,
            texture: None,
            primitive: Self::get_gl_type(&t),
			size: vertice.len() / 8,
		}
	}

    pub fn new_from_vertex_array(t: Primitive, vertice: &[Vertex])
    -> VertexBuffer {
        let mut new_vertice: Vec<f32> = vec![1.0; vertice.len() * 8];
        let mut i = 0;

        for elem in vertice {
            new_vertice[i]     =     elem.pos.x;
            new_vertice[i + 1] =     elem.pos.y;
            new_vertice[i + 2] =     elem.tex.x;
            new_vertice[i + 3] =     elem.tex.y;
            new_vertice[i + 4] =     elem.color.0;
            new_vertice[i + 5] =     elem.color.1;
            new_vertice[i + 6] =     elem.color.2;
			new_vertice[i + 7] =	 elem.color.3;
            i += 8;
        }
        VertexBuffer::new(t, new_vertice.as_slice())
    }

    /// Get primitive type
    fn get_gl_type(prim: &Primitive) -> GLenum {
        match prim {
            Primitive::Quads        	=> gl::QUADS,
            Primitive::Triangles    	=> gl::TRIANGLES,
            Primitive::Points       	=> gl::POINTS,
            Primitive::Lines        	=> gl::LINES,
			Primitive::TrianglesStrip	=> gl::TRIANGLE_STRIP,
        }
    }

    pub fn get_primitive(&self) -> Primitive {
        match self.primitive {
			gl::QUADS       => Primitive::Quads,
			gl::TRIANGLES   => Primitive::Triangles,
			gl::TRIANGLE_STRIP => Primitive::TrianglesStrip,
			gl::LINES       => Primitive::Lines,
			_               => Primitive::Points,
        }
    }
}

impl Drawable for VertexBuffer {
	fn draw<T: Drawer>(&self, window: &mut T) {
		window.activate_shader();
		window.get_shader().uniform_mat4f("projection", *PROJECTION);
		unsafe {
            if let Some(ref tex) = self.texture {
                tex.active(0);
            }
			gl::BindVertexArray(self.array);
			gl::DrawArrays(self.primitive, 0, self.size as i32);
			gl::BindVertexArray(0);
		}
	}

	fn update(&mut self) {
		unimplemented!();
	}

    fn assign_texture(&mut self, texture: Rc<Texture>) {
        self.texture = Some(Rc::clone(&texture));
    }
}

impl Drop for VertexBuffer {
	fn drop(&mut self) {
		//gl::DeleteVertexArrays(1, self.array);
		//gl::DeleteBuffers(1, self.buffer);
	}
}
