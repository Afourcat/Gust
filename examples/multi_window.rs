extern crate glfw;
extern crate gust;

use gust::prelude::*;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

fn window1() -> Result<(), Box<Error>> {
    let mut window = Window::new(600, 600, "Hello1");
    let tex_leave = Rc::new(Texture::from_path("examples/texture/Z.png").unwrap());
    let tex_dirt = Rc::new(Texture::from_path("examples/texture/Dirt.png").unwrap());
    let mut sprite = Sprite::from(&tex_dirt);
    let mut leave = Sprite::from(&tex_leave);
    leave.set_position(Point::new(500.0, 500.0));
    sprite.set_position(Point::new(10.0, 10.0));
    leave.set_scale(Vector::new(0.5, 0.5));
    leave.set_origin_to_center()?;

    let event_handler = EventHandler::new(&window);

    window.set_clear_color(Color::new(0.45, 0.0, 1.0));
    window.enable_cursor();
    window.poll(None);

    while window.is_open() {
        window.poll_events();
        leave.rotate(1.0);
        leave.update();
        sprite.update();

        for event in event_handler.fetch() {
            event_process(event, &mut window);
        }

        window.clear();
        window.draw(&mut sprite);
        window.draw(&mut leave);
        window.display();
    }
    Ok(())
}

fn window2() -> Result<(), Box<Error>> {
    let mut window = Window::new(500, 500, "Hello2");
    let font = Rc::new(RefCell::new(
        Font::from_path("examples/font/terminus.ttf").unwrap(),
    ));
    let mut text = Text::from_str(&font, "I've been looking forward to this.");
    text.set_position(Vector::new(5.0, 40.0));
    let event_handler = EventHandler::new(&window);

    window.set_clear_color(Color::new(1.0, 0.0, 1.0));
    window.enable_cursor();
    window.poll(None);

    while window.is_open() {
        text.update();
        window.poll_events();

        for event in event_handler.fetch() {
            event_process(event, &mut window);
        }

        window.clear();
        window.draw(&mut text);
        window.display();
    }
    Ok(())
}

fn main() {
    use std::thread;

    // launch first window
    let waiter = thread::spawn(|| {
        window1().unwrap();
    });

    // launch second one
    let waiter2 = thread::spawn(|| {
        window2().unwrap();
    });

    // Wait for the two other thread to end before ending main
    waiter.join().unwrap();
    waiter2.join().unwrap();
}

fn event_process(event: Event, window: &mut Window) {
    match event.1 {
        Events::Key(Key::Escape, _, _, _) => {
            window.close();
        }
        Events::MouseButton(_, _, _) => {
            println!("Mouse button !");
        }
        Events::CursorPos(x, y) => {
            println!("x: {}, y: {}", x, y);
        }
        _ => println!("Another event !"),
    }
}
