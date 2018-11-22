use std::sync::Arc;
use view::View;
use window::Window;
use nalgebra::Matrix4;
use draw::*;
use glfw::Context;
use color::Color;
use crate::Vector;

/// Shared Window is a window that can be shared between thread.
pub struct SharedWindow {
    context: glfw::RenderContext,
    view: View
}

impl SharedWindow {
    pub fn new(window: &mut Window) -> SharedWindow {
        SharedWindow {
            view: window.view().clone(),
            context: window.win.render_context()
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
            gl::ClearColor(
                color.0,
                color.1,
                color.2,
                color.3
            );
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

impl Drawer for SharedWindow {

    fn draw<T: Drawable>(&mut self, drawable: &T) {
        self.active();
        drawable.draw(self);
    }

    fn get_projection(&self) -> &Matrix4<f32> {
        &self.view.get_projection()
    }

    fn projection(&self) -> &Matrix4<f32> {
        &self.view.get_projection()
    }

    fn get_center(&self) -> Vector<f32> {
        unimplemented!("Think 'bout giving center of view.");
    }

    fn get_sizes(&self) -> Vector<f32> {
        unimplemented!("Think 'bout giving sizes of view.");
    }
}
