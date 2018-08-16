//! This is a module for vertex

use std;
use nalgebra::{Vector3, Vector2, Vector4, Matrix4, Matrix2, Matrix3};
use color::Color;

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