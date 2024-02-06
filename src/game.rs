use crate::color::*;
use super::*;


pub struct Game {
    x: f32,
    y: f32,
}

impl Game {
    pub const fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    
    pub fn update(&mut self, dt: f32) {
        self.x += dt * 20.0;
        self.y += dt * 10.0;
    }

    pub fn render(&mut self) {
        fill_rect(self.x, self.y, 100, 50, Color::rgb(0, 0, 255));
    }
}
