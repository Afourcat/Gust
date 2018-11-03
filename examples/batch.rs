//
//  Rust file | 2018
//  Author: Alexandre Fourcat
//  batch.rs
//  module:
//! batch


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
use gust::spritebatch::SpriteBatch;

fn main() -> Result<(), Box<Error>> {
    let mut window = Window::new(gust::WIDTH, gust::HEIGHT, "Hello");

    let texture = Rc::new(Texture::from_path("examples/texture/Z.png").unwrap());
    let mut batch = SpriteBatch::from(&texture);

    let event_handler = EventHandler::new(&window);

    window.set_clear_color(Color::new(0.45, 0.0, 1.0));
    window.enable_cursor();
    window.poll(None);
    while window.is_open() {
        window.poll_events();
        batch.update();

        for event in event_handler.fetch() {
            event_process(event, &mut window);
        }

        window.clear();
        window.draw(&batch);
        window.display();
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
