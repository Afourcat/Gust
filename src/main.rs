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
mod sprite;

use sprite::Sprite;
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

static VERTICES: [f32; 32] = [
    // positions          // colors           // texture coords
     0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0,   // top right
     0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0,   // bottom right
    -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0,   // bottom left
    -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0    // top left 
];

fn main()
{
    let mut window = Window::new(HEIGHT, WIDTH, "Hello");
    let vbo = VertexBuffer::new(&window::TEST);
    let mut rect = VertexBuffer::new(&VERTICES);
    let tex = Rc::new(Texture::new("texture/Z.png"));
    let sprite = Sprite::new_from_texture(Rc::clone(&tex));


    rect.set_texture(&Rc::clone(&tex));
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
