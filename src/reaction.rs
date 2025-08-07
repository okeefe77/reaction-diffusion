use anyhow::{ ensure, Result };

#[derive(Copy, Clone, Debug)]
struct RDCell {
    a: f32,
    b: f32
}

struct RDGrid {
    data: Vec<RDCell>,
    width: u32,
    height: u32
}

impl RDGrid {
    fn new(width: u32, height: u32) -> Self {
        let data = vec![RDCell { a: 1.0, b: 0.0 }; (width * height) as usize];
        Self { data, width, height }
    }

    fn get_cell(&self, x: u32, y: u32) -> Option<&RDCell> {
       let idx = self.index_for(x, y);
       self.data.get(idx)
    }

    fn set_cell(&mut self, x: u32, y: u32, a: f32, b: f32) {
        let idx = self.index_for(x, y);

        if let Some(cell) = self.data.get_mut(idx) {
            cell.a = a;
            cell.b = b;
        }
    }

    fn convolve_cell(&self, x: u32, y: u32) -> Result<(f32, f32)> {
        ensure!(x != 0 && x < self.width - 1 && y != 0 && y < self.height - 1, "Cannot convolve edge cells");

        let mut new_a = 0.0;
        let mut new_b = 0.0;
        let mut k_idx = 0;
        let kernel = [
            0.05, 0.2, 0.05,
            0.2, -1.0, 0.2, 
            0.05, 0.2, 0.05];

        for v in -1..=1 {
            for h in -1..=1 {
                let x_coord = (x as i32 + h) as u32;
                let y_coord = (y as i32 + v) as u32;

                if let Some(RDCell { a, b }) = self.get_cell(x_coord, y_coord) {
                    new_a += a * kernel[k_idx];
                    new_b += b * kernel[k_idx];
                }        

                k_idx += 1;
            }
        }

        Ok((new_a, new_b))
    }

    fn index_for(&self, x: u32, y: u32) -> usize {
       (y * self.width + x) as usize
    }
}


pub struct Reaction {
    grid: RDGrid,
    a_rate: f32,
    b_rate: f32,
    feed: f32,
    kill: f32
}

impl Reaction {
    pub fn new(width: u32, height: u32, a_rate: f32, b_rate: f32, feed: f32, kill: f32) -> Self {
        let grid = RDGrid::new(width, height);

        Reaction {
            grid,
            a_rate,
            b_rate,
            feed,
            kill }
    }

    pub fn step(&mut self) {
        let mut new_grid: Vec<RDCell> = Vec::new();

        for (idx, cell) in self.grid.data.iter().enumerate() {
            let x = idx as u32 % self.grid.width;
            let y = idx as u32 / self.grid.width;

            if let Ok((lap_a, lap_b)) = self.grid.convolve_cell(x, y) {
                let new_a = cell.a + (
                    (self.a_rate * lap_a) -
                    (cell.a * cell.b * cell.b) +
                    self.feed * (1.0 - cell.a)
                );

                let new_b = cell.b + (
                    (self.b_rate * lap_b) +
                    (cell.a * cell.b * cell.b) -
                    cell.b * (self.kill + self.feed)
                );

                new_grid.push(RDCell { a: new_a, b: new_b });
            } else {
                new_grid.push(*cell);
            }
        }

        self.grid.data = new_grid;
    }

    pub fn seed(&mut self, x: u32, y: u32) {
        let a = self.grid.get_cell(x, y).unwrap().a;
        self.grid.set_cell(x, y, a, 1.0);
    }

    pub fn sample_cell(&self, x: u32, y: u32) -> (f32, f32) {
        let c = self.grid.get_cell(x, y).unwrap();
        (c.a, c.b)
    }
}


