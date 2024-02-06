use crate::color::*;
use super::*;


const CANVAS_WIDTH:  usize = 800;
const CANVAS_HEIGHT: usize = 450;
const GRAVITY: f32 = 50.0;
const BOUNCE_LOSS: f32 = 0.9;


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
            self.vx *= BOUNCE_LOSS;
        }
        else {
            self.x = x;
        }
        let y = self.y + self.vy * dt;
        if y < 0.0 || (CANVAS_HEIGHT as f32) < y + self.h {
            self.vy *= -1.0;
            self.vy *= BOUNCE_LOSS;
        }
        else {
            self.y = y;
        }
    }

    pub fn input(&mut self, input: Input) {
        use Input::*;
        
        match input {
            ArrowRight => self.vx += 20.0,
            ArrowUp    => self.vy -= 20.0,
            ArrowLeft  => self.vx -= 20.0,
            ArrowDown  => self.vy += 20.0,
        }
    }

    pub fn render(&mut self) {
        fill_rect(self.x, self.y, self.w, self.h, Color::rgb(0, 255, 255));
    }
}
