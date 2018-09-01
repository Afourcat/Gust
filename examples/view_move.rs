extern crate gust;

use gust::sprite::Sprite;
use gust::glfw::{Key};
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
use gust::view::{View,Rect};

fn main()
{
    let mut window = Window::new(1600, 800, "Hello");
    let tex_dirt = Rc::new(Texture::new("examples/texture/Dirt.png"));
    let event_receiver = EventReceiver::from(&window);

    window.set_key_polling(true);
    while window.is_open() {
        window.poll_events();
        //leave.rotate(0.2);

        //event_receiver.fetch().for_each(|(_, input)| event_handling(&mut window, input));
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
                println!("Hello A !");
            },
            Key::E => {
                println!("TEST");
            },
            _ => {}
        };
    }
}

