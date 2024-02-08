use vector2d::Vector2D;
use crate::color::*;
use snake::Snake;
use food::*;
use super::*;


#[derive(PartialEq, Eq)]
enum GameStatus {
    Playing,
    Over,
}


pub struct Game {
    status: GameStatus,
    snake: Snake,
    food: Food,
}

impl Game {
    pub fn new() -> Self {
        Self {
            status: GameStatus::Playing,
            snake: Snake::new(Vector2D::new(MIDDLE_X, MIDDLE_Y), 10.0, Color::rgb(0, 255, 255)),
            food: Food::random(Color::rgb(255, 255, 0)),
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        match self.status {
            GameStatus::Playing => {
                self.food.update(dt);
                if self.food.status == FoodStatus::Gone {
                    self.food = Food::random(Color::rgb(255, 255, 0));
                }
                if let GameStatus::Over = self.snake.update(dt, &mut self.food) {
                    self.status = GameStatus::Over;
                }
            },
            GameStatus::Over => {
                fill_text("Game Over", MIDDLE_X, MIDDLE_Y, Color::rgb(255, 0, 0), 40);
                fill_text("press space to restart", MIDDLE_X, MIDDLE_Y + 20.0, Color::rgb(255, 0, 0), 15);
            },
        }
    }

    pub fn input(&mut self, input: Input) {
        match input {
            Input::Space => {
                if self.status == GameStatus::Over {
                    // restart everything
                    *self = Self::new();
                }
            },
            _ => self.snake.input(input),
        }
    }

    pub fn render(&mut self) {
        self.snake.render();
        self.food.render();
    }
}


mod snake;
mod food;
