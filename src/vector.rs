//
//
//
//
//

#[derive(Debug,Clone,Copy,PartialEq,PartialOrd)]
pub struct Vector2<T> {
	x: T,
	y: T,
}

#[derive(Debug,Clone,Copy,PartialEq,PartialOrd)]
pub struct Vector3<T> {
	x: T,
	y: T,
	z: T,
}

impl<T> Vector2<T> {
	pub fn new(x: T, y: T) -> Vector2<T> {
		Vector2 {
			x: x,
			y: y,
		}
	}
}