//! This is a module for vertex

use nalgebra::{Vector2};
use color::Color;
use gl;
use std::ptr;
use std::mem;
use gl::types::*;
use std::ops::{Index,IndexMut};

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
#[derive(Clone,Debug,PartialEq)]
pub struct VertexArray {
	array: Vec<Vertex>,
    id: u32,
}

impl VertexArray {
	/// Create a new vertex array from a ... vertex array :D
	pub fn new(array: Vec<Vertex>) -> VertexArray {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
		VertexArray {
			array: array,
            id: id,
		}
    }

    pub fn active(&self) {
        unsafe {
            gl::BindVertexArray(self.id);

            // Position (Of each vertex)
            gl::VertexAttribPointer(
                            0,
                            2,
                            gl::FLOAT,
                            gl::FALSE,
                            (8 * mem::size_of::<GLfloat>()) as GLsizei,
                            ptr::null()
            );
            gl::EnableVertexAttribArray(0);
            // Texture Coord (Of each vertex)
            gl::VertexAttribPointer(
                            1,
                            2,
                            gl::FLOAT,
                            gl::FALSE,
                            (8 * mem::size_of::<GLfloat>()) as GLsizei,
                            (2 * mem::size_of::<GLfloat>()) as *const _,
            );
            gl::EnableVertexAttribArray(1);
            // Color (of each vertex)
            gl::VertexAttribPointer(
                            2,
                            3,
                            gl::FLOAT,
                            gl::FALSE,
                            (8 * mem::size_of::<GLfloat>()) as GLsizei,
                            (4 * mem::size_of::<GLfloat>()) as *const _,
            );
            gl::EnableVertexAttribArray(2);
        }
    }

    pub fn from_slice(array: &[f32]) -> Result<VertexArray, &'static str> {
        if array.len() == 0 {
            Err("Array should have data inside.")
        } else {
            let mut arr = Vec::new();
            for elem in array.windows(8) {
                arr.push(Vertex::new(
                                Vector2::new(elem[0],elem[1]),
                                Vector2::new(elem[2], elem[3]),
                                Color::new(elem[4], elem[5], elem[6])
                ));
            }
            Ok(VertexArray::new(arr))
        }
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub unsafe fn get_ptr(&self) -> *const GLvoid {
        self.array.as_ptr() as *const GLvoid
    }
}

impl Index<usize> for VertexArray {
    type Output = Vertex;

    fn index(&self, vertex_index: usize) -> &Vertex {
        &self.array[vertex_index]
    }
}

impl IndexMut<usize> for VertexArray {
    fn index_mut<'a>(&'a mut self, vertex_index: usize) -> &'a mut Vertex {
        &mut self.array[vertex_index]
    }
}
