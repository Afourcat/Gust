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
use gust::text::Text;
use gust::font::Font;
use std::sync::Mutex;
use std::cell::RefCell;

fn main()
{
    let mut window = Window::new(gust::WIDTH, gust::HEIGHT, "Hello");
    let font = Rc::new(
        RefCell::new(
            Font::from_path("examples/font/test.ttf").unwrap()
        )
    );
    let mut text = Text::new(&font);
    text.set_content(String::from("Example fonts !!!"));
    println!("Text: {:?}", text);

    window.set_clear_color(Color::new(0.0, 0.0, 1.0));
    window.enable_cursor();
    let a = true;
    while window.is_open() {
        text.update();

        if a {
            let a = false;
            println!("Texture {:?}", text);
        }
        window.clear();
        window.draw(&text);
        window.display();
    }
}
