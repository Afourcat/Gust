//
//  Rust file | 2018
//  Author: Alexandre Fourcat
//  batch.rs
//  module:
//! batch


extern crate gust;
extern crate glfw;

use gust::window::Window;
use gust::{Vector, Key, Action};
use gust::event::{Events, Event, EventHandler};
use std::rc::Rc;
use gust::texture::{Texture};
use gust::draw::{Movable, Drawer};
use std::error::Error;
use gust::spritebatch::{SpriteBatch, SpriteData};
use gust::color::Color;

fn main() -> Result<(), Box<Error>> {
    let mut window = Window::new(gust::WIDTH, gust::HEIGHT, "Hello");

    let texture = Rc::new(Texture::from_path("examples/texture/Dirt.png").unwrap());
    let mut batch = SpriteBatch::from(&texture);
    for i in 0..1_000_000 {
        let mut data = SpriteData::new(Vector::new(i as f32 * 1.0, i as f32 * 10.0));
        data.set_texture_raw([Vector::new(0.0, 0.0), Vector::new(1.0, 1.0)]);
        batch.push_sprite(data);
    }
    println!("Created !");

    let event_handler = EventHandler::new(&window);

    window.set_clear_color(Color::new(0.45, 0.0, 1.0));
    window.enable_cursor();
    window.poll(None);
    while window.is_open() {
        window.poll_events();

        for event in event_handler.fetch() {
            event_process(event, &mut window, &mut batch);
        }

        window.clear();
        window.draw_mut(&mut batch);
        window.display();
    }

    Ok(())
}

fn event_process(event: Event, window: &mut Window, batch: &mut SpriteBatch) {
    match event.1 {
        Events::Key(Key::Escape, _, _, _) => {
            window.close();
        },
        Events::Key(Key::W, _, Action::Press, _) => {
            batch.translate(Vector::new(10.0, 10.0));
        },
        Events::Key(Key::D, _, Action::Press, _) => {
            batch.get_sprite_mut(0).unwrap().translate(Vector::new(10.0, 0.0));
        },
        Events::MouseButton(_, _, _) => {
            println!("Mouse button !");
        },
        Events::CursorPos(x, y) => {
            println!("x: {}, y: {}", x, y);
        },
        _ => { println!("Another event !") }
    }
    drop(event);
}
