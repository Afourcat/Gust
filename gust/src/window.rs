//!  Window module
// Alexandre Fourcat 2018
// window.rs
// Description:
//

static DEFAULT_HEIGHT: f32 = 900.0;
static DEFAULT_WIDTH: f32 = 1600.0;

extern crate glfw;
extern crate gl;

use color::Color;
use std::sync::mpsc::Receiver;
use std::rc::Rc;
use std::ops::Drop;
use draw::{Drawable,Drawer};
use glfw::Context;
use draw;
use nalgebra::Matrix4;

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
    projection: Matrix4<f32>,
//    view: Option<View>,
}

/// Window structure implementation
impl<'a> Window {

    /// Create a new window by default
    pub fn new(height: usize, width: usize, name: &str) -> Window {
        // Init the glfw system

        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        // Create window from Glfw method create_window
        // Return the glfw::WindowEvent enum and a window
        // That we are trying to wrap in this code
        let (mut win, evt) = glfw.create_window(
            height as u32, width as u32,
            name,
            glfw::WindowMode::Windowed
        ).unwrap();

        // Load all the gl function from the user configuration
        gl::load_with(|s| win.get_proc_address(s) as *const _);

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
            //gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        // Make this window usable
        win.make_current();

        Window {
            projection: Matrix4::new_orthographic(0.0, width as f32, 0.0, height as f32, -1.0, 1.0),
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

    /// Init basic gl modules
    fn init_gl() {
        unimplemented!();
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        println!("Dropped");
    }
}

impl Drawer for Window {

    fn draw<T: Drawable>(&mut self, drawable: &T) {
        drawable.draw(self);
    }

    fn draw_with_context<T: Drawable>(&mut self, drawable: &T, context: &mut draw::Context) {
        drawable.draw_with_context(self, context);
    }

    fn get_projection(&self) -> &Matrix4<f32> {
        &self.projection
    }
}

/// Default trait implementation for window
impl Default for Window {
    fn default() -> Window {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut win, evt) = glfw.create_window(
            DEFAULT_HEIGHT as u32, DEFAULT_WIDTH as u32,
            "Gust",
            glfw::WindowMode::Windowed
            ).unwrap();

        win.make_current();

        gl::load_with(|s| win.get_proc_address(s) as *const _);

        Window {
            projection: Matrix4::new_orthographic(0.0, DEFAULT_WIDTH, 0.0, DEFAULT_HEIGHT, -1.0, 1.0),
            height: DEFAULT_HEIGHT as usize,
            width: DEFAULT_WIDTH as usize,
            win: win,
            event: Rc::new(evt),
            clear_color: Color::new(1.0, 1.0, 1.0),
            glf_window: glfw,
            already_init: true,
        }
    }
}
