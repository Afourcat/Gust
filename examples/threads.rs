/// Threads test.
/// This is a test to draw with the glfw RenderContext shareable between thread.
#[macro_use]
extern crate gust;
extern crate glfw;

use gust::prelude::*;
use gust::shared_window::SharedWindow;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;

fn main() {
    let mut window = Window::new(gust::WIDTH, gust::HEIGHT, "Hello");
    let event_handler = EventHandler::new(&window);
    let mut shared = SharedWindow::new(&mut window);
    let (send, recv) = mpsc::channel();

    let renderer = thread::spawn(move || {
        render(&mut shared, recv);
    });

    window.poll(None);
    while window.is_open() {
        window.poll_events();

        for event in event_handler.fetch() {
            match event.1 {
                pressed!(Escape) => window.close(),
                _ => {}
            }
        }
    }
    send.send(()).ok().expect("Failed to send ()");

    renderer.join().unwrap();
}

fn render(shared: &mut SharedWindow, recv: mpsc::Receiver<()>) {
    shared.active();
    let texture_rc =
        Rc::new(Texture::from_path("examples/texture/Dirt.png").expect("Cannot open New.png"));
    let sprite = Sprite::from(&texture_rc);

    loop {
        if recv.try_recv() == Ok(()) {
            break;
        }
        shared.clear(Color::red());
        shared.draw(&sprite);
        shared.display();
    }
}
