use super::*;


struct SnakeCell {
    position: Vector2D<f32>,
    velocity: Vector2D<f32>,
    size: f32,
    color: Color,
}


pub struct Snake {
    cells: Vec<SnakeCell>,
    cell_distance: f32,
}

impl Snake {
    pub fn new(position: Vector2D<f32>, size: f32, color: Color) -> Self {
        let mut cells = Vec::new();
        let cell_distance = size / 2.0;
        for i in 0..10 {
            let cell = SnakeCell { position: position - Vector2D::new((i as f32) * cell_distance, 0.0), size, color, velocity: Vector2D::new(10.0, 0.0) };
            cells.push(cell);
        }
        let mut this = Self { cells, cell_distance };
        this.color_cells();
        this
    }

    pub fn shift_into_canvas(&mut self, canvas_width: f32, canvas_height: f32) {
        let mut max_x = 0.0;
        let mut max_y = 0.0;
        for cell in self.cells.iter() {
            let x = cell.position.x + cell.size - canvas_width;
            if x > max_x {
                max_x = x;
            }
            let y = cell.position.y + cell.size - canvas_height;
            if y > max_y {
                max_y = y;
            }
        }

        let v = Vector2D::new(max_x, max_y);
        if v.length_squared() > 0.0 {
            for cell in self.cells.iter_mut() {
                cell.position -= v;
            }
        }
    }

    fn get_head(&self) -> &SnakeCell {
        self.cells.first().unwrap() // Self is constructed with one cell and we never remove cells ==> there is always at least one cell
    }

    fn get_head_mut(&mut self) -> &mut SnakeCell {
        self.cells.first_mut().unwrap() // Self is constructed with one cell and we never remove cells ==> there is always at least one cell
    }

    fn get_tail(&self) -> &SnakeCell {
        self.cells.last().unwrap() // Self is constructed with one cell and we never remove cells ==> there is always at least one cell
    }

    fn color_cells(&mut self) {
        let total = self.cells.len();
        let hc = self.get_head().color;
        for (i, cell) in self.cells.iter_mut().enumerate() {
            cell.color = hc.darken((total - i) as f32 / total as f32);
        }
    }

    pub fn update(&mut self, dt: f32, food: &mut Food, canvas_width: f32, canvas_height: f32) -> Result<(), Vector2D<f32>> {
        let cell_distance = self.cell_distance;
        let head = self.get_head();

        if food.status == FoodStatus::StillThere && (food.position - head.position).length() <= food.size + head.size {
            for i in 0..5 {
                let tail = self.get_tail();
                let cell = SnakeCell { position: tail.position - Vector2D::new((i as f32) * cell_distance, 0.0), size: tail.size, color: tail.color, velocity: tail.velocity };
                self.cells.push(cell);
            }
            self.color_cells();
            food.status = FoodStatus::JustEaten;
        }


        let head = self.get_head_mut();
        let max_velocity = 400.0;
        if head.velocity.length_squared() > max_velocity * max_velocity {
            head.velocity = head.velocity.normalise() * max_velocity;
        }

        let p = head.position + head.velocity * dt;
        let r = head.size;
        if p.x - r < 0.0 || canvas_width < p.x + r { 
            head.velocity.x *= -0.9;
        }
        else if p.y - r < 0.0 || canvas_height < p.y + r {
            head.velocity.y *= -0.9;
        }
        else {
            head.position = p;
        }

        for i in 0 .. (self.cells.len() - 1) { // self.cells.len() is always at least 1
            let c0 = &self.cells[i];
            let c1 = &self.cells[i + 1];

            let dx2 = (c0.position - c1.position).length_squared();

            if dx2 > (self.cell_distance * 6.0).powi(2) {
                // if snake losses its tail then recreate it 
                let head      = self.get_head();
                let position  = head.position;
                let size      = head.size;
                let color     = head.color;
                let velocity  = head.velocity * 0.9;
                let direction = head.velocity.normalise();
                let n_cells   = self.cells.len();
                self.cells.clear();
                for i in 0..n_cells {
                    let cell = SnakeCell { position: position - direction * self.cell_distance * (i as f32), size, color, velocity };
                    self.cells.push(cell);
                }
                self.color_cells();
                break;
            }

            let alpha = (dx2 - self.cell_distance * self.cell_distance) * 0.75;
            let direction = (c0.position - c1.position).normalise();

            self.cells[i + 1].velocity = direction * alpha;
            let dp = self.cells[i + 1].velocity * dt;
            self.cells[i + 1].position += dp;
        }

        let hp = self.get_head().position;
        for cell in &mut self.cells[5..] {
            if (cell.position - hp).length() < cell.size {
                cell.color = Color::rgb(255, 0, 0);
                self.cells[0].color = Color::rgb(255, 0, 0);
                return Err(hp);
            }
        }

        Ok(())
    }

    pub fn input(&mut self, input: Input) {
        use Input::*;
 
        let first_cell = self.get_head_mut();
        match input {
            MoveRight => first_cell.velocity.x += 20.0,
            MoveUp    => first_cell.velocity.y -= 20.0,
            MoveLeft  => first_cell.velocity.x -= 20.0,
            MoveDown  => first_cell.velocity.y += 20.0,
            _         => {},
        }
    }

    pub fn render(&self) {
        for cell in self.cells.iter() {
            fill_circle(cell.position.x, cell.position.y, cell.size, cell.color);
        }
    }
}
