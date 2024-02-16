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
    alignment: TextAlignment,
}

impl Text {
    fn new(text: String, x: f32, y: f32, color: Color, size: i32, alignment: TextAlignment) -> Self {
        Self { text, x, y, color, size, alignment }
    }

    fn render(self) {
        fill_text(&self.text, self.x, self.y, self.color, self.size, self.alignment)
    }
}


pub struct Game {
    status: GameStatus,
    snake: Snake,
    food: Food,
    texts: Vec<Text>,
    score: i32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            status: GameStatus::Playing,
            snake: Snake::new(Vector2D::new(MIDDLE_X, MIDDLE_Y), 10.0, Color::rgb(0, 255, 255)),
            food: Food::random(Color::rgb(255, 255, 0)),
            texts: Vec::new(),
            score: 0,
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        match self.status {
            GameStatus::Playing => {
                self.food.update(dt);
                match self.food.status {
                    FoodStatus::JustEaten => {
                        self.score += self.food.value;
                        self.food.status = FoodStatus::Eaten;
                    },
                    FoodStatus::Eaten => {
                        self.show_text(&format!("+{}", self.food.value),
                                       self.food.position.x,
                                       self.food.position.y - 2.0 * self.food.original_size + 2.0 * self.food.size,
                                       Color{ r: 255, g: 255, b: 0, a: (255.0 * (self.food.size / self.food.original_size)) as u8 },
                                       20,
                                       TextAlignment::Center);
                    },
                    FoodStatus::Gone => {
                        self.food = Food::random(Color::rgb(255, 255, 0));
                    },
                    _ => {},
                }
                if let GameStatus::Over = self.snake.update(dt, &mut self.food) {
                    self.status = GameStatus::Over;
                }
            },
            GameStatus::Paused => {
                self.show_text("Paused", MIDDLE_X, MIDDLE_Y, Color::rgb(255, 0, 0), 40, TextAlignment::Center);
                self.show_text("press space to unpause", MIDDLE_X, MIDDLE_Y + 20.0, Color::rgb(255, 0, 0), 15, TextAlignment::Center);
            },
            GameStatus::Over => {
                self.show_text("Game Over", MIDDLE_X, MIDDLE_Y, Color::rgb(255, 0, 0), 40, TextAlignment::Center);
                self.show_text("press R to restart", MIDDLE_X, MIDDLE_Y + 20.0, Color::rgb(255, 0, 0), 15, TextAlignment::Center);
            },
        }
        self.show_text(&self.score.to_string(), 5.0, 20.0, Color::rgb(255, 255, 255), 20, TextAlignment::Left);
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

    fn show_text(&mut self, text: &str, x: f32, y: f32, color: Color, font_size: i32, alignment: TextAlignment) {
        self.texts.push(Text::new(text.to_owned(), x, y, color, font_size, alignment))
    }
}


mod snake;
mod food;
