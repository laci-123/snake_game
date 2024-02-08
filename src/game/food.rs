use super::*;


pub struct Food {
    pub position: Vector2D<f32>,
    pub size: f32,
    pub color: Color,
    pub eaten: bool,
    pub gone: bool,
}

impl Food {
    pub fn random(color: Color) -> Self {
        let size = random_between(5.0, 30.0);
        Self {
            position: Vector2D::new(random_between(size, CANVAS_WIDTH as f32 - size), random_between(size, CANVAS_HEIGHT as f32 - size)),
            size,
            color,
            eaten: false,
            gone: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.eaten && !self.gone {
            self.size -= 10.0 * dt;
        }
        if self.size < 1.0 {
            self.gone = true;
        }
    }

    pub fn render(&self) {
        fill_circle(self.position.x, self.position.y, self.size, self.color);
    }
}
