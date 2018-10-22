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
use draw::{Drawable,Drawer};
use glfw::Context;
use draw;
use ::Vector;
use view::{View};
use rect::Rect;
use nalgebra;
use nalgebra::Matrix4;
use event::{EventType, EventReceiver};

static DEFAULT_FPS: u32 = 60;

lazy_static! {
    static ref DEFAULT_DELTA: f64 = 1.0 / DEFAULT_FPS as f64;
}

/// Window struct
/// Define a struct by many thing in glfw
pub struct Window {
    pub height: usize,
    pub width: usize,
    event: Rc<Receiver<(f64, glfw::WindowEvent)>>,
    win: glfw::Window,
    clear_color: Color,
    glf_window: glfw::Glfw,
    already_init: bool,
    view: View,
    fps_limit: u32
}

/// Window structure implementation
impl<'a> Window {

    /// Create a new window by default
    pub fn new(width: usize, height: usize, name: &str) -> Window {
        // Init the glfw system

        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        // Create window from Glfw method create_window
        // Return the glfw::WindowEvent enum and a window
        // That we are trying to wrap in this code
        let (mut win, evt) = glfw.create_window(
            width as u32, height as u32,
            name,
            glfw::WindowMode::Windowed
        ).unwrap();


        // Load all the gl function from the user configuration
        gl::load_with(|s| win.get_proc_address(s) as *const _);
        win.set_cursor_mode(glfw::CursorMode::Normal);

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        // Make this window usable
        win.make_current();

        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        Window {
            view: View::from(Rect::new(0.0, 0.0, width as f32, height as f32)),
            height: height,
            width: width,
            win: win,
            event: Rc::new(evt),
            clear_color: Color::new(1.0, 1.0, 1.0),
            glf_window: glfw,
            already_init: true,
            fps_limit: self::DEFAULT_FPS
        }
    }

    pub fn set_mouse_pos<T: nalgebra::Scalar + Into<f32>>(&mut self, vec: Vector<T>) {
        self.win.set_cursor_pos(vec.x.into() as f64, vec.y.into() as f64)
    }

    pub fn poll<T: Into<Option<EventType>>>(&mut self, event: T) {
        Self::match_event_type(self, event.into(), true);
    }

    pub fn unpoll<T: Into<Option<EventType>>>(&mut self, event: T) {
        Self::match_event_type(self, event.into(), false);
    }

    fn match_event_type(window: &mut Window, event: Option<EventType>, active: bool) {
        if event.is_none() {
            window.win.set_all_polling(active);
        } else {
            match event.unwrap() {
                EventType::Key => window.win.set_key_polling(active),
                EventType::Pos => window.win.set_pos_polling(active),
                EventType::Close => window.win.set_close_polling(active),
                EventType::Size => window.win.set_size_polling(active),
                EventType::Refresh => window.win.set_refresh_polling(active),
                EventType::Focus => window.win.set_focus_polling(active),
                EventType::Char => window.win.set_char_polling(active),
                EventType::CharMods => window.win.set_char_mods_polling(active),
                EventType::MouseButton => window.win.set_mouse_button_polling(active),
                EventType::CursorPos => window.win.set_cursor_pos_polling(active),
                EventType::CursorEnter => window.win.set_cursor_enter_polling(active),
                EventType::Scroll => window.win.set_scroll_polling(active),
                EventType::FrameBuffer => window.win.set_framebuffer_size_polling(active)
            }
        }
    }

    pub fn mouse_pos(&self) -> Vector<f32> {
        let pos = self.win.get_cursor_pos();
        Vector::new(pos.0 as f32, pos.1 as f32)
    }

    /// Change cursor to hidden mode
    pub fn hide_cursor(&mut self) {
        self.win.set_cursor_mode(glfw::CursorMode::Hidden);
    }

    /// Change cursor to disabled mode
    pub fn disable_cursor(&mut self) {
        self.win.set_cursor_mode(glfw::CursorMode::Disabled);
    }

    /// Change cursor to normal mode
    pub fn enable_cursor(&mut self) {
        self.win.set_cursor_mode(glfw::CursorMode::Normal);
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

    pub fn set_view(&mut self, view: View) {
        self.view = view;
    }

    /// Display the screen
    pub fn display(&mut self) {
        self.win.swap_buffers();
    }

    /// Init basic gl modules
    fn init_gl() {
        unimplemented!();
    }

    /// Should not be used (low level glfw function)
    fn set_input_mode(&self, im: InputMode) {
        let (mode, value) = im.to_i32();
        unsafe {
            glfw::ffi::glfwSetInputMode(self.win.window_ptr() ,mode, value);
        }
    }

    /// Should not be used (low level glfw function)
    fn get_input_mode(&self, im: InputMode) -> InputMode {
        unsafe {
            InputMode::from(glfw::ffi::glfwGetInputMode(self.win.window_ptr(), im.to_i32().0))
        }
    }

    pub fn get_view(&self) -> &View {
        &self.view
    }

    pub fn get_view_mut(&mut self) -> &mut View {
        &mut self.view
    }

    pub fn set_fps_limit(&mut self, limit: u32) -> u32 {
        let old = self.fps_limit;
        self.fps_limit = limit;
        old
    }

    pub fn fps_limit(&self) -> u32 {
        self.fps_limit
    }

    pub fn limit_fps(&mut self) -> f64 {
        let time = self.glf_window.get_time();
        let limit = 1.0_f64 / self.fps_limit as f64;

        if limit <= time {
            self.glf_window.set_time(0_f64);
            limit - time
        } else {
            0.0
        }
    }

    pub fn event(&self) -> &EventReceiver {
        &self.event
    }

    pub fn event_mut(&mut self) -> &mut EventReceiver {
        &mut self.event
    }
}

