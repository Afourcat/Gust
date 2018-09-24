//
//  Rust file | 2018
//  Author: Alexandre Fourcat
//  font.rs
//  module:
//! font datastructures and fonctions

use self::ft::{
    library::Library,
    face::{
        Face,
        LoadFlag
    },
    bitmap::Bitmap,
};
use rect::Rect;
use texture::Texture;
use std::collections::HashMap;

extern crate freetype as ft;

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

fn get_texture_rect(bitmap: &Bitmap, font_texture: &Texture) {
    
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
            advance: 0.0
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
            self.face.load_char(size as usize, LoadFlag::DEFAULT).unwrap();

            let mut to_insert = CharInfo::new();

            let bitmap = self.face.glyph().bitmap();
            to_insert.rect.width = (bitmap.width() + 2) as f32;
            to_insert.rect.height = (bitmap.rows() + 2) as f32;

            get_texture_rect(&bitmap, &glyph_map.texture);

            glyph_map.map.insert(
                code,
                to_insert
            );
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

