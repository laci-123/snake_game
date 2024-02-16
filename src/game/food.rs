use super::*;


#[derive(PartialEq, Eq)]
pub enum FoodStatus {
    StillThere,
    Eaten,
    Gone,
}


pub struct Food {
    pub position: Vector2D<f32>,
    pub size: f32,
    pub color: Color,
    pub status: FoodStatus,
}

impl Food {
    pub fn random(color: Color) -> Self {
        let size = random_between(5.0, 30.0);
        Self {
            position: Vector2D::new(random_between(size, CANVAS_WIDTH as f32 - size), random_between(size, CANVAS_HEIGHT as f32 - size)),
            size,
            color,
            status: FoodStatus::StillThere,
        }
    }

    pub fn update(&mut self, dt: f32) {
        match self.status {
            FoodStatus::StillThere => {
                self.size -= 2.0 * dt;
                if self.size < 1.0 {
                    self.status = FoodStatus::Gone;
                }
            },
            FoodStatus::Eaten => {
                self.size -= 10.0 * dt;
                if self.size < 1.0 {
                    self.status = FoodStatus::Gone;
                }
            },
            FoodStatus::Gone => {},
        }
    }

    pub fn render(&self) {
        fill_circle(self.position.x, self.position.y, self.size, self.color);
    }
}
