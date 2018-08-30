extern crate gust;

use gust::sprite::Sprite;
use gust::glfw::{Key};
use gust::glfw;
use gust::window::Window;
use gust::event::{EventReceiver};
use gust::{Vector,Point,Coord};
use gust::event;
use gust::event::{Event};
use std::rc::Rc;
use gust::color::Color;
use gust::texture::{Texture};
use gust::draw::{Drawer,Movable};
use gust::draw::Drawable;

fn main()
{
    let mut window = Window::new(gust::WIDTH, gust::HEIGHT, "Hello");
    let tex_leave = Rc::new(Texture::new("examples/texture/Z.png"));
    let tex_dirt = Rc::new(Texture::new("examples/texture/Dirt.png"));
    let sprite = Sprite::from(&tex_dirt);
    let event_receiver = EventReceiver::from(&window);
    let mut leave = Sprite::from(&tex_leave);

    leave.set_position(Point::new(300.0, 300.0));
    window.set_clear_color(Color::new(0.0, 0.0, 1.0));
    window.set_key_polling(true);
    leave.set_scale(Vector::new(0.5, 0.5));
    if let Err(e) = leave.set_origin_to_center() { println!("{}", e) } else { println!("Ok") }
    while window.is_open() {
        window.poll_events();
        leave.rotate(0.2);
        leave.update();
        //leave.rotate(0.2);

        //event_receiver.fetch().for_each(|(_, input)| event_handling(&mut window, input));
        for event in event_receiver.fetch() {
            event_handling(&mut window, event);
        }

        window.clear();
        window.draw(&sprite);
        window.draw(&leave);
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
