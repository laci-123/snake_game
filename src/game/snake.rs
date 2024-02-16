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

    pub fn update(&mut self, dt: f32, food: &mut Food) -> GameStatus {
        let cell_distance = self.cell_distance;
        let head = self.get_head();

        if food.status == FoodStatus::StillThere && (food.position - head.position).length() <= food.size + head.size {
            for _ in 0..5 {
                let tail = self.get_tail();
                let cell = SnakeCell { position: tail.position - Vector2D::new(cell_distance, 0.0), size: tail.size, color: tail.color, velocity: tail.velocity };
                self.cells.push(cell);
            }
            self.color_cells();
            food.status = FoodStatus::Eaten;
        }

        let head = self.get_head_mut();
        let p = head.position + head.velocity * dt;
        let r = head.size;
        if p.x - r < 0.0 || (CANVAS_WIDTH as f32) < p.x + r { 
            head.velocity.x *= -0.9;
        }
        else if p.y - r < 0.0 || (CANVAS_HEIGHT as f32) < p.y + r {
            head.velocity.y *= -0.9;
        }
        else {
            head.position = p;
        }

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

        let hp = self.get_head().position;
        for cell in &mut self.cells[5..] {
            if (cell.position - hp).length() < cell.size {
                cell.color = Color::rgb(255, 0, 0);
                self.cells[0].color = Color::rgb(255, 0, 0);
                return GameStatus::Over;
            }
        }

        GameStatus::Playing
    }

    pub fn input(&mut self, input: Input) {
        use Input::*;
        
        if let Some(first_cell) = self.cells.get_mut(0) {
            match input {
                ArrowRight => first_cell.velocity.x += 10.0,
                ArrowUp    => first_cell.velocity.y -= 10.0,
                ArrowLeft  => first_cell.velocity.x -= 10.0,
                ArrowDown  => first_cell.velocity.y += 10.0,
                _          => {/* do nothing */},
            }
        }
    }

    pub fn render(&self) {
        for cell in self.cells.iter() {
            fill_circle(cell.position.x, cell.position.y, cell.size, cell.color);
        }
    }
}