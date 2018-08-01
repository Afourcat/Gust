//! Do all computer graphic convertions
//! ```
//! let pos: f64 = Convert::from_pixel_to_normalized(200, 1920);
//! ```

use nalgebra;
use std;

pub fn from_pixel_to_norm<T: std::ops::Div, U: std::ops::Div>(value: T, screen: U) -> f64 {
	0.1
}