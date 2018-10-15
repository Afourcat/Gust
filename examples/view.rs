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

fn main()
{
    let mut window = Window::new(gust::WIDTH, gust::HEIGHT, "Hello");
    let tex_leave = Rc::new(Texture::from_path("examples/texture/Z.png").unwrap());
    let tex_dirt = Rc::new(Texture::from_path("examples/texture/Dirt.png").unwrap());
    let event_handler = EventHandler::new(&window);
    let mut sprite = Sprite::from(&tex_dirt);
    let mut leave = Sprite::from(&tex_leave);

    leave.set_position(Point::new(300.0, 300.0));
    window.set_clear_color(Color::new(0.0, 0.0, 1.0));
    window.enable_cursor();
    window.poll(None);
    leave.set_scale(Vector::new(0.5, 0.5));
    leave.set_origin_to_center().unwrap_or_else(|e| println!("{}", e));
    while window.is_open() {
        window.poll_events();
        leave.rotate(1.0);
        leave.update();
        sprite.update();
        window.get_view_mut().update();

        for event in event_handler.fetch() {
            event_process(event, &mut window);
        }

        window.clear();
        window.draw(&sprite);
        window.draw(&leave);
        window.display();
    }
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
            let center = Vector::new(x as f32, y as f32);
            window.get_view_mut().set_center(center);
            window.set_mouse_pos(center)
        },
        _ => { println!("Another event !") }
    }
}
