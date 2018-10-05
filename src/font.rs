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
use super::Vector;
use std::fmt;

extern crate freetype as ft;

static SIZE: u32 = 10;

type FontMap = HashMap<u32, GlyphMap>;
type Utf8Map = HashMap<u32, CharInfo>;

/// The texture inside is a wide texture representing a font for a size x
/// and map is the map of utf-8 value linked to his GraphicalChar
#[derive(Debug)]
struct GlyphMap {
    pub texture: Texture,
    pub rows: Vec<Row>,
    pub map: Utf8Map,
}

/// Give information about the texture used
#[derive(Debug,Clone)]
struct Row {
    pub width: u32,
    pub height: u32,
    pub pos: u32
}

impl Row {
    pub fn new(height: u32, pos: u32) -> Row {
        Row {
            width: 0,
            height: height,
            pos: pos
        }
    }
}

/// Contain the global texture and texture information
impl GlyphMap {
    pub fn new() -> GlyphMap {
        GlyphMap {
            texture: Texture::from_size(Vector::new(500, 300)),
            rows: Vec::with_capacity(1),
            map: Utf8Map::with_capacity(10)
        }
    }

    pub fn get_texture_rect
    (&mut self, width: u32, height: u32, _size: u32) -> Rect<u32> {
        let mut ret: Option<Rect<u32>> = None;
        // Iter over all element
        for mut row in self.rows.iter_mut() {
            if row.width + width  > self.texture.width() {
                println!("Cannot get texture cause width = {} and row.width +
                width = {}.", self.texture.width(), row.width + width);
                continue;
            } else if row.height < height {
                println!("The letter is to high for this row.");
                continue;
            }
            ret = Some(Rect::new(row.width, row.height, width, height));
            row.width += width + 1;
        }

        if let Some(rect) = ret {
            // Return the rect
            rect
        } else {
            // Create a new row

            // TODO See todo behind
            // Get the last y pos of the texture
            //for e in self.rows.iter() {
            //    last_pos += e.height;
            //}
            // TODO -> TEST
            let last_pos = self.rows.iter().map(|x| x.height).sum();
            if last_pos + height + 10 > self.texture.height() {
                let mut new = Texture::from_size(
                    Vector::new(self.texture.width() * 2, self.texture.height() * 2)
                );
                new.update_from_texture(&self.texture);
                self.texture = new;
            }
            let mut new_row = Row::new(height, last_pos);
            let new_ret = Rect::new(new_row.width, new_row.height, width, height);
            new_row.width += width + 1;
            self.rows.push(new_row);
            new_ret
        }
    }

    /// Create a new texture from Utf8Map
    pub fn update_texture(&mut self, _char_info: &CharInfo) {
        // Check if there is a place
    }
}

/// rect: it's size
/// texCoord: coord of the texture inside the parent texture
#[derive(Debug)]
pub struct CharInfo {
    rect: Rect<u32>,
    tex_coord: Rect<u32>,
    advance: u32
}

impl CharInfo {

    pub fn new() -> CharInfo {
        CharInfo {
            rect: Default::default(),
            tex_coord: Default::default(),
            advance: 0
        }
    }

    pub fn from_data
    (rect: Rect<u32>, tex: Rect<u32>, adv: u32) -> CharInfo {
        CharInfo {
            rect: rect,
            tex_coord: tex,
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

impl fmt::Display for Font {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Face: {:?}", self.face)
    }
}

impl fmt::Debug for Font {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Face: {:?}", self.face)
    }
}

impl Font {
    /// Create a new font from a filepath
    pub fn from_path(path: &str) -> Option<Font> {
        let lib = Library::init().unwrap();
        match lib.new_face(path, 0) {
            Err(err) => {
                println!("Font loading error: {}", err);
                None
            },
            Ok(face) => {
                face.set_char_size(40 * 64, 0, 50, 0).unwrap();
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
            if let Some(ref mut _char_info) = map_size.map.get(&code) {
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
            // Get the glyph map
            let glyph_map = self.map.entry(size).or_insert(GlyphMap::new());

            // Load the right glyph
            self.face.load_char(code as usize, LoadFlag::DEFAULT).unwrap();

            let metrics = self.face.glyph().metrics();
            let bitmap = self.face.glyph().bitmap();
            // Create the new Charinfo that will be inserted
            let mut to_insert = CharInfo::new();

            // Get the glyph and informations
            to_insert.rect.width = (metrics.width + 2) as u32;
            to_insert.rect.height = (metrics.height + 2) as u32;
            to_insert.advance = (metrics.vertAdvance + 2) as u32;

            // Look at the glyph texture and try to find a place inside it
            to_insert.tex_coord = glyph_map.get_texture_rect(
                size, to_insert.rect.width, to_insert.rect.height);

            // Insert the new glyph map into the hasmap
            glyph_map.map.insert(
                code,
                to_insert
            );
        }

        // Return the newly inserted charinfo
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
