//
//  Rust file | 2018
//  Author: Alexandre Fourcat
//  help.rs
//  module:
//! a

extern crate freetype as ft;


const WIDTH: usize = 32;
const HEIGHT: usize = 24;

pub fn draw_bitmap(bitmap: &ft::Bitmap, x: usize, y: usize) -> [[u8; WIDTH]; HEIGHT] {
    let mut figure = [[0; WIDTH]; HEIGHT];
    let mut p = 0;
    let mut q = 0;
    let w = bitmap.width() as usize;
    let x_max = x + w;
    let y_max = y + bitmap.rows() as usize;

    for i in x .. x_max {
        for j in y .. y_max {
            if i < WIDTH && j < HEIGHT {
                figure[j][i] |= bitmap.buffer()[q * w + p];
                q += 1;
            }
        }
        q = 0;
        p += 1;
    }
    for i in 0 .. HEIGHT {
        for j in 0 .. WIDTH {
            print!("{}",
                match figure[i][j] {
                    p if p == 0 => " ",
                    p if p < 128 => "*",
                    _  => "+"
                }
            );
        }
        println!("");
    }
    figure
}
