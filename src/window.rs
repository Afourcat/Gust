//!  Window module
// Alexandre Fourcat 2018
// window.rs
// Description:
//

static DEFAULT_HEIGHT: u32 = 900;
static DEFAULT_WIDTH: u32 = 1600;

extern crate gl;
extern crate glfw;

use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;

use glfw::Context;
use nalgebra;
use nalgebra::Matrix4;

use crate::color::Color;
use crate::draw;
use crate::draw::{Drawable, DrawableMut, Drawer};
use crate::event::{EventReceiver, EventType};
use crate::rect::Rect;
use crate::vertex::{Vertex, VertexArray};
use crate::vertex_buffer::{Primitive, VertexBuffer};
use crate::view::View;
use crate::Vector;

static DEFAULT_FPS: u32 = 60;

lazy_static! {
    static ref DEFAULT_DELTA: f64 = 1.0 / f64::from(DEFAULT_FPS);
}

/// Window struct
/// Define a struct by many thing in glfw
pub struct Window {
    pub height: u32,
    pub width: u32,
    event: Rc<Receiver<(f64, glfw::WindowEvent)>>,
    pub(super) win: glfw::Window,
    clear_color: Color,
    already_init: bool,
    view: View,
    fps_limit: u32,
    vertex_array: u32,
}

lazy_static! {
    static ref GLFW_INSTANCE: Mutex<glfw::Glfw> =
        Mutex::new(glfw::init(glfw::FAIL_ON_ERRORS).unwrap());
}

