extern crate gust;

use gust::sprite::Sprite;
use gust::Key;
use gust::window::Window;
use gust::event::{EventReceiver};
use gust::{Vector,Point};
use gust::event;
use gust::event::{Event};
use std::rc::Rc;
use gust::color::Color;
use gust::texture::{Texture};
use gust::draw::{Drawer,Movable};
use gust::draw::Drawable;
use gust::view::{View};
use gust::rect::{Rect};

fn main()
{
    let mut window = Window::new(1600, 800, "Hello");
    let tex_dirt = Rc::new(Texture::new("examples/texture/Dirt.png"));
    let event_receiver = EventReceiver::from(&window);
    let mut sprite = Sprite::from(&tex_dirt);
    sprite.set_position(Vector::new(100.0, 100.0));

    window.set_key_polling(true);
    while window.is_open() {
        window.poll_events();
        sprite.update();
        window.get_view_mut().update();

        for event in event_receiver.fetch() {
            event_handling(&mut window, event);
        }
        window.clear();
        window.draw(&sprite);
        window.display();
    }
}

fn event_handling(window: &mut Window, event: Event) {

    if let Some(key) = event::pressed(event) {
        match key {
            Key::Escape => {
                window.close();
            },
            Key::A  => {
                println!("A !");
                window.get_view_mut().translate(Vector::new(-10.0, 0.0));
            },
            Key::W => {
                window.get_view_mut().translate(Vector::new(0.0, -10.0));
            },
            Key::S => {
                window.get_view_mut().translate(Vector::new(0.0, 10.0));
            },
            Key::D => {
                window.get_view_mut().translate(Vector::new(10.0, 0.0));
            },
            Key::E => {
                println!("TEST");
            },
            _ => {}
        };
    }
}

