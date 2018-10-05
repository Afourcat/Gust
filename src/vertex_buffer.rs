//! This module encapsulate the system of vertexBuffer
//! Here you can create a drawable object easily with a VertexArray

use gl;
use gl::types::*;
use std;
use draw::{Drawable,Drawer,Context,BlendMode};
use std::rc::Rc;
use texture::Texture;
use vertex::*;
use shader::*;
use std::ops::{Index,IndexMut};

/// Vertex Buffer structure
#[derive(Debug,Clone,PartialEq)]
pub struct VertexBuffer {
	id: u32,
    texture: Option<Rc<Texture>>,
    array: VertexArray,
    primitive: GLenum,
}

#[derive(Debug,Clone,PartialEq,Copy,Hash)]
pub enum Primitive {
    Triangles,
    Quads,
	TrianglesStrip,
    TriangleFan,
    Points,
    Lines,
}

impl Primitive {
    /// Get primitive type
    pub fn get_gl_type(&self) -> GLenum {
        match self {
            Primitive::Quads        	=> gl::QUADS,
            Primitive::Triangles    	=> gl::TRIANGLES,
            Primitive::Points       	=> gl::POINTS,
            Primitive::Lines        	=> gl::LINES,
			Primitive::TrianglesStrip	=> gl::TRIANGLE_STRIP,
			Primitive::TriangleFan      => gl::TRIANGLE_STRIP,
        }
    }

    pub fn get_primitive(prim: GLenum) -> Primitive {
        match prim {
			gl::QUADS       => Primitive::Quads,
			gl::TRIANGLES   => Primitive::Triangles,
			gl::TRIANGLE_STRIP => Primitive::TrianglesStrip,
			gl::LINES       => Primitive::Lines,
			gl::TRIANGLE_FAN => Primitive::TriangleFan,
			_               => Primitive::Points,
        }
    }
}

impl Into<GLenum> for Primitive {
    fn into(self) -> GLenum {
        self.get_gl_type()
    }
}

impl VertexBuffer {

	fn clear_gl() {
		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
			gl::BindVertexArray(0);
		}
	}

	/// Create new Vertex Buffer for vertices
	pub fn new(t: Primitive, vertice: VertexArray) -> VertexBuffer {
		let mut buffer_id: u32 = 0;

		unsafe {
			gl::GenBuffers(1, &mut buffer_id);
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

			vertice.bind();
			gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);
			// Put data inside
			gl::BufferData(
				gl::ARRAY_BUFFER,
				(std::mem::size_of::<GLfloat>() * vertice.len() * 8) as GLsizeiptr,
				vertice.get_ptr(),
				gl::STATIC_DRAW
			);
			vertice.active();
		};

		let vertex_buffer = VertexBuffer {
			id: buffer_id,
            texture: None,
            primitive: Self::get_gl_type(&t),
            array: vertice,
		};
		Self::clear_gl();

		vertex_buffer
	}

    pub fn append(&mut self, mut vertices: Vec<Vertex>) {
        self.array.array_mut().append(&mut vertices)
    }

    /// Get primitive type
    fn get_gl_type(prim: &Primitive) -> GLenum {
        match prim {
            Primitive::Quads        	=> gl::QUADS,
            Primitive::Triangles    	=> gl::TRIANGLES,
            Primitive::Points       	=> gl::POINTS,
            Primitive::Lines        	=> gl::LINES,
			Primitive::TrianglesStrip	=> gl::TRIANGLE_STRIP,
            Primitive::TriangleFan      => gl::TRIANGLE_FAN
        }
    }

    pub fn get_primitive(&self) -> Primitive {
        match self.primitive {
			gl::QUADS       => Primitive::Quads,
			gl::TRIANGLES   => Primitive::Triangles,
			gl::TRIANGLE_STRIP => Primitive::TrianglesStrip,
			gl::LINES       => Primitive::Lines,
            gl::TRIANGLE_FAN => Primitive::TriangleFan,
			_               => Primitive::Points,
        }
    }

	#[inline]
	pub fn bind(&self) {
		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
		}
	}

	#[inline]
	pub fn unbind(&self) {
		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		}
	}
}

impl Drawable for VertexBuffer {
	fn draw<T: Drawer>(&self, target: &mut T) {
    
        let texture = if let Some(ref rc_texture) = self.texture {
            Some(rc_texture.as_ref())
        } else {
            None
        };
            
	    self.draw_with_context(target, &mut Context::new(
            texture,
			if texture.is_none() {
                &*NO_TEXTURE_SHADER
            } else {
                &*DEFAULT_SHADER
            },
			None,
			BlendMode::Alpha,
		));
	}

    fn draw_with_context<T: Drawer>(&self, target: &mut T, context: &mut Context) {
		unsafe {
			self.setup_draw(context, target);
			self.array.bind();
			self.bind();
			gl::DrawArrays(self.primitive, 0, self.array.len() as i32);
			self.unbind();
		}
    }

	fn update(&mut self) {
        unsafe {
            self.array.bind();
            self.bind();
            gl::BufferSubData(
				gl::ARRAY_BUFFER,
                0,
				(std::mem::size_of::<GLfloat>() * self.array.len() * 8) as GLsizeiptr,
				self.array.get_ptr(),
			);
            self.unbind();
            self.array.unbind();
        }
	}

    fn set_texture(&mut self, texture: &Rc<Texture>) {
        self.texture = Some(Rc::clone(texture));
    }
}

impl Index<usize> for VertexBuffer {
    type Output = Vertex;

    fn index(&self, vertex_index: usize) -> &Vertex {
        &self.array[vertex_index]
    }
}

impl IndexMut<usize> for VertexBuffer {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Vertex {
        &mut self.array[index]
    }
}

impl Default for VertexBuffer {
    fn default() -> Self {
        VertexBuffer::new(Primitive::Points, VertexArray::new(Vec::new()))
    }
}

impl Drop for VertexBuffer {
	fn drop(&mut self) {
		//gl::DeleteVertexArrays(1, self.array);
		//gl::DeleteBuffers(1, self.buffer);
	}
}
