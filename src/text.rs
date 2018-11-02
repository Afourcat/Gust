//
//  Rust file | 2018
//  Author: Alexandre Fourcat
//  text.rs
//  module:
//! text render utils
/// # How to use
/// ```no_run
/// use gust::text::Text;
/// use gust::font::Font;
/// use gust::window::Window;
///
/// fn draw(score: u32) {
///     let window = Default::default();
///     let arial = MutResource::new(Font::from_path("resources/font/arial.ttf"));
///     let score = Text::new(&arial);
///
///     score.set_content(format!("Score: {}", score));
///     score.set_position(Vector::new(10.0, 10.0));
///     while window.is_open() {
///         window.clear();
///         window.draw(&score);
///         window.display();
///     }
/// }
/// ```
/// It's made from the Text system of the C++ library SFML.

use texture::Texture;
use font::{Font,CharInfo};
use draw::{Drawable, Drawer, Context, Movable, BlendMode, IDENTITY};
use shader;
use ::{Point,Vector};
use nalgebra;
use std::{error::Error, rc::Rc};
use std::cell::RefCell;
use vertex_buffer::VertexBuffer;
use vertex::Vertex;
use color::Color;

extern crate freetype as ft;

#[derive(Debug)]
/// # Text struct
/// Text is a drawable entity that can be used to display text.
/// The text need a MutResource<Font> because the text mut the
/// internal texture of his font.
pub struct Text {
    font: Rc<RefCell<Font>>,
    content: String,
    actual_size: u32,
    vertex_buffer: VertexBuffer,
    need_update: bool,
    pos: Vector<f32>
}

impl Text {

    /// Dump the font texture to a file
    pub fn dump_texture(&mut self) -> Result<(), Box<Error>>{
        // Get the texture
        let font_ref = self.font.try_borrow().unwrap();
        let texture = font_ref.texture(self.actual_size).unwrap();

        texture.to_file("font_dump.png")?;
        Ok(())
    }

    /// Create a new text from a font previously created
	pub fn new(font: &Rc<RefCell<Font>>) -> Text {
        Text {
            font: Rc::clone(font),
            content: String::new(),
            actual_size: 14,
            vertex_buffer: VertexBuffer::default(),
            need_update: true,
            pos: Vector::new(0.0, 0.0)
        }
    }

    /// Create a text from it's content and a font
    pub fn from_str(font: &Rc<RefCell<Font>>, content: &str) -> Text {
        Text {
            font: Rc::clone(font),
            content: String::from(content),
            actual_size: 14,
            vertex_buffer: VertexBuffer::default(),
            need_update: true,
            pos: Vector::new(0.0, 0.0)
        }
    }

    /// Set the content of the text
    pub fn set_content(&mut self, content: &str) {
        self.content = String::from(content);
        self.need_update = true;
    }

    /// Get the content of the text as &String
    pub fn content(&self) -> &String {
        &self.content
    }

    /// Get the content of the text as &mut String
    pub fn content_mut(&mut self) -> &mut String {
        self.need_update = true;
        &mut self.content
    }

    /// Set the local font size
    pub fn set_size(&mut self, size: u32) {
        self.actual_size = size;
        self.need_update = true;
    }

    /// Get the local font size
    pub fn size(&self) -> u32 {
        self.actual_size
    }

    fn set_texture(&mut self, _texture: &Rc<Texture>) {
        unimplemented!();
    }
}

impl Movable for Text {

    fn contain<T>(&self, _point: Point<T>) -> bool
    where
        T: nalgebra::Scalar + From<f32> + Into<f32>
    {
        true
    }

    fn translate<T>(&mut self, offset: Vector<T>)
    where
        T: nalgebra::Scalar + From<f32> + Into<f32>
    {
        self.pos.x += offset.x.into();
        self.pos.y += offset.y.into();
        self.need_update = true;
    }

    fn set_position<T>(&mut self, pos: Vector<T>)
    where
        T: nalgebra::Scalar + From<f32> + Into<f32>
    {
        self.pos.x = pos.x.into();
        self.pos.y = pos.y.into();
        self.need_update = true;
    }

    fn get_position(&self) -> Vector<f32> {
        self.pos
    }

    fn scale<T>(&mut self, _factor: Vector<T>)
    where
        T: nalgebra::Scalar + From<f32> + Into<f32>
    {
        unimplemented!();
    }

    fn set_scale<T>(&mut self, _vec: Vector<T>)
    where
        T: nalgebra::Scalar + From<f32> + Into<f32>
    {
        unimplemented!();
    }

    fn get_scale(&self) -> Vector<f32> {
        unimplemented!();
    }

    fn rotate<T>(&mut self, _angle: T)
    where
        T: nalgebra::Scalar + Into<f32>
    {
        unimplemented!();
    }

    fn set_rotation<T>(&mut self, _angle: T)
    where
        T: nalgebra::Scalar + Into<f32>
    {
        unimplemented!();
    }

