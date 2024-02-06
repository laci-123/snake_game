use crate::color::*;
use super::*;


const CANVAS_WIDTH:  usize = 800;
const CANVAS_HEIGHT: usize = 450;
const GRAVITY: f32 = 50.0;


pub struct Game {
    w: f32,
    h: f32,
    vx: f32,
    vy: f32,
    x: f32,
    y: f32,
}

impl Game {
    pub const fn new() -> Self {
        Self { x: 0.0, y: 0.0, w: 100.0, h: 50.0, vx: 40.0, vy: 0.0 }
    }
    
    pub fn update(&mut self, dt: f32) {
        self.vy += GRAVITY * dt;
        
        let x = self.x + self.vx * dt;
        if x < 0.0 || (CANVAS_WIDTH as f32) < x + self.w {
            self.vx *= -1.0;
        }
        else {
            self.x = x;
        }
        let y = self.y + self.vy * dt;
        if y < 0.0 || (CANVAS_HEIGHT as f32) < y + self.h {
            self.vy *= -1.0;
        }
        else {
            self.y = y;
        }
    }

    pub fn render(&mut self) {
        fill_rect(self.x, self.y, self.w, self.h, Color::rgb(0, 255, 255));
    }
}
