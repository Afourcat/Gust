//! Module to handle keyboard and mouse event one day
use std::sync::mpsc::Receiver;
use glfw::{Key, Action, FlushedMessages, MouseButton};
use glfw;
pub use glfw::WindowEvent as Events;
use std::rc::Rc;

type EventFunction = fn(Event) -> Result<(),String>;

pub type Event = (f64, Events);

pub fn fetch(event: &Event) -> &Events {
        &event.1
}

pub struct EventHandler {
    handlers: Vec<EventFunction>,
    receiver: Rc<Receiver<Event>>,
}

pub fn default_event(event: Event) -> Result<(),String> {
    Ok(())
}

impl EventHandler {
    pub fn new(window: &::window::Window) -> EventHandler {
        let mut handlers = Vec::with_capacity(15);

        for i in 0..14 {
            handlers.push(default_event as EventFunction);
        }

        EventHandler {
            handlers: handlers,
            receiver: Rc::clone(&window.event)
        }
    }

    fn get_handlers(event: &Events) -> i32 {
        return match event {
            Events::Pos(_, _) => 0,
            Events::Size(_, _) => 1,
            Events::Close => 2,
            Events::Refresh => 3,
            Events::Focus(_) => 4,
            Events::Iconify(_) => 5,
            Events::FramebufferSize(_, _) => 6,
            Events::MouseButton(_, _, _) => 7,
            Events::CursorPos(_, _) => 8,
            Events::CursorEnter(_) => 9,
            Events::Scroll(_, _) => 10,
            Events::Key(_, _, _, _) => 11,
            Events::Char(_) => 12,
            Events::CharModifiers(_, _) => 13,
            Events::FileDrop(_) => 14
        };
    }

    pub fn register_callback(&mut self, function: EventFunction, event: Events) {
        let index = Self::get_handlers(&event) as usize;

        self.handlers[index] = function;
    }
    
    pub fn handle(&self) -> Result<(),String> {
        for event in glfw::flush_messages(&*self.receiver) { 
            let index = Self::get_handlers(&event.1) as usize;
        
            if let Err(a) = self.handlers[index](event) {
                return Err(a);
            }
        }
        Ok(())
    }
}
