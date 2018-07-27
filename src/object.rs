use gl;
use gl::types::*;
use std::mem;
use std;
use draw::{Drawable,Drawer};
use window::Window;
use std::ptr;
use std::os::raw::c_void;
use std::rc::Rc;
use texture::Texture;
use vertex::Vertex;

/// Vertex Buffer structure
#[derive(Debug)]
pub struct VertexBuffer {
	buffer: u32,
	array: u32,
    texture: Option<Rc<Texture>>,
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
						2,
						gl::FLOAT,
						gl::FALSE,
						(7 * mem::size_of::<GLfloat>()) as GLsizei,
						ptr::null()
			);
            gl::EnableVertexAttribArray(0);
			gl::VertexAttribPointer(
						1,
						2,
						gl::FLOAT,
						gl::FALSE,
						(7 * mem::size_of::<GLfloat>()) as GLsizei,
						(2 * mem::size_of::<GLfloat>()) as *const _,
			);
			gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
						2,
						3,
						gl::FLOAT,
						gl::FALSE,
						(7 * mem::size_of::<GLfloat>()) as GLsizei,
						(4 * mem::size_of::<GLfloat>()) as *const _,
			);
			gl::EnableVertexAttribArray(2);
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
			gl::BindVertexArray(0);
		};
		VertexBuffer {
			buffer: buffer_id,
			array: array_id,
            texture: None,
            primitive: Self::get_gl_type(&t),
            size: vertice.len() as i32 / 2,
		}
	}

    pub fn new_from_vertex_array(t: Primitive, vertice: &[Vertex])
    -> VertexBuffer {
        let mut new_vertice: Vec<f32> = vec![0.0; vertice.len() * 8];
        let mut i = 0;

        for elem in vertice {
            new_vertice[i]     =     elem.pos.x;
            new_vertice[i + 1] =     elem.pos.y;
            new_vertice[i + 2] =     elem.tex.x;
            new_vertice[i + 3] =     elem.tex.y;
            new_vertice[i + 4] =     elem.color.0;
            new_vertice[i + 5] =     elem.color.1;
            new_vertice[i + 6] =     elem.color.2;
            i += 7;
        }
        VertexBuffer::new(t, new_vertice.as_slice())
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
	fn draw<T: Drawer>(&self, window: &mut T) {
		window.activate_shader();
		unsafe {
            if let Some(ref tex) = self.texture {
                tex.active(0);
            }
			gl::BindVertexArray(self.array);
			gl::DrawArrays(self.primitive, 0, self.size as i32);
			gl::BindVertexArray(0);
		}
	}

    fn assign_texture(&mut self, texture: Rc<Texture>) {
        self.texture = Some(Rc::clone(&texture));
    }
}
