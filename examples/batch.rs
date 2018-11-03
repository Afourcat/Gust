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
use gust::spritebatch::{SpriteBatch, SpriteData};
use gust::Matrix4;
use std::sync::Arc;

fn main() -> Result<(), Box<Error>> {
    let mut window = Window::new(gust::WIDTH, gust::HEIGHT, "Hello");

    let texture = Arc::new(Texture::from_path("examples/texture/Dirt.png").unwrap());
    let mut batch = SpriteBatch::from(&texture);
    for i in 0..100000 {
        batch.add_sprites(SpriteData::new(Vector::new(i as f32 + 10.0, i as f32 + 10.0)))
    }

    let event_handler = EventHandler::new(&window);

    window.set_clear_color(Color::new(0.45, 0.0, 1.0));
    window.enable_cursor();
    window.poll(None);
    while window.is_open() {
        window.poll_events();

        for event in event_handler.fetch() {
            event_process(event, &mut window);
        }

        window.clear();
        window.draw_mut(&mut batch);
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
