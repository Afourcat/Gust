extern crate gust;
extern crate glfw;

use gust::sprite::Sprite;
use gust::window::Window;
use gust::{Vector,Point,Key};
use gust::event;
use gust::event::{EventHandler,Events,Event};
use std::rc::Rc;
use gust::color::Color;
use gust::texture::{Texture};
use gust::draw::{Drawer,Movable};
use gust::draw::Drawable;
use glfw::{Action,Modifiers};

fn main()
{
    let mut window = Window::new(gust::WIDTH, gust::HEIGHT, "Hello");
    let tex_leave = Rc::new(Texture::new("examples/texture/Z.png"));
    let tex_dirt = Rc::new(Texture::new("examples/texture/Dirt.png"));
    let mut sprite = Sprite::from(&tex_dirt);
    let mut event_handler = EventHandler::new(&window);
    let mut leave = Sprite::from(&tex_leave);

    leave.set_position(Point::new(300.0, 300.0));
    window.set_clear_color(Color::new(0.0, 0.0, 1.0));
    window.set_key_polling(true);
    leave.set_scale(Vector::new(0.5, 0.5));
    if let Err(e) = leave.set_origin_to_center() { println!("{}", e) } else { println!("Ok") }
    while window.is_open() {
        window.poll_events();
        leave.rotate(1.0);
        leave.update();
        sprite.update();
        //leave.rotate(0.2);

        //event_receiver.fetch().for_each(|(_, input)| event_handling(&mut window, input));
        event_handler.register_callback(handle_key,
            Events::Key(Key::A, 1, Action::Press, Modifiers::Shift));
        event_handler.handle();

        window.clear();
        window.draw(&sprite);
        window.draw(&leave);
        window.display();
    }
}

fn handle_key(event: Event) -> Result<(),String> {
    match event::fetch(&event) {
        _ => { 
            println!("Test");
            Ok(())
        }
    }
}
