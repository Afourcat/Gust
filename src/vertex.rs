//! This is a module for vertex
//! # Vertex
//!
//! A vertex is a point in space giving information to opengl.
//! x vertex form x point that can be drawed.
//!
//! ```Rust
//! use gust::Vector;
//!
//! let position = Vector::new(100.0, 100.0);
//! let texture_coord = Vector::new(0.0, 0.0);
//! let color = Color::new(1.0, 0.3, 0.5);
//!
//! Vertex::new(position, texture_coord, color);
//! ````
//! # VertexArray
//!
//! A vertex array is a Vec of Vertex that can be drawn. It's the littlest
//! object that can be printed.
//!
//! ```Rust
//! let pos_vec vec![
//!     Vector::new(0.0, 0.0);
//!     Vector::new(10.0, 0.0);
//!     Vector::new(10.0, 10.0);
//!     Vector::new(0.0, 10.0);
//! ];
//!
//! let coord_vec = vec![
//!     Vector2::new(0.0, 0.0),
//!     Vector2::new(1.0, 0.0),
//!     Vector2::new(1.0, 1.0),
//!     Vector2::new(0.0, 1.0),
//! ];
//!
//! let vertice = vec![
//!     Vertex::new(pos_vec[0], coord_vec[0], Color::white()),
//!     Vertex::new(pos_vec[1], coord_vec[1], Color::white()),
//!     Vertex::new(pos_vec[2], coord_vec[2], Color::white()),
//!     Vertex::new(pos_vec[3], coord_vec[3], Color::white()),
//! ];
//!
//! VertexArray::new(vertice);
//! ```

use color::Color;
use gl;
use gl::types::*;
use nalgebra::Vector2;
use std::mem;
use std::ops::{Index, IndexMut};
use std::ptr;

/// Vertex structure defined by texture coord, space coors and color
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Vertex {
    pub pos: Vector2<f32>,
    pub tex: Vector2<f32>,
    pub color: Color,
}

impl Vertex {
    /// Create a vertex containing position, texCoord and Color
    pub fn new(pos: Vector2<f32>, tex: Vector2<f32>, color: Color) -> Vertex {
        Vertex { pos, tex, color }
    }
}

impl From<Vector2<f32>> for Vertex {
    /// create a vertex with just a position in 2D space
    fn from(pos: Vector2<f32>) -> Vertex {
        Vertex {
            pos,
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
#[derive(Clone, Debug, PartialEq, Default)]
pub struct VertexArray {
    array: Vec<Vertex>,
    id: u32,
}

impl VertexArray {
    /// Create a empty vertex array
    pub fn new() -> VertexArray {
        let mut id = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        VertexArray {
            array: Vec::new(),
            id,
        }
    }

    pub fn array(&self) -> &Vec<Vertex> {
        &self.array
    }

    pub fn clear(&mut self) {
        self.array.clear();
    }

    pub fn array_mut(&mut self) -> &mut Vec<Vertex> {
        &mut self.array
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
                ptr::null(),
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

    pub fn len(&self) -> usize {
        self.array.len()
    }

    pub fn is_empty(&self) -> bool {
        self.array.len() == 0
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

impl<'a> From<&'a [Vertex]> for VertexArray {
    fn from(array: &[Vertex]) -> VertexArray {
        if array.is_empty() {
            VertexArray::new()
        } else {
            let mut id = 0;
            unsafe { gl::GenVertexArrays(1, &mut id) };
            VertexArray {
                array: Vec::from(array),
                id,
            }
        }
    }
}

impl<'a> From<&'a [f32]> for VertexArray {
    fn from(array: &[f32]) -> VertexArray {
        if array.is_empty() {
            VertexArray::new()
        } else {
            let mut arr = Vec::new();
            for elem in array.windows(8) {
                arr.push(Vertex::new(
                    Vector2::new(elem[0], elem[1]),
                    Vector2::new(elem[2], elem[3]),
                    Color::new(elem[4], elem[5], elem[6]),
                ));
            }
            let mut id = 0;
            unsafe { gl::GenVertexArrays(1, &mut id) };
            VertexArray { array: arr, id }
        }
    }
}

impl Index<usize> for VertexArray {
    type Output = Vertex;

    fn index(&self, vertex_index: usize) -> &Vertex {
        &self.array[vertex_index]
    }
}

impl IndexMut<usize> for VertexArray {
    fn index_mut(&mut self, vertex_index: usize) -> &mut Vertex {
        &mut self.array[vertex_index]
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &[self.id] as *const _);
        }
        println!("VertexArray {} delete.", self.id);
    }
}
