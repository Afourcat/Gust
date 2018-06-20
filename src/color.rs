/*
** Alexandre Fourcat 2018
** color
** Description:
** color gesture
*/

use std::default;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);

static MAGENTA: Color = Color(1.0, 0.0, 1.0, 1.0);
static RED: Color = Color(1.0, 0.0, 0.0, 1.0);

/// Color class
impl Color {
    /// Create a color with the alpha
    pub fn new_alpha(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color(r, g, b, a)
    }

    /// Create a new color
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color(r, g, b, 1.0)
    }
}

// Adding ops trait ----------------------------------------------------<

impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        )
    }
}

impl Sub for Color {
    type Output = Color;
    fn sub(self, other: Color) -> Color {
        Color(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
        )
    }
}

impl Mul for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        Color(
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
            self.3 * other.3,
        )
    }
}

impl Default for Color {
    fn default() -> Color {
        Color (1.0, 1.0, 1.0, 1.0)
    }
}