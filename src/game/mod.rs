use vector2d::Vector2D;
use crate::color::*;
use snake::Snake;
use food::*;
use super::*;


#[derive(PartialEq, Eq)]
enum GameStatus {
    Playing,
    Paused,
    Dying,
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
    canvas_width: f32,
    canvas_height: f32,
    status: GameStatus,
    snake: Snake,
    food: Food,
    texts: Vec<Text>,
    score: i32,
    collision_pos: Vector2D<f32>,
    collision_size: f32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            canvas_width: 100.0,
            canvas_height: 100.0,
            status: GameStatus::Playing,
            snake: Snake::new(Vector2D::new(random_between(10.0, 500.0), random_between(10.0, 500.0)), 10.0, Color::rgb(0, 255, 255)),
            food: Food::random(Color::rgb(255, 255, 0), 1000.0, 1000.0),
            texts: Vec::new(),
            score: 0,
            collision_pos: Vector2D::new(0.0, 0.0),
            collision_size: 1.0,
        }
    }

    fn reset(&mut self) {
        self.status = GameStatus::Playing;
        self.snake = Snake::new(Vector2D::new(random_between(10.0, self.canvas_width - 10.0), random_between(10.0, self.canvas_height - 10.0)), 10.0, Color::rgb(0, 255, 255));
        self.food = Food::random(Color::rgb(255, 255, 0), self.canvas_width, self.canvas_height);
        self.texts.clear();
        self.score = 0;
        self.collision_size = 1.0;
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.canvas_width = width;
        self.canvas_height = height;

        self.snake.shift_into_canvas(width, height);
        self.food.shift_into_canvas(width, height);
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
                        self.food = Food::random(Color::rgb(255, 255, 0), self.canvas_width, self.canvas_height);
                    },
                    _ => {},
                }

                let s = self.snake.update(dt, &mut self.food, self.canvas_width, self.canvas_height); 

                if let Err(collision_pos) = s {
                    self.collision_pos = collision_pos;
                    self.status = GameStatus::Dying;
                }
            },
            GameStatus::Paused => {
                self.show_text("Paused", self.canvas_width / 2.0, self.canvas_height / 2.0, Color::rgb(255, 0, 0), 50, TextAlignment::Center);
                self.show_text("press space to unpause", self.canvas_width / 2.0, self.canvas_height / 2.0 + 30.0, Color::rgb(255, 0, 0), 25, TextAlignment::Center);
            },
            GameStatus::Dying => {
                if self.collision_size < self.canvas_width {
                    fill_circle(self.collision_pos.x,
                                self.collision_pos.y,
                                self.collision_size,
                                Color { r: 255, g: 0, b: 0, a: (255.0 * (1.0 - self.collision_size / self.canvas_width)) as u8 });
                    self.collision_size += 500.0 * dt;
                }
                else {
                    self.status = GameStatus::Over;
                }
            },
            GameStatus::Over => {
                self.show_text("Game Over", self.canvas_width / 2.0, self.canvas_height / 2.0, Color::rgb(255, 0, 0), 50, TextAlignment::Center);
                self.show_text("press R to restart", self.canvas_width / 2.0, self.canvas_height / 2.0 + 30.0, Color::rgb(255, 0, 0), 25, TextAlignment::Center);
            },
        }
        self.show_text(&self.score.to_string(), 5.0, 20.0, Color::rgb(255, 255, 255), 20, TextAlignment::Left);
    }

    pub fn input(&mut self, input: Input) {
        match input {
            Input::PauseUnpause => {
                if self.status == GameStatus::Playing {
                    self.status = GameStatus::Paused;
                }
                else if self.status == GameStatus::Paused {
                    self.status = GameStatus::Playing;
                }
            },
            Input::Restart => {
                if self.status == GameStatus::Over {
                    self.reset();
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
        self.food.render();
        self.snake.render();
        for t in self.texts.drain(..) {
            t.render();
        }
    }

    fn show_text(&mut self, text: &str, x: f32, y: f32, color: Color, font_size: i32, alignment: TextAlignment) {
        self.texts.push(Text::new(text.to_owned(), x, y, color, font_size, alignment))
    }
}


mod snake;
mod food;
