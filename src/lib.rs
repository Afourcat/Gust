//! # Gust
//! Gust is a **graphical library** written in Rust by myself (Alexandre Fourcat)
//! Nothing really big or incredible but I inspired myself of SFML and kiss3D
//! The idea behind it it's to make the 2019 GGJ with this motor
//! This project is based on learning purpose and abstract everything that put me in pain
//! when I was trying to do computer graphics.
//! Here is some gust code
//! ```rust
//! use sprite::Sprite;
//! use gl::types::*;
//! use glfw::{Action, Context, Key};
//! use glfw;
//! use window::Window;
//! use event::{EventReceiver};
//! use event;
//! use std::cell::RefCell;
//! use std::rc::Rc;
//! use color::Color;
//! use object::{VertexBuffer, Primitive};
//! use texture::{Texture};
//! use vertex::Vertex;
//! use draw::Drawable;
//! use nalgebra::*;
//! use draw::Drawer;
//!
//! fn main() {
//!     let mut window = Window::new(1920, 1080, "Hello");
//!     let tex = Rc::new(Texture::new("texture/Z.png"));
//!     let tex_leave = Rc::new(Texture::new("texture/test.jpg"));
//!     let sprite = Sprite::from_texture(Rc::clone(&tex_leave));
//!     let event_receiver = EventReceiver::from(&window);
//!
//!     window.set_clear_color(Color::new(1.0, 0.2, 0.7));
//!     window.set_key_polling(true);
//!     while window.is_open() {
//!         window.poll_events();
//!
//!         for (_, input) in event_receiver.fetch() {
//!             event_handling(&mut window, input);
//!         }
//!
//!         window.clear();
//!         window.draw(&sprite);
//!         window.display();
//!     }
//! }
//!
//! fn event_handling(window: &mut Window, event: glfw::WindowEvent) {
//!
//!     if let Some(key) = event::pressed(&event) {
//!         match key {
//!             Key::Escape => {
//!                 window.close();
//!             },
//!             Key::A  => {
//!                 println!("Hello A !");
//!             },
//!             Key::K => {
//!                 println!("Je taime!");
//!             },
//!             Key::E => {
//!                 println!("TEST");
//!             },
//!             _ => {}
//!         };
//!     }
//! }
//! ```

#![allow(dead_code)]


extern crate gl;
pub extern crate glfw;
extern crate nalgebra;
#[macro_use]
extern crate lazy_static;

pub mod vertex_buffer;
pub mod color;
pub mod window;
pub mod draw;
pub mod texture;
pub mod event;
pub mod shader;
pub mod sprite;
pub mod vertex;

pub type Vector = nalgebra::Vector2<f32>;
pub type Point  = Vector;
pub type Coord  = Point;

pub static HEIGHT: usize = 900;
pub static WIDTH: usize = 1600;
