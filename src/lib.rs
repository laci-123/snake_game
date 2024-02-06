use color::Color;
use game::*;
use std::sync::Mutex;


extern "C" {
    fn js_fill_rect(x: f32, y: f32, width: f32, height: f32, color: u32);
}

fn fill_rect(x: f32, y: f32, width: f32, height: f32, color: Color) {
    unsafe {
        js_fill_rect(x, y, width, height, color.to_u32());
    }
}


static GAME: Mutex<Game> = Mutex::new(Game::new());

#[no_mangle]
pub extern "C" fn update(dt: f32) {
    GAME.lock().unwrap().update(dt);
}

#[no_mangle]
pub extern "C" fn render() {
    GAME.lock().unwrap().render();
}


mod color;
mod game;
