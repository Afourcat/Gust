//! This module encapsulate the system of vertexBuffer
//! Here you can create a drawable object easily with a VertexArray

use draw::{BlendMode, Context, Drawable, DrawableMut, Drawer, IDENTITY};
use gl;
use gl::types::*;
use resources::Resource;
use shader::*;
use std;
use std::ops::{Index, IndexMut};
use texture::Texture;
use vertex::*;

/// Vertex Buffer structure
#[derive(Debug, Clone, PartialEq)]
/// A vertexbuffer is an buffer object in OpenGl.
/// Here it's linked with VertexArray for data.
/// The VertexBuffer is the 'low levelest' object that is drawable.
/// You can create it from Vertice slice or VertexArray
/// ```no_run
/// use gust::window::Window;
/// use gust::vertex::{VertexArray, Vertex};
/// use gust::Drawable;
///
/// fn main() {
///     let win = Window::default();
///     let vertice = &[
///     Vertex::new(Vector::new(0.0, 0.0), Vector::new(0.0, 0.0), Color::new(1.0, 0.0, 0.0)),
///     Vertex::new(Vector::new(1.0, 0.0), Vector::new(0.0, 0.0), Color::new(1.0, 0.0, 0.0)),
///     Vertex::new(Vector::new(0.0, 1.0), Vector::new(0.0, 0.0), Color::new(1.0, 0.0, 0.0)),
///     ];
///     let triangle = VertexBuffer::from(vertice);
///     while window.is_open() {
///         window.clear();
///         window.draw(&triangle);
///         window.display();
///     }
/// }
/// ```
pub struct VertexBuffer {
    id: u32,
    texture: Option<Resource<Texture>>,
    array: VertexArray,
    primitive: GLenum,
    len: usize,
}

#[derive(Debug, Clone, PartialEq, Copy, Hash)]
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
    pub fn get_gl_type(self) -> GLenum {
        match self {
            Primitive::Quads => gl::QUADS,
            Primitive::Triangles => gl::TRIANGLES,
            Primitive::Points => gl::POINTS,
            Primitive::Lines => gl::LINES,
            Primitive::TrianglesStrip => gl::TRIANGLE_STRIP,
            Primitive::TriangleFan => gl::TRIANGLE_STRIP,
        }
    }

    pub fn get_primitive(prim: GLenum) -> Primitive {
        match prim {
            gl::QUADS => Primitive::Quads,
            gl::TRIANGLES => Primitive::Triangles,
            gl::TRIANGLE_STRIP => Primitive::TrianglesStrip,
            gl::LINES => Primitive::Lines,
            gl::TRIANGLE_FAN => Primitive::TriangleFan,
            _ => Primitive::Points,
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

    /// Clear all data from VertexArray
    pub fn clear(&mut self) {
        self.array.clear();
    }

    /// Create new Vertex Buffer from vertices
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
                gl::STATIC_DRAW,
            );
            vertice.active();
        };

        let vertex_buffer = VertexBuffer {
            id: buffer_id,
            texture: None,
            primitive: Self::get_gl_type(t),
            len: vertice.len(),
            array: vertice,
        };
        Self::clear_gl();

        vertex_buffer
    }

    /// Append data to the actual VertexArray while be updated internaly.
    pub fn append(&mut self, vertices: &[Vertex]) {
        self.array.array_mut().append(&mut Vec::from(vertices));
    }

    /// Get primitive type
    fn get_gl_type(prim: Primitive) -> GLenum {
        match prim {
            Primitive::Quads => gl::QUADS,
            Primitive::Triangles => gl::TRIANGLES,
            Primitive::Points => gl::POINTS,
            Primitive::Lines => gl::LINES,
            Primitive::TrianglesStrip => gl::TRIANGLE_STRIP,
            Primitive::TriangleFan => gl::TRIANGLE_FAN,
        }
    }

    fn set_texture(&mut self, texture: &Resource<Texture>) {
        self.texture = Some(Resource::clone(texture));
    }

    pub fn get_primitive(&self) -> Primitive {
        match self.primitive {
            gl::QUADS => Primitive::Quads,
            gl::TRIANGLES => Primitive::Triangles,
            gl::TRIANGLE_STRIP => Primitive::TrianglesStrip,
            gl::LINES => Primitive::Lines,
            gl::TRIANGLE_FAN => Primitive::TriangleFan,
            _ => Primitive::Points,
        }
    }

    pub fn set_geometry(&mut self, vertice: &[Vertex]) {
        self.array = VertexArray::from(vertice);
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

impl DrawableMut for VertexBuffer {
    fn draw_mut<T: Drawer>(&mut self, target: &mut T) {
        self.update();
        self.draw(target);
    }

    fn draw_with_context_mut(&mut self, context: &mut Context) {
        self.update();
        self.draw_with_context(context);
    }
}

impl Drawable for VertexBuffer {
    fn draw<T: Drawer>(&self, target: &mut T) {
        let texture = if let Some(ref rc_texture) = self.texture {
            Some(rc_texture.as_ref())
        } else {
            None
        };

        let mut context = Context::new(
            texture,
            if texture.is_none() {
                &*NO_TEXTURE_SHADER
            } else {
                &*DEFAULT_SHADER
            },
            vec![
                ("transform".to_string(), &*IDENTITY),
                ("projection".to_string(), target.projection()),
            ],
            BlendMode::Alpha,
        );

        unsafe {
            self.setup_draw(&mut context);
            self.array.bind();
            self.bind();
            gl::DrawArrays(self.primitive, 0, self.array.len() as i32);
            self.unbind();
        }
    }

    fn draw_with_context(&self, context: &mut Context) {
        unsafe {
            self.setup_draw(context);
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

            if self.len != self.array.len() {
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (std::mem::size_of::<GLfloat>() * self.array.len() * 8) as GLsizeiptr,
                    self.array.get_ptr(),
                    gl::STATIC_DRAW,
                );
                self.len = self.array.len();
            }
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (std::mem::size_of::<GLfloat>() * self.array.len() * 8) as GLsizeiptr,
                self.array.get_ptr(),
            );
            self.array.active();
            self.unbind();
            self.array.unbind();
        }
    }
}

impl Index<usize> for VertexBuffer {
    type Output = Vertex;

    fn index(&self, vertex_index: usize) -> &Vertex {
        &self.array[vertex_index]
    }
}

impl IndexMut<usize> for VertexBuffer {
    fn index_mut(&mut self, index: usize) -> &mut Vertex {
        &mut self.array[index]
    }
}

impl Default for VertexBuffer {
    fn default() -> Self {
        VertexBuffer::new(Primitive::Triangles, VertexArray::new())
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &[self.id] as *const _);
        }
        println!("Buffer {} deleted.", self.id);
    }
}
