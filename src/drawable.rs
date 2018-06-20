/*
** Alexandre Fourcat 2018
** drawable
** Description:
** define Drawable trait
*/

use nalgebra::{Vector2};
use window::Window;

/// Trait that can be use to draw on window
pub trait Drawable {
    fn draw(window: &Window) {
        println!("Test");
    }
}

struct Sprite {
    position: Vector2<f32>,
    sizes: Vector2<usize>,
    //Texture
}

impl Drawable for Sprite {
    fn draw(window: &Window) {
        println!("Impl pls !");
    }
}

