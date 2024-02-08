use color::Color;
use game::*;
use lazy_static::lazy_static;
use std::sync::Mutex;


extern "C" {
    fn js_fill_circle(x: f32, y: f32, r: f32, color: u32);
    fn js_fill_text(text_ptr: i32, text_len: i32, x: f32, y: f32, color: u32, font_size: i32);
}

fn fill_circle(x: f32, y: f32, width: f32, color: Color) {
    unsafe {
        js_fill_circle(x, y, width, color.to_u32());
    }
}

fn fill_text(text: &str, x: f32, y: f32, color: Color, font_size: i32) {
    let text_ptr = (text.as_ptr() as usize) as i32;
    let text_len = text.len() as i32;
    unsafe {
        js_fill_text(text_ptr, text_len, x, y, color.to_u32(), font_size);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Input {
    ArrowRight,
    ArrowUp,
    ArrowLeft,
    ArrowDown,
}

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

#[no_mangle]
pub extern "C" fn update(dt: f32) {
    GAME.lock().unwrap().update(dt);
}

#[no_mangle]
pub extern "C" fn render() {
    GAME.lock().unwrap().render();
}

#[no_mangle]
pub extern "C" fn input(x: i32) {
    match x {
        0 => GAME.lock().unwrap().input(Input::ArrowRight),
        1 => GAME.lock().unwrap().input(Input::ArrowUp),
        2 => GAME.lock().unwrap().input(Input::ArrowLeft),
        3 => GAME.lock().unwrap().input(Input::ArrowDown),
        _ => unreachable!(),
    }
}


mod color;
mod game;
