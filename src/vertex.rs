//
//
//
//
//
//

use std;
use nalgebra::{Vector3, Vector2, Vector4, Matrix4, Matrix2, Matrix3};
use color::Color;

/// Vertex structure defined by texture coord, space coors and color
pub struct Vertex {
    pub pos:    Vector2<f32>,
    pub tex:    Vector2<f32>,
    pub color:  Color,
}

impl Vertex {

    pub fn new_with_pos(pos: Vector2<f32>) -> Vertex {
        Vertex {
            pos: pos,
            tex: pos,
            color: Color::new(1.0, 1.0, 1.0),
        }
    }

    pub fn new_with_texture(pos: Vector2<f32>, tex: Vector2<f32>) -> Vertex {
        Vertex {
            pos: pos,
            tex: tex,
            color: Color::new(1.0, 1.0, 1.0),
        }
    }

    pub fn new_with_color(pos: Vector2<f32>, color: Color) -> Vertex {
        Vertex {
            pos: pos,
            tex: pos,
            color: color,
        }
    }

    pub fn new
    (pos: Vector2<f32>, tex: Vector2<f32>, color: Color) -> Vertex {
        Vertex {
            pos: pos,
            tex: tex,
            color: color,
        }
    }
}   
