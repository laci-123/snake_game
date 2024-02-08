use crate::color::*;
use vector2d::Vector2D;
use snake::Snake;
use super::*;


enum GameStatus {
    Playing,
    Over,
}


pub struct Game {
    status: GameStatus,
    snake: Snake,
}

impl Game {
    pub fn new() -> Self {
        Self {
            status: GameStatus::Playing,
            snake: Snake::new(Vector2D::new(MIDDLE_X, MIDDLE_Y), 10.0, Color::rgb(0, 255, 255))
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        match self.status {
            GameStatus::Playing => {
                if let GameStatus::Over = self.snake.update(dt) {
                    self.status = GameStatus::Over;
                }
            },
            GameStatus::Over => {
                fill_text("Game Over", (CANVAS_WIDTH / 2) as f32, (CANVAS_HEIGHT / 2) as f32, Color::rgb(255, 0, 0), 30);
            },
        }
    }

    pub fn input(&mut self, input: Input) {
        self.snake.input(input);
    }

    pub fn render(&mut self) {
        self.snake.render();
    }
}


mod snake;
