/*
** Alexandre Fourcat 2018
** window.rs
** Description:
**
*/
//!  Window

static TEST: [f32; 9] = [
    -0.5, -0.5, 0.0,
     0.5, -0.5, 0.0,
     0.0,  0.5, 0.0
];

extern crate glfw;
extern crate gl;

use object::{VertexBuffer};
use gl::types::*;
use std::default;
use color::Color;
use drawable::Drawable;
use std::sync::mpsc::Receiver;
use std::cell::RefCell;
use std::rc::Rc;
use event::*;
use glfw::Context;

/// Window struct
/// Define a struct by many thing in glfw
pub struct Window {
    pub height: usize,
    pub width: usize,
    pub event: Rc<Receiver<(f64, glfw::WindowEvent)>>,
    win: glfw::Window,
    clear_color: Color,
    glf_window: glfw::Glfw,
    already_init: bool,
}

/// Window structure implementation
impl<'a> Window {

    /// Create a new window by default
    pub fn new(height: usize, width: usize, name: &str) -> Window {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut win, evt) = glfw.create_window(
            height as u32, width as u32,
            name,
            glfw::WindowMode::Windowed
            ).unwrap();

        win.make_current();

        gl::load_with(|s| win.get_proc_address(s) as *const _);

        Window {
            height: height,
            width: width,
            win: win,
            event: Rc::new(evt),
            clear_color: Color::new(1.0, 1.0, 1.0),
            glf_window: glfw,
            already_init: true,
        }
    }

    /// Check if the window is open
    pub fn is_open(&self) -> bool {
       !self.win.should_close()
    }

    /// Poll the event
    pub fn poll_events(&mut self) {
        self.glf_window.poll_events();
    }

    /// Set clear color
    pub fn set_clear_color(&mut self, new_color: Color) {
        self.clear_color = new_color;
    }

    /// Close window
    pub fn close(&mut self) {
        self.win.set_should_close(true);
    }

    /// Clear screen
    pub fn clear(&self) {
        unsafe {
            gl::ClearColor(
                self.clear_color.0,
                self.clear_color.1,
                self.clear_color.2,
                self.clear_color.3,
            );
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    /// Activate window on OpenGl context
    pub fn active(&mut self) -> bool {
        self.win.make_current();
        self.win.is_current()
    }

    /// Set the key polling mode on
    pub fn set_key_polling(&mut self, is: bool) {
        self.win.set_key_polling(is);
    }

    /// Display the screen
    pub fn display(&mut self) {
        self.win.swap_buffers();
    }

    /// Draw on actual target
    pub fn draw<T: Drawable>(&self, drawable: T) {
        T::draw(self);
    }

    fn init_gl() {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }
    }
}

/// Default trait implementation for window
impl Default for Window {
    fn default() -> Window {
        let glf_window = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (tmp_win, evt) = glf_window.create_window(800, 600, "Window", glfw::WindowMode::Windowed)
            .unwrap();

        Window {
            height: 800,
            width: 600,
            win: tmp_win,
            event: Rc::new(evt),
            clear_color: Color::new(0.0, 0.0, 0.0),
            glf_window: glf_window,
            already_init: true,
        }
    }
}