extern crate gust;

use gust::window::Window;
use gust::{Vector,Key,Action};
use gust::event::{EventHandler,Events,EventType};
use gust::color::Color;
use gust::draw::Drawer;
use gust::vertex_buffer::{VertexBuffer,Primitive};
use gust::vertex::{VertexArray,Vertex};

fn main()
{
    let mut window = Window::new(gust::WIDTH, gust::HEIGHT, "Hello");
    let vert_arr = VertexArray::new(vec![
            Vertex::new(Vector::new(800.0, 400.0), Vector::new(0.0, 0.0), Color::new(0.0, 1.0, 0.5)),
            Vertex::new(Vector::new(1200.0, 700.0), Vector::new(0.0, 0.0), Color::new(0.0, 1.0, 0.5)),
            Vertex::new(Vector::new(1000.0, 300.0), Vector::new(0.0, 0.0), Color::new(0.0, 1.0, 0.5)),
            Vertex::new(Vector::new(800.0, 100.0), Vector::new(0.0, 0.0), Color::new(0.0, 1.0, 0.5)),
            Vertex::new(Vector::new(600.0, 300.0), Vector::new(0.0, 0.0), Color::new(0.0, 1.0, 0.5)),
            Vertex::new(Vector::new(400.0, 700.0), Vector::new(0.0, 0.0), Color::new(0.0, 1.0, 0.5)),
    ]);
    let vert_buf = VertexBuffer::new(Primitive::TriangleFan, vert_arr);
    let event_handler = EventHandler::new(&window);

    window.set_clear_color(Color::red());
    window.poll(EventType::Key);
    while window.is_open() {
        window.poll_events();

        for event in event_handler.fetch() {
            match event.1 {
                Events::Key(Key::Escape, _, Action::Press, _) => { window.close(); },
                _ => {}
            }
        }

        window.clear();
        window.draw(&vert_buf);
        window.display();
    }
}
