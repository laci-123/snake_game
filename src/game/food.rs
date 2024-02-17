use super::*;


#[derive(PartialEq, Eq)]
pub enum FoodStatus {
    StillThere,
    JustEaten,
    Eaten,
    Gone,
}


pub struct Food {
    pub position: Vector2D<f32>,
    pub size: f32,
    pub original_size: f32,
    pub color: Color,
    pub status: FoodStatus,
    pub value: i32,
}

impl Food {
    pub fn random(color: Color, canvas_width: f32, canvas_height: f32) -> Self {
        let size = random_between(5.0, 30.0);
        Self {
            position: Vector2D::new(random_between(size, canvas_width - size), random_between(size, canvas_height - size)),
            size,
            original_size: size,
            color,
            status: FoodStatus::StillThere,
            value: ((35.0 - size) / 5.0) as i32,
        }
    }


    pub fn shift_into_canvas(&mut self, canvas_width: f32, canvas_height: f32) {
        let x = self.position.x + self.size - canvas_width;
        if x > 0.0 {
            self.position.x -= x;
        }
        let y = self.position.y + self.size - canvas_height;
        if y > 0.0 {
            self.position.y -= y;
        }
    }

    pub fn update(&mut self, dt: f32) {
        match self.status {
            FoodStatus::StillThere => {
                self.size -= 1.5 * dt;
                if self.size < 1.0 {
                    self.status = FoodStatus::Gone;
                }
            },
            FoodStatus::JustEaten => {},
            FoodStatus::Eaten => {
                self.size -= 0.5 * self.original_size * dt;
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
