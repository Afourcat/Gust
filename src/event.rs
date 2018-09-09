//! Module to handle keyboard and mouse event one day

use glfw::{WindowEvent, Key, Action, FlushedMessages};
use glfw;
use window::Window;
use std::rc::Rc;
use std::marker::Send;
use std::sync::mpsc::Receiver;
use std::any::Any;

/// EventReceiver Wrapper for glfw
/// ```ignore
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

/// The event struc is here to wrap glfw message that way
/// the user doesn't have to use the glfw event implementation
pub struct Event {
	 wrapped: Box<Any + Send + 'static>
}

impl Event {
    pub fn new<T: Send + 'static>(elem: T) -> Event {
        Event {
            wrapped: Box::new(elem)
        }
    }

	pub fn into_window_event(self) -> Box<(f64, WindowEvent)> {
		self.wrapped.downcast::<(f64, WindowEvent)>().unwrap()
	}
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
	type Item = Event;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(elem) = self.fmsg.next() {
			Some(Event::new(elem))
		} else {
			None
		}
	}
}

/// Get Pressed Key
pub fn pressed(event: Event) -> Option<Key> {
	let elem = event.into_window_event();

	match elem.1 {
		WindowEvent::Key(value, _, Action::Press, _) => {
			Some(value)
		},
		_ => None,
	}
}

/// Get Released Key
pub fn released(event: Event) -> Option<Key> {

	match event.into_window_event().1 {
		WindowEvent::Key(value, _, Action::Release, _) => {
			Some(value)
		},
		_ => None,
	}
}

/// Get pressed once then keep pushed
pub fn repeat(event: Event) -> Option<Key> {
    match event.into_window_event().1 {
        WindowEvent::Key(value, _, Action::Repeat, _) => Some(value),
        _ => None,
    }
}
