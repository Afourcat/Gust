//! This module encapsulate the system of vertexBuffer
//! Here you can create a drawable object easily with a VertexArray

use crate::draw::{BlendMode, Context, Drawable, DrawableMut, Drawer, IDENTITY};
use crate::gl_utils;
use crate::resources::Resource;
use crate::shader::*;
use crate::texture::Texture;
use crate::vertex::*;
use gl;
use gl::types::*;
use std;
use std::ops::{Index, IndexMut};

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
///         Vertex::new(Vector::new(0.0, 0.0), Vector::new(0.0, 0.0), Color::new(1.0, 0.0, 0.0)),
///         Vertex::new(Vector::new(1.0, 0.0), Vector::new(0.0, 0.0), Color::new(1.0, 0.0, 0.0)),
///         Vertex::new(Vector::new(0.0, 1.0), Vector::new(0.0, 0.0), Color::new(1.0, 0.0, 0.0)),
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
    primitive: GLenum,
    len: usize,
    buffer_type: BufferType
}

#[derive(Debug, Clone, PartialEq, Copy, Hash)]
pub enum BufferType {
    Static,
    Dynamic,
    Stream
}

impl BufferType {
    fn as_gl(&self) -> GLenum {
        match self {
            BufferType::Static => gl::STATIC_DRAW,
            BufferType::Dynamic => gl::DYNAMIC_DRAW,
            BufferType::Stream => gl::STREAM_DRAW,
        }
    }
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
        unsafe {
            gl::DeleteBuffers(1, &[self.id] as *const _);
            gl::GenBuffers(1, &mut self.id);
            gl_utils::alloc_vbo(self.id, &[], self.buffer_type.as_gl());
        }
    }

    pub fn new_typed(t: Primitive, vertice: &[Vertex], buffer_type: BufferType) -> VertexBuffer {
        let mut buffer_id: u32 = 0;
        let len = vertice.len();
        unsafe {
            gl::GenBuffers(1, &mut buffer_id);

            gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);
            // Put data inside
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of::<GLfloat>() * vertice.len() * 8) as GLsizeiptr,
                0 as *const GLvoid,
                gl::DYNAMIC_DRAW
            );
        };

        VertexBuffer {
            id: buffer_id,
            texture: None,
            primitive: t.get_gl_type(),
            len,
            buffer_type: BufferType::Dynamic
        }
    }

    /// Create new Vertex Buffer from vertices
    pub fn new(t: Primitive, vertice: &[Vertex]) -> VertexBuffer {
        let mut buffer_id: u32 = 0;
        let len = vertice.len();
        unsafe {
            gl::GenBuffers(1, &mut buffer_id);

            gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);
            // Put data inside
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of::<GLfloat>() * vertice.len() * 8) as GLsizeiptr,
                0 as *const GLvoid,
                gl::STATIC_DRAW,
            );
        };

        VertexBuffer {
            id: buffer_id,
            texture: None,
            primitive: t.get_gl_type(),
            len,
            buffer_type: BufferType::Static
        }
    }

    #[inline]
    /// Get primitive type
    pub(crate) fn get_gl_type(&self) -> GLenum {
        self.primitive
    }

    #[inline]
    pub fn set_texture(&mut self, texture: &Resource<Texture>) {
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
        unsafe {
            gl::DeleteBuffers(1, &[self.id] as *const _);
            gl::GenBuffers(1, &mut self.id);
            gl_utils::alloc_vbo(self.id, vertice, self.buffer_type.as_gl());
        }
        self.len = vertice.len();
    }

    #[inline]
    /// Get the len of the vertexBuffer
    pub fn len(&self) -> usize {
        self.len
    }

    /// Get the data from the openGL backend.
    pub fn get_data(&mut self) -> &mut [Vertex] {
        let vertex: &mut [Vertex];
        unsafe {
            let ptr: *mut Vertex = std::ptr::null_mut();

            gl::GetBufferSubData(
                gl::ARRAY_BUFFER,
                0,
                self.len as isize,
                ptr as *mut std::ffi::c_void,
            );
            vertex = std::slice::from_raw_parts_mut(ptr, self.len);
        }
        vertex
    }

    #[inline]
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    #[inline]
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    #[inline]
    pub fn texture(&self) -> Option<Resource<Texture>> {
        if let Some(ref texture) = self.texture {
            Some(Resource::clone(texture))
        } else {
            None
        }
    }

    #[inline]
    pub fn reserve(&self, len: usize) {
        unsafe {
            gl_utils::alloc_vbo(self.id, &[], self.buffer_type.as_gl());
        }
    }

    pub fn extend(&mut self, vertice: &[Vertex]) {
        let mut data = self.get_data().to_vec();
        data.extend_from_slice(vertice);
        unsafe {
            gl_utils::alloc_vbo(self.id, &data, self.buffer_type.as_gl());
        }
    }
}

impl DrawableMut for VertexBuffer {
    fn draw_mut<T: Drawer>(&mut self, target: &mut T) {
        self.update();
        self.draw(target);
    }

    fn draw_with_context_mut<T: Drawer>(&mut self, target: &mut T, context: &mut Context) {
        self.update();
        self.draw_with_context(target, context);
    }
}

impl Drawable for VertexBuffer {
    fn draw<T: Drawer>(&self, target: &mut T) {
        let texture = if let Some(ref rc_texture) = self.texture {
            Some(rc_texture.as_ref())
        } else {
            None
        };

        let proj = target.projection();
        let mut context = Context::new(
            texture,
            if texture.is_none() {
                &*NO_TEXTURE_SHADER
            } else {
                &*DEFAULT_SHADER
            },
            vec![
                ("transform".to_string(), &*IDENTITY),
                ("projection".to_string(), &proj),
            ],
            BlendMode::Alpha,
        );
        target.draw_vertex_buffer(self, &mut context);
    }

    fn draw_with_context<T: Drawer>(&self, target: &mut T, context: &mut Context) {
        target.draw_vertex_buffer(self, context);
    }

    fn update(&mut self) {
        unimplemented!("Looking for an interest to implement it.");
    }
}

impl Default for VertexBuffer {
    fn default() -> Self {
        VertexBuffer::new(Primitive::Triangles, &[])
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
