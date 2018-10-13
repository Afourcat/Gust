extern crate gust;
extern crate glfw;

use gust::sprite::Sprite;
use gust::window::Window;
use gust::{Vector,Point,Action,Key};
use gust::event;
use gust::event::{EventHandler,Events,Event};
use std::rc::Rc;
use gust::color::Color;
use gust::texture::{Texture};
use gust::event::EventType;
use gust::draw::{Drawer,Movable};
use gust::draw::Drawable;
use gust::text::Text;
use gust::font::Font;
use std::sync::Mutex;
use std::cell::RefCell;

fn main() {
    // Create drawer window
    let mut window = Window::new(gust::WIDTH, gust::HEIGHT, "Hello");

    // Create event handler
    let event_handler = EventHandler::new(&window);

    // Create font
    let font = Rc::new(RefCell::new(Font::from_path("examples/font/terminus.ttf").unwrap()));

    // Create text with font
    let mut text = Text::new(&font);
    text.set_content(String::from("Herbosan salut, tu vas bien ? Je trouve que tu suce bien <3"));
    text.set_position(Vector::new(500.0, 500.0));
    // Loop preparation
    window.set_clear_color(Color::new(0.0, 0.0, 1.0));
    window.enable_cursor();
    window.poll(EventType::Key);
    while window.is_open() {
        // update text
        text.update();
        window.poll_events();

        // Event handling
        event_handler.fetch().for_each(|event| handle(&event, &mut window));
     
        // Draw process (Clear -> Draw -> Display)
        window.clear();
        window.draw(&text);
        window.display();
    }
}

fn handle(event: &Event, window: &mut Window) {
    match event.1 {
        Events::Key(Key::Escape, _, Action::Press, _) => window.close(),
        _ => {}
    }
}
