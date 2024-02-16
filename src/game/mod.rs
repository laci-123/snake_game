use vector2d::Vector2D;
use crate::color::*;
use snake::Snake;
use food::*;
use super::*;


#[derive(PartialEq, Eq)]
enum GameStatus {
    Playing,
    Paused,
    Over,
}


struct Text {
    text: String,
    x: f32,
    y: f32,
    color: Color,
    size: i32,
}

impl Text {
    fn new(text: String, x: f32, y: f32, color: Color, size: i32) -> Self {
        Self { text, x, y, color, size }
    }

    fn render(self) {
        fill_text(&self.text, self.x, self.y, self.color, self.size)
    }
}


pub struct Game {
    status: GameStatus,
    snake: Snake,
    food: Food,
    texts: Vec<Text>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            status: GameStatus::Playing,
            snake: Snake::new(Vector2D::new(MIDDLE_X, MIDDLE_Y), 10.0, Color::rgb(0, 255, 255)),
            food: Food::random(Color::rgb(255, 255, 0)),
            texts: Vec::new(),
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
            GameStatus::Paused => {
                self.show_text("Paused", MIDDLE_X, MIDDLE_Y, Color::rgb(255, 0, 0), 40);
                self.show_text("press space to unpause", MIDDLE_X, MIDDLE_Y + 20.0, Color::rgb(255, 0, 0), 15);
            },
            GameStatus::Over => {
                self.show_text("Game Over", MIDDLE_X, MIDDLE_Y, Color::rgb(255, 0, 0), 40);
                self.show_text("press R to restart", MIDDLE_X, MIDDLE_Y + 20.0, Color::rgb(255, 0, 0), 15);
            },
        }
    }

    pub fn input(&mut self, input: Input) {
        match input {
            Input::Space => {
                if self.status == GameStatus::Playing {
                    self.status = GameStatus::Paused;
                }
                else if self.status == GameStatus::Paused {
                    self.status = GameStatus::Playing;
                }
            },
            Input::R => {
                if self.status == GameStatus::Over {
                    // restart everything
                    *self = Self::new();
                }
            },
            _ => {
                if self.status == GameStatus::Playing {
                    self.snake.input(input);
                }
            },
        }
    }

    pub fn render(&mut self) {
        self.snake.render();
        self.food.render();
        for t in self.texts.drain(..) {
            t.render();
        }
        self.texts.clear();
    }

    fn show_text(&mut self, text: &str, x: f32, y: f32, color: Color, font_size: i32) {
        self.texts.push(Text::new(text.to_owned(), x, y, color, font_size))
    }
}


mod snake;
mod food;
