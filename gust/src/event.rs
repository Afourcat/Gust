//! Module to handle keyboard and mouse event one day

use glfw::{WindowEvent, Key, Action, FlushedMessages};
use glfw;
use window::Window;
use std::rc::Rc;
use std::marker::Send;
use std::sync::mpsc::Receiver;

/// EventReceiver Wrapper for glfw
/// ```
/// let event_r = EventReceiver::from(&window);
///
/// for (_, event) in event_r.fetch() {
/// 	handle_events(/*  omited  */);
/// }
/// ```
///
pub struct EventReceiver {
	event: Rc<Receiver<(f64, WindowEvent)>>,
}

/// Wrapper for flushed message iterator that is simplet to use
pub struct EventIterator<'a, Message: 'a + Send> {
	fmsg: FlushedMessages<'a, Message>,
}

impl<'a> From<&'a EventReceiver> for EventIterator<'a, (f64, WindowEvent)> {
	fn from(var: &'a EventReceiver) -> EventIterator<'a, (f64, WindowEvent)> {
		EventIterator {
			fmsg: glfw::flush_messages(&var.event)
		}
	}
}

impl EventReceiver {
	pub fn new(event: Receiver<(f64, WindowEvent)>) -> EventReceiver {
		EventReceiver {
			event: Rc::new(event),
		}
	}

	pub fn from(window: &Window) -> EventReceiver {
		EventReceiver {
			event: Rc::clone(&window.event),
		}
	}

	pub fn fetch<'a>(&'a self) -> EventIterator<'a, (f64, WindowEvent)> {
		EventIterator::from(&*self)
	}
}

impl<'a, Message: 'static + Send> Iterator for EventIterator<'a, Message> {
	type Item = Message;

	fn next(&mut self) -> Option<Self::Item> {
		self.fmsg.next()
	}
}

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