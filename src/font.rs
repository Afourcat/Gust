//
//  Rust file | 2018
//  Author: Alexandre Fourcat
//  font.rs
//  module:
//! font datastructures and fonctions made from Font.cpp of SFML

use self::ft::{
    bitmap::PixelMode,
    face::{Face, LoadFlag},
    library::Library,
};
use super::Vector;
use crate::rect::Rect;
use crate::texture::{RgbMode, Texture};
use std::{collections::HashMap, error::Error, fmt};

extern crate freetype as ft;

static SIZE: u32 = 10;

/// Map defining a font <size -> Glyphmap>
type FontMap = HashMap<u32, GlyphMap>;

/// Map for each character <code -> Graphical Informations>
type Utf8Map = HashMap<u32, CharInfo>;

#[derive(Debug)]
/// # GlyphMap
/// A glyphmap represent a font for the size x.
/// ## Texture
/// the texture contain all needed character from
/// all text which this Font is used.
/// ## Rows
/// The rows 'slice' the texture into each character and help to find place
/// for each new glyph.
/// ## Utf8Map
/// the Utf8Map store information about
/// each previously added Char to texture(Graphical offsets and textureCoords).
struct GlyphMap {
    pub texture: Texture,
    pub rows: Vec<Row>,
    pub map: Utf8Map,
}

#[derive(Debug, Clone)]
struct Row {
    pub width: u32,
    pub height: u32,
    pub pos: u32,
}

impl Row {
    pub fn new(height: u32, pos: u32) -> Row {
        Row {
            width: 0,
            height,
            pos,
        }
    }
}

/// Contain the global texture and texture information
impl GlyphMap {
    /// Create a new glyph_map
    pub fn new() -> GlyphMap {
        let mut data: Vec<u8> = vec![255; 128 * 128 * 4];
        for elem in data.chunks_mut(4) {
            elem[3] = 0
        }

        GlyphMap {
            texture: Texture::from_slice(data.as_mut_slice(), RgbMode::RGBA, 128, 128),
            rows: Vec::with_capacity(1),
            map: Utf8Map::with_capacity(10),
        }
    }

    /// Get texture rect from width and height of a char.
    /// And return information about newly inserted char.
    /// Heavy function.
    pub fn get_texture_rect(&mut self, width: u32, height: u32) -> Rect<u32> {
        let mut ret: Option<Rect<u32>> = None;
        // Iter over all element
        for mut row in self.rows.iter_mut() {
            if (row.width + width > self.texture.width()) || (row.height < height) {
                continue;
            }
            ret = Some(Rect::new(row.width, row.pos, width, height));
            row.width += width + 1;
        }

        if let Some(rect) = ret {
            // Return the rect
            rect
        } else {
            // iter on row to have the most y
            let last_pos = self.rows.iter().map(|x| x.height).sum();

            // while last_pos
            while last_pos + (height + height / 10) > self.texture.height()
                || width > self.texture.width()
            {
                let mut new = Texture::from_size(Vector::new(
                    self.texture.width() * 2,
                    self.texture.height() * 2,
                ));
                new.update_from_texture(&self.texture, Vector::new(0, 0))
                    .unwrap();
                self.texture = new;
            }
            let mut new_row = Row::new(height + height / 10, last_pos);
            let new_ret = Rect::new(new_row.width, last_pos, width, height + height / 10);
            new_row.width += width + 1;
            self.rows.push(new_row);
            new_ret
        }
    }

    /// Create a new texture from Utf8Map
    pub fn update_texture(&mut self, char_info: &CharInfo, data: &[u8]) -> Result<(), Box<Error>> {
        self.texture.update_block(
            data,
            Vector::new(char_info.tex_coord.width, char_info.tex_coord.height),
            Vector::new(char_info.tex_coord.left, char_info.tex_coord.top),
            RgbMode::RGBA,
        )?;
        Ok(())
    }
}

#[derive(Debug, Default)]
/// # CharInfo
/// CharInfo are data struct used into Utf8Map.
/// ## Rect
/// Is the offsets of the glyph like bearing etc...
/// ## tex_coord
/// Is the TexCoord of the char inside the GlyphMap.
/// ## advance
/// Is the global x offset between the previous char and the next one.
pub struct CharInfo {
    pub rect: Rect<f32>,
    pub tex_coord: Rect<u32>,
    pub advance: f32,
}

impl CharInfo {
    /// Create an empty CharInfo
    pub fn new() -> CharInfo {
        CharInfo {
            rect: Default::default(),
            tex_coord: Default::default(),
            advance: 0.0,
        }
    }

    /// Create CharInfo from data
    pub fn from_data(rect: Rect<f32>, tex_coord: Rect<u32>, advance: f32) -> CharInfo {
        CharInfo {
            rect,
            tex_coord,
            advance,
        }
    }
}

