extern crate gust;

use gust::sprite::Sprite;
use gust::glfw::{Key};
use gust::glfw;
use gust::window::Window;
use gust::event::{EventReceiver};
use gust::event;
use std::rc::Rc;
use gust::color::Color;
use gust::object::{VertexBuffer, Primitive};
use gust::texture::{Texture};
use gust::draw::{Drawable,Drawer,Movable};
use gust::{Vector,Point,Coord};

static VERTICE: [f32; 32] = [
    -0.5, -0.5, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0,
    0.5, -0.5, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0,
    -0.5,  0.5, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    0.5,  0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
];

fn main()
{
    let mut window = Window::new(gust::WIDTH, gust::HEIGHT, "Hello");
    let tex_leave = Rc::new(Texture::new("texture/test.jpg"));
    let tex_dirt = Rc::new(Texture::new("texture/Dirt.png"));
    let mut sprite = Sprite::from(Rc::clone(&tex_dirt));
    let mut buffer = VertexBuffer::new(Primitive::TrianglesStrip, &VERTICE);

    buffer.assign_texture(tex_leave);
    sprite.set_position(Vector::new(-250 as f32, -250 as f32));
    let event_receiver = EventReceiver::from(&window);

    window.set_clear_color(Color::new(1.0, 0.2, 0.7));
    window.set_key_polling(true);
    while window.is_open() {
        window.poll_events();

        //event_receiver.fetch().for_each(|(_, input)| event_handling(&mut window, input));
        for (_, input) in event_receiver.fetch() {
            event_handling(&mut window, input);
        }


        window.clear();
        window.draw(&buffer);
        window.draw(&sprite);
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
            Key::K => {
                println!("Je taime!");
            },
            Key::E => {
                println!("TEST");
            },
            _ => {}
        };
    }
}

