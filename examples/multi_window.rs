extern crate gust;
extern crate glfw;

use gust::sprite::Sprite;
use gust::window::Window;
use gust::{Vector,Point,Key};
use gust::event::{EventHandler,Events,Event};
use std::rc::Rc;
use gust::color::Color;
use gust::texture::{Texture};
use gust::draw::{Drawer,Movable};
use gust::draw::Drawable;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let mut window = Window::new(gust::WIDTH, gust::HEIGHT, "Hello");
    let mut window2 = Window::new(gust::WIDTH, gust::HEIGHT, "Hello");

    let tex_leave = Rc::new(Texture::from_path("examples/texture/Z.png").unwrap());
    let tex_dirt = Rc::new(Texture::from_path("examples/texture/Dirt.png").unwrap());
    let mut sprite = Sprite::from(&tex_dirt);
    let mut leave = Sprite::from(&tex_leave);
    leave.set_position(Point::new(500.0, 500.0));
    sprite.set_position(Point::new(10.0, 10.0));
    leave.set_scale(Vector::new(0.5, 0.5));
    leave.set_origin_to_center()?;

    let event_handler = EventHandler::new(&window);

    window.set_clear_color(Color::new(0.45, 0.0, 1.0));
    window.enable_cursor();
    window.poll(None);
    
    while window.is_open() {
        window.poll_events();
        leave.rotate(1.0);
        leave.update();
        sprite.update();

        for event in event_handler.fetch() {
            event_process(event, &mut window);
        }

        window.clear();
        window.draw(&sprite);
        window.draw(&leave);
        window.display();

        window2.clear();
        window2.draw(&sprite);
        window2.display();
    }

    Ok(())
}

fn event_process(event: Event, window: &mut Window) {
    match event.1 {
        Events::Key(Key::Escape, _, _, _) => {
            window.close();
        },
        Events::MouseButton(_, _, _) => {
            println!("Mouse button !");
        },
        Events::CursorPos(x, y) => {
            println!("x: {}, y: {}", x, y);
        },
        _ => { println!("Another event !") }
    }
}