/// Window structure implementation
impl<'a> Window {
    /// Create a new window by default
    pub fn new(width: u32, height: u32, name: &str) -> Window {
        // Init the glfw system

        let mut glfw = GLFW_INSTANCE.lock().unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        // Create window from Glfw method create_window
        // Return the glfw::WindowEvent enum and a window
        // That we are trying to wrap in this code
        let (mut win, evt) = glfw
            .create_window(width, height, name, glfw::WindowMode::Windowed)
            .unwrap();

        // Load all the gl function from the user configuration
        gl::load_with(|s| win.get_proc_address(s) as *const _);
        win.set_cursor_mode(glfw::CursorMode::Normal);

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        let mut va = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut va);
            crate::gl_utils::update_vao(va);
        }

        Window {
            view: View::from(Rect::new(0.0, 0.0, width as f32, height as f32)),
            height,
            width,
            win,
            event: Rc::new(evt),
            clear_color: Color::new(1.0, 1.0, 1.0),
            already_init: true,
            fps_limit: self::DEFAULT_FPS,
            vertex_array: va,
        }
    }

    pub fn bind_vertex_array(&self) {
        unsafe {
            gl::BindVertexArray(self.vertex_array);
        }
    }

    pub fn set_mouse_pos<T: nalgebra::Scalar + Into<f32>>(&mut self, vec: Vector<T>) {
        self.win
            .set_cursor_pos(f64::from(vec.x.into()), f64::from(vec.y.into()))
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
                EventType::FrameBuffer => window.win.set_framebuffer_size_polling(active),
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
        GLFW_INSTANCE.lock().unwrap().poll_events();
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
        GLFW_INSTANCE
            .lock()
            .unwrap()
            .make_context_current(Some(&self.win));
        true
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
    fn set_input_mode(&self, im: &InputMode) {
        let (mode, value) = im.to_i32();
        unsafe {
            glfw::ffi::glfwSetInputMode(self.win.window_ptr(), mode, value);
        }
    }

    /// Should not be used (low level glfw function)
    fn input_mode(&self, im: &InputMode) -> InputMode {
        unsafe {
            InputMode::from(glfw::ffi::glfwGetInputMode(
                self.win.window_ptr(),
                im.to_i32().0,
            ))
        }
    }

    pub fn view(&self) -> &View {
        &self.view
    }

    pub fn view_mut(&mut self) -> &mut View {
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

    pub fn sizes(&self) -> Vector<u32> {
        Vector::new(self.width, self.height)
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
        self.active();
        drawable.draw(self);
    }

    fn draw_mut<T: DrawableMut>(&mut self, drawable: &mut T) {
        self.active();
        drawable.draw_mut(self);
    }

    fn draw_with_context<T: Drawable>(&mut self, drawable: &T, context: &mut draw::Context) {
        self.active();
        drawable.draw_with_context(self, context);
    }

    fn draw_with_context_mut<T: DrawableMut>(
        &mut self,
        drawable: &mut T,
        context: &mut draw::Context,
    ) {
        self.active();
        drawable.draw_with_context_mut(self, context);
    }

    fn draw_vertices(
        &self,
        vertices: &[Vertex],
        primitive: Primitive,
        context: &mut draw::Context,
    ) {
        unimplemented!("Draw vertices");
    }

    fn draw_vertex_array(&self, vertices: &VertexArray, context: &mut draw::Context) {
        unimplemented!("Vertex Array");
    }

    fn draw_vertex_buffer(&self, vertex_buffer: &VertexBuffer, context: &mut draw::Context) {
        context.setup_draw();
        self.bind_vertex_array();
        vertex_buffer.bind();
        unsafe {
            gl::DrawArrays(vertex_buffer.get_gl_type(), 0, vertex_buffer.len() as i32);
        }
    }

    unsafe fn draw_from_raw(
        &self,
        raw: *const std::ffi::c_void,
        len: usize,
        context: &mut draw::Context,
    ) {
        unimplemented!("Raw");
    }

    fn center(&self) -> Vector<f32> {
        let view_sizes = self.view.sizes();
        Vector::new(view_sizes.x / 2.0, view_sizes.y / 2.0)
    }

    fn sizes(&self) -> Vector<f32> {
        let vec = Window::sizes(self);
        Vector::new(vec.x as f32, vec.y as f32)
    }

    /// Return projection.
    fn projection(&self) -> Matrix4<f32> {
        self.view.projection()
    }
}

//impl Drawer for Window {
//    fn draw<T: Drawable>(&mut self, drawable: &T) {
//        self.active();
//        drawable.draw(self);
//    }
//
//    #[inline]
//    fn draw_mut<T: DrawableMut>(&mut self, drawable: &mut T) {
//        self.active();
//        drawable.draw_mut(self);
//    }
//
//    #[inline]
//    fn draw_with_context<T: Drawable>(&mut self, drawable: &mut T, context: &mut draw::Context) {
//        self.active();
//        drawable.draw_with_context(context);
//    }
//
//    #[inline]
//    fn draw_with_context_mut<T: DrawableMut>(
//        &mut self,
//        drawable: &mut T,
//        context: &mut draw::Context,
//    ) {
//        self.active();
//        drawable.draw_with_context(context);
//    }
//
//    #[inline]
//    fn get_sizes(&self) -> Vector<f32> {
//        Vector::new(self.width as f32, self.height as f32)
//    }
//
//    #[inline]
//    fn get_center(&self) -> Vector<f32> {
//        let view_pos = self.view().postition();
//        let view_zoom = self.view().get_zoom();
//
//        println!("View pos: {:?}", view_pos);
//        Vector::new(
//            (self.width as f32 / (2.0 * (1.0 / view_zoom))) + view_pos.x,
//            (self.height as f32 / (2.0 * (1.0 / view_zoom))) + view_pos.y,
//        )
//    }
//
//    fn projection(&self) -> &Matrix4<f32> {
//        self.view.projection()
//    }
//}

/// Default trait implementation for window
impl Default for Window {
    fn default() -> Window {
        let (mut win, evt) = GLFW_INSTANCE
            .lock()
            .unwrap()
            .create_window(
                DEFAULT_HEIGHT as u32,
                DEFAULT_WIDTH as u32,
                "Gust",
                glfw::WindowMode::Windowed,
            )
            .unwrap();

        gl::load_with(|s| win.get_proc_address(s) as *const _);

        let mut vertex_array = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vertex_array);
            crate::gl_utils::update_vao(vertex_array);
        }

        Window {
            view: View::from(Rect::new(
                0.0,
                0.0,
                DEFAULT_WIDTH as f32,
                DEFAULT_HEIGHT as f32,
            )),
            height: DEFAULT_HEIGHT,
            width: DEFAULT_WIDTH,
            win,
            event: Rc::new(evt),
            clear_color: Color::new(1.0, 1.0, 1.0),
            already_init: true,
            fps_limit: self::DEFAULT_FPS,
            vertex_array,
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
            InputMode::CursorMode(a) => (0x0003_3001, a as *const _ as i32),
            InputMode::StickKeys => (0x0003_3002, 1),
            InputMode::StickMouseButtons => (0x0003_3003, 1),
            InputMode::NotDefined => (-1, -1),
        }
    }
}

impl From<i32> for InputMode {
    fn from(value: i32) -> InputMode {
        match value {
            0x0003_3001 => InputMode::CursorMode(InputState::Normal),
            0x0003_3002 => InputMode::StickKeys,
            0x0003_3003 => InputMode::StickMouseButtons,
            _ => InputMode::NotDefined,
        }
    }
}

pub enum InputState {
    Normal = 0x0003_4001,
    Hidden = 0x0003_4002,
    Disable = 0x0003_4003,
}
