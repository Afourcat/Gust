use crate::color::Color;
use crate::draw::Context as gustContext;
use crate::draw::*;
use crate::vertex::{Vertex, VertexArray};
use crate::vertex_buffer::{Primitive, VertexBuffer};
use crate::view::View;
use crate::window::Window;
use crate::Vector;
use glfw::Context;
use nalgebra::Matrix4;

/// Shared Window is a window that can be shared between thread.
pub struct SharedWindow {
    context: glfw::RenderContext,
    view: View,
}

impl SharedWindow {
    pub fn new(window: &mut Window) -> SharedWindow {
        SharedWindow {
            view: window.view().clone(),
            context: window.win.render_context(),
        }
    }

    pub fn active(&mut self) -> bool {
        self.context.make_current();
        self.context.is_current()
    }

    pub fn display(&mut self) {
        self.active();
        self.context.swap_buffers();
    }

    pub fn clear(&self, color: Color) {
        unsafe {
            gl::ClearColor(color.0, color.1, color.2, color.3);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

impl Drawer for SharedWindow {
    fn draw<T: Drawable>(&mut self, drawable: &T) {
        self.active();
        drawable.draw(self);
    }

    fn draw_mut<T: DrawableMut>(&mut self, drawable: &mut T) {
        self.active();
        drawable.draw_mut(self);
    }

    fn draw_with_context<T: Drawable>(&mut self, drawable: &T, context: &mut gustContext) {
        self.active();
        drawable.draw_with_context(self, context);
    }

    fn draw_with_context_mut<T: DrawableMut>(
        &mut self,
        drawable: &mut T,
        context: &mut gustContext,
    ) {
        self.active();
        drawable.draw_with_context_mut(self, context);
    }

    fn draw_vertices(&self, vertices: &[Vertex], primitive: Primitive, context: &mut gustContext) {
        unimplemented!("Draw vertices");
    }

    fn draw_vertex_array(&self, vertices: &VertexArray, context: &mut gustContext) {
        unimplemented!("Vertex Array");
    }

    fn draw_vertex_buffer(&self, vertex_buffer: &VertexBuffer, context: &mut gustContext) {
        unimplemented!("VertexBuffer");
    }

    unsafe fn draw_from_raw(
        &self,
        raw: *const std::ffi::c_void,
        len: usize,
        context: &mut gustContext,
    ) {
        unimplemented!("Raw");
    }

    fn center(&self) -> Vector<f32> {
        Self::center(self)
    }

    fn sizes(&self) -> Vector<f32> {
        Self::sizes(self)
    }

    fn projection(&self) -> Matrix4<f32> {
        Self::projection(self)
    }
}
