//
//  Rust file | 2018
//  Author: Alexandre Fourcat
//  event.rs
//  module:
//! event example
#[macro_use]
extern crate gust;
extern crate glfw;

use gust::prelude::*;
use std::rc::Rc;
use std::collections::HashMap;

fn main() {
    let mut window = Window::new(gust::WIDTH, gust::HEIGHT, "Hello");
    let tex_dirt = Rc::new(Texture::from_path("examples/texture/Dirt.png").unwrap());
    let event_handler = EventHandler::new(&window);
    let mut sprites = HashMap::new();
    sprites.insert("dirt_1", Sprite::from(&tex_dirt));
    sprites.insert("dirt_2", Sprite::from(&tex_dirt));

    window.set_clear_color(Color::new(0.0, 0.0, 1.0));
    window.enable_cursor();
    window.poll(None);
    while window.is_open() {
        window.poll_events();

        for event in event_handler.fetch() {
            event_process(event, &mut window, &mut sprites);
        }

        draw(&mut window, &mut sprites);
    }
}

fn draw(window: &mut Window, sprites: &mut HashMap<&'static str,Sprite>) {
    for (_name, sprite) in sprites.iter_mut() { sprite.update(); }

    window.clear();
    window.draw(&sprites["dirt_1"]);
    window.draw(&sprites["dirt_2"]);
    window.display();
}

fn event_process(event: Event, window: &mut Window, sprites: &mut HashMap<&'static str,Sprite>) {
    match event.1 {
        Events::Key(Key::Escape, _, _, _) => {
            window.close();
        },
        pressed!(Space) => {
            sprites.get_mut("dirt_1").unwrap().rotate(45.0);
        },
        pressed!(W) => {
            sprites.get_mut("dirt_2").unwrap().translate(Vector::new(0.0, -10.0));
        },
        pressed!(A) => {
            sprites.get_mut("dirt_2").unwrap().translate(Vector::new(-10.0, 0.0));
        },
        pressed!(S) => {
            sprites.get_mut("dirt_2").unwrap().translate(Vector::new(0.0, 10.0));
        },
        pressed!(D) => {
            sprites.get_mut("dirt_2").unwrap().translate(Vector::new(10.0, 0.0));
        },
        Events::MouseButton(glfw::MouseButtonLeft, Action::Press, _) => {
            let mouse_pos = window.mouse_pos();

            if let Some(sprite) = sprites.get_mut("dirt_1") {

                if sprite.contain(mouse_pos) {
                    sprite.set_color(&Color::blue());
                }
            }
        },
        _ => {}
    }
}
