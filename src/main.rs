/*
** Alexandre Fourcat 2018
** main
** Description:
**
*/

extern crate gl;
extern crate glfw;
extern crate nalgebra;

mod object;
mod color;
mod window;
mod drawable;
mod texture;
mod event;
mod shader;

use gl::types::*;
use glfw::{Action, Context, Key};
use window::Window;
use std::cell::RefCell;
use std::rc::Rc;
use color::Color;
use object::VertexBuffer;
use texture::{Texture};

static HEIGHT: usize = 800;
static WIDTH: usize = 600;

fn main()
{
    let mut window = Window::new(HEIGHT, WIDTH, "Hello");
    let vbo = VertexBuffer::new(&window::TEST);
    let tex = Texture::new("texture/Z.png");

    window.set_clear_color(Color::new(0.6, 0.0, 1.0));
    window.set_key_polling(true);
    while window.is_open() {
        window.poll_events();

        for (_, event) in glfw::flush_messages(&*window.event.clone()) {
            event_handling(&mut window, event);
        }

        window.clear();
        window.draw(&vbo);
        window.display();
    }
}

fn event_handling(window: &mut Window, event: glfw::WindowEvent) {

    if let Some(key) = event::pressed(&event) {
        match key {
            Key::Escape => {
                window.close();
            },
            _ => {}
        };
    }
}