//
//
//
//
//

use gl;

pub struct VertexBuffer {
	id: u32,
}

pub struct VertexArray {
	id: u32,
}

impl VertexBuffer {
	pub fn new() -> VertexBuffer {
		let mut id = 0;
		let id_address: *mut u32 = &mut id;
		unsafe {
			gl::GenBuffers(1, id_address);
		};
		println!("ID = {}", id);
		VertexBuffer {
			id: id
		}
	}
}