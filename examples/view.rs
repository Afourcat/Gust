extern crate glfw;
extern crate gust;

use gust::prelude::*;
use std::rc::Rc;

fn main() {
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
    leave
        .set_origin_to_center()
        .unwrap_or_else(|e| println!("{}", e));
    while window.is_open() {
        window.poll_events();
        leave.rotate(1.0);
        leave.update();
        sprite.update();
        window.view_mut().update();

        for event in event_handler.fetch() {
            event_process(event, &mut window);
        }

        window.clear();
        window.draw(&mut sprite);
        window.draw(&mut leave);
        window.display();
    }
}

fn event_process(event: Event, window: &mut Window) {
    match event.1 {
        Events::Key(Key::Escape, _, _, _) => {
            window.close();
        }
        Events::Key(Key::Up, _, _, _) => {
            window.view_mut().zoom(2.0);
        }
        Events::Key(Key::Down, _, _, _) => {
            window.view_mut().zoom(0.5);
        }
        Events::CursorPos(x, y) => {
            let center = Vector::new(x as f32, y as f32);
            window.view_mut().set_center(center);
            window.set_mouse_pos(center)
        }
        _ => println!("Another event !"),
    }
}
