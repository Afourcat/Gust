//! # Gust
//! Gust is a **graphical library** written in Rust by myself (Alexandre Fourcat)
//! Nothing really big or incredible but I inspired myself of SFML and kiss3D
//! The idea behind it it's to make the 2019 GGJ with this motor
//! This project is based on learning purpose and abstract everything that put me in pain
//! when I was trying to do computer graphics.
//! Here is some gust code
//! ```no_run
//! extern crate gust;
//! extern crate glfw;
//! 
//! use gust::sprite::Sprite;
//! use gust::window::Window;
//! use gust::{Vector,Point,Key};
//! use gust::event;
//! use gust::event::{EventHandler,Events,Event};
//! use std::rc::Rc;
//! use gust::color::Color;
//! use gust::texture::{Texture};
//! use gust::draw::{Drawer,Movable};
//! use gust::draw::Drawable;
//! 
//! fn main()
//! {
//!     let mut window = Window::new(gust::WIDTH, gust::HEIGHT, "Hello");
//!     let tex_leave = Rc::new(Texture::new("examples/texture/Z.png"));
//!     let tex_dirt = Rc::new(Texture::new("examples/texture/Dirt.png"));
//!     let event_handler = EventHandler::new(&window);
//!     let mut sprite = Sprite::from(&tex_dirt);
//!     let mut leave = Sprite::from(&tex_leave);
//! 
//!     leave.set_position(Point::new(300.0, 300.0));
//!     window.set_clear_color(Color::new(0.0, 0.0, 1.0));
//!     window.enable_cursor();
//!     window.poll(None);
//!     leave.set_scale(Vector::new(0.5, 0.5));
//!     leave.set_origin_to_center().unwrap_or_else(|e| println!("{}", e));
//!     while window.is_open() {
//!         window.poll_events();
//!         leave.rotate(1.0);
//!         leave.update();
//!         sprite.update();
//! 
//!         for event in event_handler.fetch() {
//!             event_process(event, &mut window);
//!         }
//! 
//!         window.clear();
//!         window.draw(&sprite);
//!         window.draw(&leave);
//!         window.display();
//!     }
//! }
//! 
//! fn event_process(event: Event, window: &mut Window) {
//!     match event.1 {
//!         Events::Key(Key::Escape, _, _, _) => {
//!             window.close();
//!         },
//!         Events::MouseButton(_, _, _) => {
//!             println!("Mouse button !");
//!         },
//!         Events::CursorPos(x, y) => {
//!             println!("x: {}, y: {}", x, y);
//!         },
//!         _ => { println!("Another event !") }
//!     }
//! }
//! ```

#![allow(dead_code)]
#![feature(test)]

extern crate gl;
extern crate glfw;
extern crate nalgebra;
extern crate freetype;
#[macro_use]
extern crate lazy_static;
extern crate image;

pub mod resources;
pub mod gl_error;
pub mod vertex_buffer;
pub mod color;
pub mod window;
pub mod draw;
pub mod texture;
pub mod event;
pub mod shader;
pub mod sprite;
pub mod vertex;
pub mod view;
pub mod rect;
pub mod text;
pub mod font;

pub use glfw::Key;
pub use glfw::MouseButton;
pub use glfw::MouseButtonLeft;
pub use glfw::MouseButtonRight;
pub use glfw::MouseButtonMiddle;
pub use glfw::Action;
pub use resources::{Resource, MutResource, ThreadResource, MutThreadResource};

pub type Vector<T> = nalgebra::Vector2<T>;
pub type Point<T>  = Vector<T>;
pub type Coord  = nalgebra::Vector2<usize>;

pub static HEIGHT: usize = 900;
pub static WIDTH: usize = 1600;
