extern crate gust;
extern crate glfw;

use gust::sprite::Sprite;
use gust::window::Window;
use gust::{Vector,Point,Key};
use gust::event;
use gust::event::{EventHandler,Events,Event};
use std::rc::Rc;
use gust::color::Color;
use gust::texture::{Texture};
use gust::draw::{Drawer,Movable};
use gust::draw::Drawable;
use gust::texture::RgbMode;

fn main()
{
    let mut window = Window::new(gust::WIDTH, gust::HEIGHT, "Hello");
    let _tex_leave = Rc::new( Texture::from_path("examples/texture/Z.png").unwrap());
    let mut pixels: Vec<u8> = vec![125; 200 * 200 * 4];
    let pix: Vec<u8> = vec![255; 100 * 100 * 3];
    let mut my_tex = Texture::from_path("examples/texture/New.png").unwrap();
    let mut blank;
    let mut texture = Texture::from_path("examples/texture/Dirt.png").unwrap();

    unsafe {
        use std::os::raw::c_void; 
        blank = Texture::from_data(pixels.as_mut_ptr() as *mut c_void, RgbMode::RGBA, 200, 200);
    }

    texture.to_file("before.png");
    my_tex.update_block(pix.as_slice(), Vector::new(100, 100), Vector::new(10, 10), None);
    texture.update_from_texture(&blank, Vector::new(10, 10));
    texture.to_file("after.png");

    let blank_rc = Rc::new(blank);
    let tex_dirt = Rc::new(my_tex);
    let event_handler = EventHandler::new(&window);
    let mut sprite = Sprite::from(&tex_dirt);
    let mut leave = Sprite::from(&blank_rc);
    let mut sprite2 = Sprite::from(&tex_dirt);
        
    leave.set_position(Point::new(600.0, 100.0));
    sprite.set_position(Vector::new(100.0, 100.0));
    sprite2.set_position(Point::new(1000.0, 100.0));
    sprite.update();
    sprite2.update();
    leave.update();

    window.poll(None);
    window.enable_cursor();
    window.set_clear_color(Color::new(0.0, 0.0, 1.0));
    while window.is_open() {
        window.poll_events();

        for event in event_handler.fetch() {
            event_process(event, &mut window, &tex_dirt, &mut sprite);
        }
        
        window.clear();
        window.draw(&sprite);
        window.draw(&leave);
        window.draw(&sprite2);
        window.display();
    }
}

fn event_process(event: Event, window: &mut Window, texture: &Texture, sprite: &mut Sprite) {
    match event.1 {
        Events::Key(Key::Escape, _, _, _) => {
            window.close();
        },
        Events::MouseButton(_, _, _) => {
            println!("Dumping texture to test.png");
            texture.to_file("test.png");
        },
        Events::CursorPos(x, y) => {
            sprite.set_position(Vector::new(x as f32, y as f32));
        },
        _ => {}
    }
}
