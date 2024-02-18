use color::Color;
use game::*;
use lazy_static::lazy_static;
use std::sync::Mutex;


extern "C" {
    fn js_fill_circle(x: f32, y: f32, r: f32, color: u32);
    fn js_fill_text(text_ptr: i32, text_len: i32, x: f32, y: f32, color: u32, font_size: i32, alignment: u8);
    fn js_random_between(min: f32, max: f32) -> f32;
}

fn fill_circle(x: f32, y: f32, width: f32, color: Color) {
    unsafe {
        js_fill_circle(x, y, width, color.to_u32());
    }
}

fn fill_text(text: &str, x: f32, y: f32, color: Color, font_size: i32, alignment: TextAlignment) {
    let text_ptr = (text.as_ptr() as usize) as i32;
    let text_len = text.len() as i32;
    unsafe {
        js_fill_text(text_ptr, text_len, x, y, color.to_u32(), font_size, alignment as u8);
    }
}

fn random_between(min: f32, max: f32) -> f32 {
    unsafe {
        js_random_between(min, max)
    }
}


#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum TextAlignment {
    Center = 0,
    Left,
    // Right,
}

#[derive(Clone, Copy, PartialEq)]
enum Input {
    MoveRight,
    MoveUp,
    MoveLeft,
    MoveDown,
    PauseUnpause,
    Restart,
}

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

#[no_mangle]
pub extern "C" fn update(dt: f32) {
    GAME.lock().unwrap().update(dt);
}

#[no_mangle]
pub extern "C" fn resize_canvas(width: f32, height: f32) {
    GAME.lock().unwrap().resize(width, height);
}

#[no_mangle]
pub extern "C" fn render() {
    GAME.lock().unwrap().render();
}

#[no_mangle]
pub extern "C" fn input(x: i32) {
    match x {
        0 => GAME.lock().unwrap().input(Input::MoveRight),
        1 => GAME.lock().unwrap().input(Input::MoveUp),
        2 => GAME.lock().unwrap().input(Input::MoveLeft),
        3 => GAME.lock().unwrap().input(Input::MoveDown),
        4 => GAME.lock().unwrap().input(Input::PauseUnpause),
        5 => GAME.lock().unwrap().input(Input::Restart),
        _ => unreachable!(),
    }
}


mod color;
mod game;
