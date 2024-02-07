use color::Color;
use game::*;
use lazy_static::lazy_static;
use std::sync::Mutex;


extern "C" {
    fn js_fill_rect(x: f32, y: f32, width: f32, height: f32, color: u32);
}

fn fill_rect(x: f32, y: f32, width: f32, height: f32, color: Color) {
    unsafe {
        js_fill_rect(x, y, width, height, color.to_u32());
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
