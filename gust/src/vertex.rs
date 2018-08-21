//! This is a module for vertex

use nalgebra::{Vector2};
use color::Color;
use gl;
use texture::Texture;
use shader::Shader;
use draw::{Drawable,Drawer};
use std::rc::Rc;
use object::Primitive;

/// Vertex structure defined by texture coord, space coors and color
#[derive(Debug,Clone,PartialEq)]
pub struct Vertex {
    pub pos:    Vector2<f32>,
    pub tex:    Vector2<f32>,
    pub color:  Color,
}

impl Vertex {

    /// Create a vertex containing position, texCoord and Color
    pub fn new
    (pos: Vector2<f32>, tex: Vector2<f32>, color: Color) -> Vertex {
        Vertex {
            pos: pos,
            tex: tex,
            color: color,
        }
    }
}

impl From<Vector2<f32>> for Vertex {

    /// create a vertex with just a position in 2D space
    fn from(pos: Vector2<f32>) -> Vertex {
        Vertex {
            pos: pos,
            tex: pos,
            color: Color::new(1.0, 1.0, 1.0),
        }
    }
}

impl From<(Vector2<f32>, Color)> for Vertex {

    /// datas.0 = pos
    /// datas.1 = color
    fn from(datas: (Vector2<f32>, Color)) -> Vertex {
        Vertex {
            pos: datas.0,
            tex: Vector2::new(0.0, 0.0),
            color: datas.1,
        }
    }
}

impl From<(Vector2<f32>, Vector2<f32>)> for Vertex {

    /// datas.0 = position
    /// datas.1 = texCoord
    fn from(datas: (Vector2<f32>, Vector2<f32>)) -> Vertex {
        Vertex {
            pos: datas.0,
            tex: datas.1,
            color: Color::new(1.0, 1.0, 1.0),
        }
    }
}

impl Default for Vertex {

    /// Default vertex
    fn default() -> Vertex {
        Vertex {
            pos: Vector2::new(0.0, 0.0),
            tex: Vector2::new(0.0, 0.0),
            color: Color::white(),
        }
    }
}

/// VertexArray is a vertex data structure that is drawable and it's the basic system
pub struct VertexArray {
	array: Box<[Vertex]>,
	context: Context,
    id: u32,
}

/// Blend mode needed to draw
pub enum BlendMode {
	Alpha,
	Beta,
	Ceta
}

impl BlendMode {
	fn blend_to_gl(&self) {
		match self {
			BlendMode::Alpha => unsafe { gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA) },
			_ => {},
		}
	}
}

/// Context needed to handle a draw of a vertex array
pub struct Context {
	texture: Option<Texture>,
	shader: Option<Shader>,
	blend_mode: BlendMode,
}

impl Default for Context {
	fn default() -> Context {
		Context {
			texture: None,
			shader: None,
			blend_mode: BlendMode::Alpha,
		}
	}
}

impl VertexArray {
	/// Create a new vertex array from a ... vertex array :D
	fn new(array: Box<[Vertex]>) -> VertexArray {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
		VertexArray {
			array: array,
			context: Context::default(),
            id: id,
		}
    }

    /// Low level drawing function that is called by all drawable
    fn draw_vertex_array(&self, prim: Primitive, size: i32, offset: i32) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::DrawArrays(prim.get_gl_type(), offset, size);
        }
    }
}