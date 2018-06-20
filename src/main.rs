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
mod vector;
mod texture;
mod event;

use gl::types::*;
use glfw::{Action, Context, Key};
use window::Window;
use std::cell::RefCell;
use std::rc::Rc;
use color::Color;

static HEIGHT: usize = 800;
static WIDTH: usize = 600;

fn main()
{
    let mut window = Window::new(HEIGHT, WIDTH, "Hello");
    window.set_clear_color(Color::new(1.0, 0.5, 0.5));
    window.set_key_polling(true);

    while window.is_open() {
        window.poll_events();

        for (_, event) in glfw::flush_messages(&*window.event.clone()) {
            event_handling(&mut window, event);
        }

        window.clear();
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