use crate::color::*;
use vector2d::Vector2D;
use super::*;


const CANVAS_WIDTH:  usize = 800;
const CANVAS_HEIGHT: usize = 450;


struct SnakeCell {
    position: Vector2D<f32>,
    velocity: Vector2D<f32>,
    size: f32,
    color: Color,
}


struct Snake {
    cells: Vec<SnakeCell>,
    cell_distance: f32,
}

impl Snake {
    fn new(position: Vector2D<f32>, size: f32, color: Color, cell_distance: f32) -> Self {
        let mut cells = Vec::new();
        for i in 0..5 {
            let cell = SnakeCell { position: position - Vector2D::new((i as f32) * cell_distance, 0.0), size, color, velocity: Vector2D::new(10.0, 0.0) };
            cells.push(cell);
        }
        Self { cells, cell_distance }
    }

    fn update(&mut self, dt: f32) -> GameStatus {
        let Some(first_cell) = self.cells.get_mut(0) else {
            unreachable!(); // Self is constructed with one cell and we never remove cells ==> there is always at least one cell
        };

        let p = first_cell.position + first_cell.velocity * dt;
        let r = first_cell.size;
        if p.x - r < 0.0 || (CANVAS_WIDTH as f32) < p.x + r || p.y - r < 0.0 || (CANVAS_HEIGHT as f32) < p.y + r {
            return GameStatus::Over;
        }

        first_cell.position = p;

        for i in 0 .. (self.cells.len() - 1) { // self.cells.len() is always at least 1
            let c0 = &self.cells[i];
            let c1 = &self.cells[i + 1];

            let dx2 = (c0.position - c1.position).length_squared();
            let alpha = (dx2 - self.cell_distance * self.cell_distance) * 1.0;
            let direction = (c0.position - c1.position).normalise();

            self.cells[i + 1].velocity = direction * alpha;
            let dp = self.cells[i + 1].velocity * dt;
            self.cells[i + 1].position += dp;
        }

        GameStatus::Playing
    }
}


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
        let cell_size     = 10.0;
        let cell_distance = 5.0;
        let head_position = Vector2D::new((CANVAS_WIDTH / 2) as f32, (CANVAS_HEIGHT / 2) as f32);
        Self {
            status: GameStatus::Playing,
            snake: Snake::new(head_position, cell_size, Color::rgb(0, 255, 255), cell_distance)
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
        use Input::*;
        
        if let Some(first_cell) = self.snake.cells.get_mut(0) {
            match input {
                ArrowRight => first_cell.velocity.x += 10.0,
                ArrowUp    => first_cell.velocity.y -= 10.0,
                ArrowLeft  => first_cell.velocity.x -= 10.0,
                ArrowDown  => first_cell.velocity.y += 10.0,
            }
        }
    }

    pub fn render(&mut self) {
        for cell in self.snake.cells.iter() {
            fill_circle(cell.position.x, cell.position.y, cell.size, cell.color);
        }
    }
}
