//
//  Rust file | 2018
//  Author: Alexandre Fourcat
//  event.rs
//  module:
//! event example
extern crate gust;
extern crate glfw;

use gust::sprite::Sprite;
use gust::window::Window;
use gust::{Vector,Point,Key};
use glfw::Action;
use gust::event;
use gust::event::{EventHandler,Events,Event};
use std::rc::Rc;
use gust::color::Color;
use gust::texture::{Texture};
use gust::draw::{Drawer,Movable};
use gust::draw::Drawable;
use std::collections::HashMap;

fn main()
{
    let mut window = Window::new(gust::WIDTH, gust::HEIGHT, "Hello");
    let tex_dirt = Rc::new(Texture::new("examples/texture/Dirt.png"));
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
    for (name, sprite) in sprites.iter_mut() { sprite.update(); }

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
        Events::Key(Key::Space, _, Action::Press, _) => {
            sprites.get_mut("dirt_1").unwrap().rotate(45.0);
        },
        Events::Key(Key::W, _, _, _) => {
            sprites.get_mut("dirt_2").unwrap().translate(Vector::new(0.0, -10.0));
        },
        Events::Key(Key::A, _, _, _) => {
            sprites.get_mut("dirt_2").unwrap().translate(Vector::new(-10.0, 0.0));
        },
        Events::Key(Key::S, _, _, _) => {
            sprites.get_mut("dirt_2").unwrap().translate(Vector::new(0.0, 10.0));
        },
        Events::Key(Key::D, _, _, _) => {
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
