//! Color handling module

use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);

lazy_static! {
    static ref WHITE: Color = Color(1.0, 1.0, 1.0, 1.0);
    static ref BLACK: Color = Color(0.0, 0.0, 0.0, 1.0);
    static ref RED: Color   = Color(1.0, 0.0, 0.0, 1.0);
    static ref GREEN: Color = Color(0.0, 1.0, 0.0, 1.0);
    static ref BLUE: Color  = Color(0.0, 0.0, 1.0, 1.0);
}

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

    pub fn white() -> Color {
        Color(1.0, 1.0, 1.0, 1.0)
    }

    pub fn red() -> Color {
        Color(1.0, 0.0, 0.0, 1.0)
    }

    pub fn green() -> Color {
        Color(0.0, 1.0, 0.0, 1.0)
    }

    pub fn blue() -> Color {
        Color(0.0, 0.0, 1.0, 1.0)
    }

    pub fn black() -> Color {
        Color(0.0, 0.0, 0.0, 1.0)
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

impl Into<[u8; 4]> for Color {
    fn into(self) -> [u8; 4] {
        [
            (self.0 * 255.0) as u8,
            (self.1 * 255.0) as u8,
            (self.2 * 255.0) as u8,
            (self.3 * 255.0) as u8
        ]
    }
}

impl Into<(u8, u8, u8, u8)> for Color {
    fn into(self) -> (u8, u8, u8, u8) {
        ((self.0 * 255.0) as u8,
         (self.1 * 255.0) as u8,
         (self.2 * 255.0) as u8,
         (self.3 * 255.0) as u8)
    }
}

impl Default for Color {
    fn default() -> Color {
        Color (1.0, 1.0, 1.0, 1.0)
    }
}

#[cfg(test)]
mod test {
    use color::Color;

    #[test]
    fn color_new() {
        let blue = Color::new(0.0, 0.0, 1.0);
        assert!(blue.2 == 1.0);
        assert!(blue.0 == 0.0);
    }

    #[test]
    fn color_add() {
        let red = Color::new(1.0, 0.0, 0.0);
        let blue = Color::new(0.0, 0.0, 1.0);
        let purple = red + blue;

        assert!(purple.0 == 1.0 && purple.2 == 1.0);
    }

    #[test]
    fn color_sub() {
        let green = Color::new(0.0, 1.0, 0.0);
        let marron = Color::new(0.2, 0.2, 0.2);
        let sub = green - marron;

        assert!(sub.0 == -0.2 && sub.1 == 0.8);
    }

    fn color_multiply() {
        let black = Color::new(0.0, 0.0, 0.0);
        let rdm = Color::new(0.4, 0.0, 0.6);
        let multiply = black * rdm;

        assert!(multiply.0 == 0.0 && multiply.2 == 0.0);
    }
}
