//! Module to handle keyboard and mouse event one day
use std::sync::mpsc::Receiver;
use glfw;
pub use glfw::WindowEvent as Events;
use std::rc::Rc;

pub type EventMessage<'a> = glfw::FlushedMessages<'a, (f64, Events)>;

/// Event Wrap glfwEvent data
pub type Event = (f64, Events);

pub struct EventHandler {
    receiver: Rc<Receiver<(f64, glfw::WindowEvent)>>,
}

impl EventHandler {
    pub fn new(window: &::window::Window) -> EventHandler {
        EventHandler {
            receiver: Rc::clone(&window.event)
        }
    }

    pub fn fetch<'a>(&'a self) -> EventIterator<'a> {
        EventIterator::from(&*self)
    }
}

impl<'a> Iterator for EventIterator<'a> {
	type Item = Event;

	fn next(&mut self) -> Option<Self::Item> {
		self.fmsg.next()
	}
}

/// EventIterator is an iterator on eventMessage to glob glfw Event system
pub struct EventIterator<'a> {
	fmsg: EventMessage<'a>
}


impl<'a> From<&'a EventHandler> for EventIterator<'a> {
	fn from(var: &'a EventHandler) -> EventIterator<'a> {
		EventIterator {
			fmsg: glfw::flush_messages(&var.receiver)
		}
	}
}

pub enum EventType {
    Key,
    Pos,
    Close,
    Size,
    Refresh,
    Focus,
    Char,
    CharMods,
    MouseButton,
    CursorPos,
    CursorEnter,
    Scroll,
    FrameBuffer,
}
