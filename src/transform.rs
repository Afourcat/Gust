//! Transform
//! this mod is a groupment of all traits dedicated to the movement of a gust entity.
//! You can attach these trait to anything that you need to move.

use nalgebra::Scalar;
use Vector;

/// A trait that define something fully transformable.
pub trait Transformable: Movable + Rotable + Scalable {
    /// Move the sprite off the offset.
    fn contain<T>(&self, offset: Vector<T>) -> bool
    where
        T: Scalar + Into<f32>;

    /// Set the origin of the object.
    fn set_origin<T>(&mut self, origin: Vector<T>)
    where
        T: Scalar + Into<f32>;

    /// Get the origin of the transformable.
    fn get_origin(&self) -> Vector<f32>;
}

/// Trait defining movable structures as sprite or higher
pub trait Movable {
    /// Move the sprite off the offset
    fn translate<T>(&mut self, offset: Vector<T>)
    where
        T: Scalar + Into<f32>;

    /// Set position of the sprite
    fn set_position<T>(&mut self, pos: Vector<T>)
    where
        T: Scalar + Into<f32>;

    /// Get current position
    fn get_position(&self) -> Vector<f32>;
}

/// A trait that define everything that can rotate in a 3D space.
pub trait Rotable {
    /// Rotate from the actual angle with the angle given in argument.
    fn rotate<T>(&mut self, angle: T)
    where
        T: Scalar + Into<f32>;

    /// Set the rotation of the Rotable struct.
    fn set_rotation<T>(&mut self, angle: T)
    where
        T: Scalar + Into<f32>;

    /// Return the rotation of the struct.
    fn get_rotation(&self) -> f32;
}

pub trait Scalable {
    /// Scale the sprite from a factor
    fn scale<T>(&mut self, factor: Vector<T>)
    where
        T: Scalar + Into<f32>;

    /// Set the scale of the sprite
    fn set_scale<T>(&mut self, vec: Vector<T>)
    where
        T: Scalar + Into<f32>;

    /// Get the current scale
    fn get_scale(&self) -> Vector<f32>;
}