impl Drawer for Window {

    fn draw<T: Drawable>(&mut self, drawable: &T) {
        //let time = self.limit_fps();
        drawable.draw(self);
    }

    fn draw_with_context<T: Drawable>(&mut self, drawable: &T, context: &mut draw::Context) {
        drawable.draw_with_context(self, context);
    }

    fn get_projection(&self) -> &Matrix4<f32> {
        self.view.get_projection()
    }

    fn get_sizes(&self) -> Vector<f32> {
        Vector::new(self.width as f32, self.height as f32)
    }

    fn get_center(&self) -> Vector<f32> {
        Vector::new(self.width as f32 / 2.0, self.height as f32 / 2.0)
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
            view: View::from(Rect::new(0.0, 0.0, DEFAULT_WIDTH as f32, DEFAULT_HEIGHT as f32)),
            height: DEFAULT_HEIGHT as usize,
            width: DEFAULT_WIDTH as usize,
            win: win,
            event: Rc::new(evt),
            clear_color: Color::new(1.0, 1.0, 1.0),
            glf_window: glfw,
            already_init: true,
            fps_limit: self::DEFAULT_FPS,
        }
    }
}

pub enum InputMode {
    CursorMode(InputState),
    StickMouseButtons,
    StickKeys,
    NotDefined,
}

impl InputMode {
    fn to_i32(&self) -> (i32, i32) {
        match self {
            InputMode::CursorMode(a)        => { (0x00033001, a as *const _ as i32) },
            InputMode::StickKeys            => { (0x00033002, 1) },
            InputMode::StickMouseButtons    => { (0x00033003, 1) },
            InputMode::NotDefined           => { (-1, -1) }
        }
    }
}

impl From<i32> for InputMode {
    fn from(value: i32) -> InputMode {
        match value {
            0x00033001  => InputMode::CursorMode(InputState::Normal),
            0x00033002  => InputMode::StickKeys,
            0x00033003  => InputMode::StickMouseButtons,
            _           => InputMode::NotDefined,
        }
    }
}

pub enum InputState {
    Normal = 0x00034001,
    Hidden =  0x00034002,
    Disable = 0x00034003,
}

