//
//  Rust file | 2018
//  Author: Alexandre Fourcat
//  text.rs
//  module:
//! text render utils
//! # How it works
//! When you create a font you create a face from freetype
//! This face is stocked inside the Font struct.
//! When you trigger glyph from Text component. The font look if the glyph from
//! this size and this letter exist. Else it loadIt from the Face and Add it to
//! a globaltexture linked to the font size.
//! This way it load only a rect from the same texture when you draw text.
//! # Example
//! ```no_run
//! use text::{Font,Text};
//! let font = Rc::new(Font::new("examples/fonts/Monaco.ttf"));
//! let text = Text::new(&font);
//! text.set_content("This is the text content");
//! ```
//! Text is drawable so you can use targer.draw(text);
//! after initialising it.
//!
use rect::Rect;
use texture::Texture;
use draw::{Drawable,Drawer,Context,Movable};
use ::{Point,Vector};
use nalgebra;
use std::{
    rc::Rc,
    path::Path,
    collections::HashMap,
};

extern crate freetype as ft;


use self::ft::{
    library::Library,
    face::{
        Face,
        LoadFlag
    },
    glyph::Glyph,
    FtResult
};

static SIZE: u32 = 10;

type FontMap = HashMap<u32, GlyphMap>;
type Utf8Map = HashMap<u32, CharInfo>;

/// The texture inside is a wide texture representing a font for a size x
/// and map is the map of utf-8 value linked to his GraphicalChar
struct GlyphMap {
    pub texture: Texture,
    pub map: Utf8Map,
}

impl GlyphMap {
    pub fn new() -> GlyphMap {
        GlyphMap {
            texture: Texture::empty(),
            map: Utf8Map::with_capacity(10)
        }
    }

    /// Create a new texture from Utf8Map
    pub fn update(&mut self) {
        
    }
}

/// rect: it's size
/// texCoord: coord of the texture inside the parent texture
pub struct CharInfo {
    rect: Rect<f32>,
    texCoord: Rect<f32>,
    advance: f32
}

impl CharInfo {

    pub fn new() -> CharInfo {
        CharInfo {
            rect: Default::default(),
            texCoord: Default::default(),
            advance: 0_f32
        }
    }

    pub fn from_data(rect: Rect<f32>, tex: Rect<f32>, adv: f32) -> CharInfo {
        CharInfo {
            rect: rect,
            texCoord: tex,
            advance: adv
        }
    }
}

/// Contain a face and everything needed to render a text
pub struct Font {
    face: Face,
    lib: Library,
    outline: i32,
    map: FontMap
}

impl Font {
    /// Create a new font from a filepath
    pub fn new(path: &str) -> Option<Font> {
        let lib = Library::init().unwrap();
        match lib.new_face(path, 0) {
            Err(err) => {
                println!("Font loading error: {}", err);
                None
            },
            Ok(face) => {
                Some(Font {
                    face: face,
                    lib: lib,
                    outline: 0,
                    map: FontMap::with_capacity(1)
                })
            }
        }
    }

    fn glyph_exist(&mut self, size: u32, code: u32) -> bool {
        if let Some(ref mut map_size) = self.map.get(&size) {
            if let Some(ref mut char_info) = map_size.map.get(&code) {
                return true;
            }
        }
        return false;
    }

    fn get_map_mut(&mut self) -> &mut FontMap {
        &mut self.map
    }

    fn create_glyph<'a>(&'a mut self, size: u32, code: u32) -> &'a CharInfo {
        {
            let glyph_map = self.map.entry(size).or_insert(GlyphMap::new());
            glyph_map.map.insert(code, CharInfo::new());
            glyph_map.update();
        }
        self.get_map_mut()[&size].map.get(&code).unwrap()
    }

    /// Check if the glyph exist:
    /// If the glyph exist get GraphicChar from it
    /// Else add it to the row and update the texture
    pub fn glyph<'a>(&'a mut self, size: u32, code: u32) -> &'a CharInfo {
        let glyph_exist: bool = self.glyph_exist(size, code);

        if glyph_exist {
            self.map.get(&size).unwrap().map.get(&code).unwrap()
        } else {
            self.create_glyph(size, code)
        }
    }
}

pub struct Text {
    font: Rc<Font>,
    content: String,
    size: u32
}

impl Text {
    pub fn new(font: &Rc<Font>) -> Text {
        Text {
            font: Rc::clone(font),
            content: String::new(),
            size: self::SIZE
        }
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn set_size(&mut self, size: u32) {
        self.size = size;
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}

impl Movable for Text {
    fn contain<T: nalgebra::Scalar + From<f32> + Into<f32>>(&self, point: Point<T>) -> bool {
        true
    }

    fn translate<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, offset: Vector<T>) {

    }

    fn set_position<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, pos: Vector<T>) {

    }

    fn get_position(&self) -> Vector<f32> {
        Vector::new(0.0, 0.0)
    }

    fn scale<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, factor: Vector<T>) {

    }

    fn set_scale<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, vec: Vector<T>) {

    }

    fn get_scale(&self) -> Vector<f32> {
        Vector::new(0.0, 0.0)
    }

    fn rotate<T: nalgebra::Scalar + Into<f32>>(&mut self, angle: T) { 

    }

    fn set_rotation<T: nalgebra::Scalar + Into<f32>>(&mut self, angle: T) {

    }

    fn get_rotation(&self) -> f32 {
        0.0
    }

    fn set_origin<T: nalgebra::Scalar + Into<f32>>(&mut self, origin: Vector<T>) {

    }

    fn get_origin(&self) -> Vector<f32> {
        Vector::new(0.0, 0.0)
    }
}

impl Drawable for Text {
    fn update(&mut self) {

    }

    fn draw<T: Drawer>(&self, target: &mut T) {
    }

    fn draw_with_context<T: Drawer>(&self, target: &mut T, context: &mut Context) {
        unimplemented!();
    }

    fn set_texture(&mut self, texture: &Rc<Texture>) {
        unimplemented!();
    }
}
