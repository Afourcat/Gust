extern crate gust;

use gust::color::Color;
use gust::draw::Drawer;
use gust::event::{EventHandler, EventType, Events};
use gust::vertex::{Vertex, VertexArray};
use gust::vertex_buffer::{Primitive, VertexBuffer};
use gust::window::Window;
use gust::{Action, Key, Vector};

fn main() {
    let mut window = Window::new(gust::WIDTH, gust::HEIGHT, "Hello");
    let vert_arr = VertexArray::from(
        vec![
            Vertex::new(
                Vector::new(800.0, 400.0),
                Vector::new(200.0, 0.0),
                Color::new(0.0, 1.0, 0.0),
            ),
            Vertex::new(
                Vector::new(1200.0, 700.0),
                Vector::new(20.0, 10.0),
                Color::new(0.0, 1.0, 1.0),
            ),
            Vertex::new(
                Vector::new(1000.0, 300.0),
                Vector::new(0.0, 0.0),
                Color::new(0.0, 0.2, 1.0),
            ),
            Vertex::new(
                Vector::new(800.0, 100.0),
                Vector::new(0.0, 0.0),
                Color::new(1.0, 1.0, 0.5),
            ),
            Vertex::new(
                Vector::new(600.0, 300.0),
                Vector::new(0.0, 0.0),
                Color::new(0.5, 0.2, 0.1),
            ),
            Vertex::new(
                Vector::new(400.0, 700.0),
                Vector::new(0.0, 0.0),
                Color::new(1.0, 0.0, 0.0),
            ),
        ]
        .as_slice(),
    );

    let vert_arr_2 = VertexArray::from(
        vec![
            Vertex::new(Vector::new(0.0, 0.0), Vector::new(0.0, 0.0), Color::blue()),
            Vertex::new(
                Vector::new(0.0, 100.0),
                Vector::new(0.0, 0.0),
                Color::blue(),
            ),
            Vertex::new(
                Vector::new(100.0, 100.0),
                Vector::new(0.0, 0.0),
                Color::blue(),
            ),
            Vertex::new(Vector::new(0.0, 0.0), Vector::new(0.0, 0.0), Color::green()),
            Vertex::new(
                Vector::new(100.0, 100.0),
                Vector::new(0.0, 0.0),
                Color::green(),
            ),
            Vertex::new(
                Vector::new(100.0, 0.0),
                Vector::new(0.0, 0.0),
                Color::green(),
            ),
        ]
        .as_slice(),
    );

    let vert_buf = VertexBuffer::new(Primitive::TriangleFan, vert_arr);
    let vert_buf2 = VertexBuffer::new(Primitive::Triangles, vert_arr_2);

    let event_handler = EventHandler::new(&window);

    window.set_clear_color(Color::red());
    window.poll(EventType::Key);
    while window.is_open() {
        window.poll_events();

        for event in event_handler.fetch() {
            match event.1 {
                Events::Key(Key::Escape, _, Action::Press, _) => {
                    window.close();
                }
                _ => {}
            }
        }

        window.clear();
        window.draw(&vert_buf);
        window.draw(&vert_buf2);
        window.display();
    }
}
