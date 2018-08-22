extern crate gust;

use gust::sprite::Sprite;
use gust::glfw::{Key};
use gust::glfw;
use gust::window::Window;
use gust::event::{EventReceiver};
use gust::{Vector,Point,Coord};
use gust::event;
use std::rc::Rc;
use gust::color::Color;
use gust::texture::{Texture};
use gust::draw::{Drawer,Movable};
use gust::draw::Drawable;

fn main()
{
    let mut window = Window::new(gust::WIDTH, gust::HEIGHT, "Hello");
    let tex_leave = Rc::new(Texture::new("texture/test.jpg"));
    let tex_dirt = Rc::new(Texture::new("texture/Dirt.png"));
    let sprite = Sprite::from(&tex_dirt);
    let event_receiver = EventReceiver::from(&window);
    let mut leave = Sprite::from(&tex_leave);

    leave.set_position(Point::new(100.0, 100.0));
    window.set_clear_color(Color::new(0.0, 0.0, 1.0));
    window.set_key_polling(true);
    while window.is_open() {
        window.poll_events();
        leave.update();
        leave.translate(Vector::new(10.0, 10.0));

        //event_receiver.fetch().for_each(|(_, input)| event_handling(&mut window, input));
        for (_, input) in event_receiver.fetch() {
            event_handling(&mut window, input);
        }

        window.clear();
        window.draw(&sprite);
        window.draw(&leave);
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
            Key::E => {
                println!("TEST");
            },
            _ => {}
        };
    }
}