    fn get_rotation(&self) -> f32 {
        0.0
    }

    fn set_origin<T>(&mut self, _origin: Vector<T>)
    where
        T: nalgebra::Scalar + Into<f32>
    {
        unimplemented!();
    }

    fn get_origin(&self) -> Vector<f32> {
        Vector::new(0.0, 0.0)
    }

}

impl Drawable for Text {

    fn update(&mut self) {
        // Si l'update n'est pas necessaire
        if !self.need_update { return }

        // Relative position
        let mut pos = self.pos;
        let mut offset = 0.0;

        // Get reference to the font that is a reference counter
        let mut font_ref = self.font
                        .try_borrow_mut()
                        .unwrap();

        // Get the whitespace x size
        let whitespace;
        let height;
        {
            let space_glyph = font_ref.glyph(self.actual_size, 0x20_u32);
            whitespace = space_glyph.advance;
        }
        {
            let a_glyph = font_ref.glyph(self.actual_size, 0x41_u32);
            height = a_glyph.rect.height + a_glyph.rect.height / 5.0;
        }

        // Setup padding
        let padding = 0.0;

        // Clear the buffer of the data
        self.vertex_buffer.clear();

        // Iter of character of the content to create a geometry for each one of them
        for charr in self.content.as_str().chars() {

            // If the char is a special one
            match charr {
                '\n'    => {
                    pos.y += height;
                    offset = 0.0;
                    continue;
                },
                '\r'    => continue,
                '\t'    => {
                    offset += 4.0 * whitespace;
                    continue;
                },
                ' '     => {
                    offset += whitespace;
                    continue;
                },
                _       => {}
            };

            // Get the glyph from the the font
            let char_info = font_ref.glyph(self.actual_size, charr as u32);

            // get vertices from char_info
            let vertices = get_vertice_letter(&char_info, pos, padding, offset);

            // append vertice to vertex_buffer
            self.vertex_buffer.append(&vertices);

            // x position of the character
            offset += char_info.advance as f32;
        }
        // Update final buffer
        self.vertex_buffer.update();

        // Set to false the boolean that contral this function
        self.need_update = false;
    }

    fn draw_mut<T: Drawer>(&mut self, target: &mut T) {
        self.update();
        self.draw(target);
    }

    fn draw<T: Drawer>(&self, target: &mut T) {
        // If there is no text don't draw
        if self.content.is_empty() { return }

        // Get the texture
        let font_ref = self.font.try_borrow().unwrap();
        let texture = font_ref.texture(self.actual_size).unwrap();

        // Create a new context with the Texture of the font
        let mut context = Context::new(
            Some(texture),
            &*shader::DEFAULT_SHADER,
            vec![
                ("transform".to_string(), &*IDENTITY),
                ("projection".to_string(), target.projection())
            ],
            BlendMode::Alpha
        );

        // Draw the vertex_buffer with context
        self.vertex_buffer.draw_with_context(&mut context);
    }

    fn draw_with_context_mut(&mut self, context: &mut Context) {
        self.update();
        self.draw_with_context(context);
    }

    fn draw_with_context(&self, context: &mut Context) {
        self.vertex_buffer.draw_with_context(context);
    }
}

/// Get a vertice from a character information, padding and offset
fn get_vertice_letter(char_info: &CharInfo, pos: Vector<f32>, padding: f32, offset: f32) -> [Vertex; 6] {
    let x = pos.x + offset;
    let y = pos.y;

    // Set geometry for 1 character
    let left   = char_info.rect.left - padding;
    let top    = char_info.rect.top - padding;
    let right  = char_info.rect.left + char_info.rect.width + padding;
    let bottom = char_info.rect.top  + char_info.rect.height + padding;

    // Set texture coord for each character
    let u1 = ((char_info.tex_coord.left - padding as u32) as f32) / 128.0;
    let v1 = ((char_info.tex_coord.top - padding as u32) as f32) / 128.0;
    let u2 = ((char_info.tex_coord.left + char_info.tex_coord.width + padding as u32) as f32) / 128.0;
    let v2 = ((char_info.tex_coord.top  + char_info.tex_coord.height + padding as u32) as f32) / 128.0;

    [
        Vertex::new(Vector::new(x + left,   y + top),       Vector::new(u1, v1), Color::white()),
        Vertex::new(Vector::new(x + left,   y + bottom),    Vector::new(u1, v2), Color::white()),
        Vertex::new(Vector::new(x + right,  y + bottom),    Vector::new(u2, v2), Color::white()),
        Vertex::new(Vector::new(x + left,   y + top),       Vector::new(u1, v1), Color::white()),
        Vertex::new(Vector::new(x + right,  y + bottom),    Vector::new(u2, v2), Color::white()),
        Vertex::new(Vector::new(x + right,  y + top),       Vector::new(u2, v1), Color::white()),
    ]
}
