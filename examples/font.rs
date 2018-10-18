extern crate gust;
extern crate glfw;

use gust::window::Window;
use gust::{Vector,Action,Key};
use gust::event::{EventHandler,Events,Event};
use std::rc::Rc;
use gust::color::Color;
use gust::draw::{Drawer,Movable};
use gust::draw::Drawable;
use gust::text::Text;
use gust::font::Font;
use std::cell::RefCell;

fn main() {
    // Create drawer window
    let mut window = Window::new(gust::WIDTH, gust::HEIGHT, "Hello");

    // Create event handler
    let event_handler = EventHandler::new(&window);

    // Create font
    let font = Rc::new(RefCell::new(Font::from_path("examples/font/test.ttf").unwrap()));

    // Create text with font
    let mut text = Text::new(&font);
    text.set_content("Welcome to Gust you can write text and\na lot more !\t(Like tabs)");
    text.set_position(Vector::new(100.0, 100.0));

    // Create a 2nd text with font
    let mut text2 = Text::from_str(&font, "Salut !");
    text2.set_position(Vector::new(200.0, 200.0));
    text2.update();

    // Loop preparation
    window.set_clear_color(Color::new(0.0, 0.0, 0.0));
    window.enable_cursor();
    window.poll(None);
    while window.is_open() {
        // update text
        text.update();

        // Poll event
        window.poll_events();
        event_handler.fetch().for_each(|event| handle(&event, &mut window, &mut text));

        // Draw process (Clear -> Draw -> Display)
        window.clear();
        window.draw(&text);
        window.draw(&text2);
        window.display();
    }
}

fn handle(event: &Event, window: &mut Window, text: &mut Text) {
    match event.1 {
        Events::Key(Key::Escape, _, Action::Press, _) => window.close(),
        Events::CursorPos(x, y) => {
            text.set_position(Vector::new(x as f32, y as f32));
        },
        _ => {}
    }
}