/// Contain a face and everything needed to render a glyph.
/// The font have to be modified by the Text that old it
/// so most of the time you will need to wrap it into MutResource<Self>.
pub struct Font {
    face: Face,
    lib: Library,
    map: FontMap,
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
    /// Create a new font from a file path.
    pub fn from_path(path: &str) -> Option<Font> {
        let lib = Library::init().unwrap();
        match lib.new_face(path, 0) {
            Err(err) => {
                println!("Font loading error: {}", err);
                None
            }
            Ok(face) => {
                face.set_pixel_sizes(0, 30).unwrap();
                Some(Font {
                    face,
                    lib,
                    map: FontMap::with_capacity(1),
                })
            }
        }
    }

    /// Check if a glyph exist.
    pub fn glyph_exist(&mut self, size: u32, code: u32) -> bool {
        if let Some(ref mut map_size) = self.map.get(&size) {
            if let Some(ref mut _char_info) = map_size.map.get(&code) {
                return true;
            }
        }
        false
    }

    /// Get mutable FontMap.
    fn get_map_mut(&mut self) -> &mut FontMap {
        &mut self.map
    }

    /// Create a glyph if the previously asked isn't already created.
    /// Heavy fonction.
    fn create_glyph<'a>(&'a mut self, size: u32, code: u32) -> Result<&'a CharInfo, Box<Error>> {
        {
            // Get the glyph map
            let glyph_map = self.map.entry(size).or_insert_with(GlyphMap::new);

            // Load the right glyph
            self.face.load_char(code as usize, LoadFlag::RENDER)?;

            let metrics = self.face.glyph().metrics();
            let bitmap = self.face.glyph().bitmap();
            // Create the new Charinfo that will be inserted

            let height = bitmap.rows();
            let width = bitmap.width();

            // Get the glyph and informations
            let mut to_insert = CharInfo::new();
            to_insert.rect.left = metrics.horiBearingX as f32 / (1 << 6) as f32;
            to_insert.rect.top = -metrics.horiBearingY as f32 / (1 << 6) as f32;
            to_insert.rect.width = metrics.width as f32 / (1 << 6) as f32;
            to_insert.rect.height = metrics.height as f32 / (1 << 6) as f32;
            to_insert.advance = (metrics.horiAdvance + 2) as f32 / (1 << 6) as f32;

            // Look at the glyph texture and try to find a place inside it
            to_insert.tex_coord = glyph_map.get_texture_rect(width as u32, height as u32);

            // Resize buffer
            let mut data = vec![255; (height * width * 4) as usize];
            for elem in &mut data.chunks_mut(4) {
                elem[3] = 0;
            }

            // fill pixel buffer
            let pixels: Vec<u8> = Vec::from(bitmap.buffer());
            let mut offset = 0;
            match bitmap.pixel_mode().unwrap() {
                PixelMode::None => {
                    panic!("Error while creating glyph");
                }
                PixelMode::Mono => {
                    // If it's mono just change the alpha of each pixel to make it black or white
                    for y in 0..height {
                        for x in 0..width {
                            let index = ((x + y * width) * 4 + 3) as usize;
                            let pix = pixels[(offset + (x / 8)) as usize];

                            data[index] = if (pix & (1 << (7 - (x % 8)))) != 0 {
                                255
                            } else {
                                0
                            };
                        }
                        offset += bitmap.pitch();
                    }
                }
                _ => {
                    // Just change the alpha to make thw whole blakc or white
                    for y in 0..height {
                        for x in 0..width {
                            let index = ((x + y * width) * 4 + 3) as usize;
                            let pix = pixels[(offset + x) as usize];

                            data[index] = pix;
                        }
                        offset += bitmap.pitch();
                    }
                }
            }

            // Update the texture at the right position
            glyph_map.update_texture(&to_insert, data.as_slice())?;

            // Insert the new glyph map into the hasmap
            glyph_map.map.insert(code, to_insert);
        }

        // Return the newly inserted charinfo
        Ok(&self.get_map_mut()[&size].map[&code])
    }

    /// Check if the glyph exist:
    /// If the glyph exist get GraphicChar from it
    /// Else add it to the row and update the texture
    pub fn glyph(&mut self, size: u32, code: u32) -> &CharInfo {
        let glyph_exist: bool = self.glyph_exist(size, code);

        if glyph_exist {
            &self.map[&size].map[&code]
        } else {
            self.create_glyph(size, code).unwrap()
        }
    }

    pub fn texture(&self, font_size: u32) -> Result<&Texture, TextError> {
        if let Some(t) = &self.map.get(&font_size) {
            Ok(&t.texture)
        } else {
            Err(TextError::NoTexture)
        }
    }
}

#[derive(Debug)]
pub enum TextError {
    NoTexture,
}

impl fmt::Display for TextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TextError::NoTexture => write!(f, "No texture try updating before."),
        }
    }
}

impl Error for TextError {
    fn cause(&self) -> Option<&Error> {
        match self {
            TextError::NoTexture => None,
        }
    }
}
