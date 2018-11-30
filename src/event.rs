//! Module to handle keyboard and mouse event one day
use glfw;
pub use glfw::WindowEvent as Events;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use window::Window;

#[macro_export]
/// Should expand from pressed!(W) -> Events::Key(Key::W, _, Action::Pressed, _)
macro_rules! pressed {
    ($x:ident) => {
        Events::Key(Key::$x, _, Action::Press, _)
    }
}

#[macro_export]
macro_rules! release {
    ($x:ident) => {
        Events::Key(Key::$x, _, Action::Release, _)
    }
}

#[macro_export]
macro_rules! repeat {
    ($x:ident) => {
        Events::Key(Key::$x, _, Action::Repeat, _)
    }
}

pub type EventMessage<'a> = glfw::FlushedMessages<'a, (f64, Events)>;

/// Event Wrap glfwEvent data
pub type Event = (f64, Events);
pub type EventReceiver = Rc<Receiver<(f64, glfw::WindowEvent)>>;

pub struct EventHandler {
    receiver: EventReceiver,
}

impl EventHandler {
    pub fn new(window: &Window) -> EventHandler {
        EventHandler {
            receiver: Rc::clone(window.event()),
        }
    }

    pub fn fetch(&self) -> EventIterator {
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
    fmsg: EventMessage<'a>,
}

impl<'a> From<&'a EventHandler> for EventIterator<'a> {
    fn from(var: &'a EventHandler) -> EventIterator<'a> {
        EventIterator {
            fmsg: glfw::flush_messages(&var.receiver),
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
