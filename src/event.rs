//
//
//
//
//

use glfw::{WindowEvent, Key, Action};
use window::Window;

/// Get Pressed Key
pub fn pressed(ref event: &WindowEvent) -> Option<Key> {
	match event {
		WindowEvent::Key(value, _, Action::Press, _) => {
			Some(*value)
		},
		_ => None,
	}
}

/// Get Released Key
pub fn released(ref event: &WindowEvent) -> Option<Key> {
	match event {
		WindowEvent::Key(value, _, Action::Release, _) => {
			Some(*value)
		},
		_ => None,
	}
}
