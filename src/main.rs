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
mod vertex;

use sprite::Sprite;
use gl::types::*;
use glfw::{Action, Context, Key};
use window::Window;
use std::cell::RefCell;
use std::rc::Rc;
use color::Color;
use object::{VertexBuffer, Primitive};
use texture::{Texture};
use vertex::Vertex;
use drawable::Drawable;
use nalgebra::*;

static HEIGHT: usize = 800;
static WIDTH: usize = 600;

static RECT_VBO: [f32; 18] = [
    // first triangle
     0.5,  0.5, 0.0,  // top right
     0.5, -0.5, 0.0,  // bottom right
    -0.5,  0.5, 0.0,  // top left 
    // scond triangle
     0.5, -0.5, 0.0,  // bottom right
    -0.5, -0.5, 0.0,  // bottom left
    -0.5,  0.5, 0.0   // top left
];

fn main()
{
    let test_1: [Vertex; 6] = [
        Vertex::new(Vector2::new( 0.5, 0.5), Vector2::new(1.0, 1.0), Color::new(1.0, 0.0, 0.0)),
        Vertex::new(Vector2::new( 0.5,-0.5), Vector2::new(1.0, 0.0), Color::new(1.0, 0.0, 0.0)),
        Vertex::new(Vector2::new(-0.5,-0.5), Vector2::new(0.0, 0.0), Color::new(1.0, 0.0, 0.0)),
        Vertex::new(Vector2::new( 0.5, 0.5), Vector2::new(1.0, 1.0), Color::new(1.0, 0.0, 0.0)),
        Vertex::new(Vector2::new(-0.5,-0.5), Vector2::new(0.0, 0.0), Color::new(1.0, 0.0, 0.0)),
        Vertex::new(Vector2::new(-0.5, 0.5), Vector2::new(0.0, 1.0), Color::new(1.0, 0.0, 0.0)),
    ];

    let mut window = Window::new(HEIGHT, WIDTH, "Hello");
    let mut vertex_b = VertexBuffer::new_from_vertex_array(Primitive::Triangles, &test_1);
    let tex = Rc::new(Texture::new("texture/Z.png"));
    let tex_leave = Rc::new(Texture::new("texture/test.jpg"));

    vertex_b.assign_texture(&Rc::clone(&tex_leave));
    window.set_clear_color(Color::new(1.0, 0.2, 0.7));
    window.set_key_polling(true);
    while window.is_open() {
        window.poll_events();

        for (_, event) in glfw::flush_messages(&*window.event.clone()) {
            event_handling(&mut window, event);
        }

        window.clear();
        window.draw(&vertex_b);
//      window.draw(&rect);
        window.display();
    }
}

fn event_handling(window: &mut Window, event: glfw::WindowEvent) {

    if let Some(key) = event::pressed(&event) {
        match key {
            Key::Escape => {
                window.close();
            },
            Key::A  => {
              println!("Hello A !");  
            },
            _ => {}
        };
    }
}
