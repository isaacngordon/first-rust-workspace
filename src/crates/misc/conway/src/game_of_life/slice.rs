use std::fmt;

/// Represents a universe slice of cells in Conway's Game of Life
#[derive(Clone, Debug)]
pub struct Slice {
    /// One side of the square grid. i.e. n = 3 means a 3x3 grid
    n: usize,
    /// The cells in the slice, stored in row-major order
    cells: Vec<bool>,
}

impl Slice {
    pub fn new(n: usize) -> Slice {
        let cells = vec![false; n * n];
        Slice {
            n,
            cells,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> bool {
        let p = row * self.n + col;
        self.cells[p].clone()
    }

    pub fn get_size(&self) -> usize {
        self.n.clone()
    }

    pub fn set_cells(&mut self, cells: Vec<bool>) {
        if cells.len() != (self.n * self.n) {
            panic!("Invalid cell count");
        }
        self.cells = cells;
    }

    pub fn set_cell(&mut self, row: usize, col: usize, value: bool) {
        let p = row * self.n + col;
        self.cells[p] = value;
    }

    pub fn randomize(&mut self) {
        for cell in &mut self.cells {
            *cell = rand::random();
        }
    }

    pub fn next_generation_naive(&mut self) {
        let mut next_cells = vec![false; self.n * self.n];

        for row in 0..self.n {
            for col in 0..self.n {
                let p = row * self.n + col;
                let live_neighbors = self.count_live_neighbors(p);

                next_cells[p] = match (self.cells[p], live_neighbors) {
                    (true, 0..=1) => false, // Any live cell with fewer than two live neighbors dies as if by underpopulation.
                    (true, 2..=3) => true, // Any live cell with two or three live neighbors lives on to the next generation.
                    (true, _) => false, // Any live cell with more than three live neighbors dies as if by overpopulation.
                    (false, 3) => true, // Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
                    (false, _) => self.cells[p], // Otherwise, the cell remains unchanged.
                };
            }
        }

        self.cells = next_cells;
    }

    pub fn next_generation_naive_optimized(&mut self) {
        let mut next_cells = vec![false; self.n * self.n];
        let mut queue = Vec::new();
        let mut visited = vec![false; self.n * self.n];

        // Initialize queue with alive cells and their neighbors
        for i in 0..self.cells.len() {
            if self.cells[i] {
                queue.push(i);
                visited[i] = true;

                let row = i / self.n;
                let col = i % self.n;

                for r in row.saturating_sub(1)..=row.saturating_add(1) {
                    for c in col.saturating_sub(1)..=col.saturating_add(1) {
                        let index = r * self.n + c;
                        if index < self.cells.len() && !visited[index] {
                            queue.push(index);
                            visited[index] = true;
                        }
                    }
                }
            }
        }

        // Process each cell in the queue
        for &index in &queue {
            let live_neighbors = self.count_live_neighbors(index);
            next_cells[index] = match (self.cells[index], live_neighbors) {
                (true, 0..=1) | (true, 4..=usize::MAX) => false,
                (true, 2..=3) => true,
                (false, 3) => true,
                _ => false,
            };
        }

        self.cells = next_cells;
    }

    fn count_live_neighbors(&self, index: usize) -> usize {
        let mut count = 0;

        let row = index / self.n;
        let col = index % self.n;

        // Iterate over the 3x3 grid centered on the cell
        for i in row.saturating_sub(1)..=row.saturating_add(1) {
            for j in col.saturating_sub(1)..=col.saturating_add(1) {
                // Skip the center cell
                if i == row && j == col {
                    continue;
                }

                let p = i * self.n + j;

                // Skip cells outside the grid
                if p >= self.cells.len() || i >= self.n || j >= self.n {
                    continue;
                }

                if self.cells[p] {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn to_hex_string(&self) -> String {
        let mut s = String::new();

        //get 4 bits at a time
        for i in 0..self.cells.len() / 4 {
            let mut val = 0;
            for j in 0..4 {
                val += (self.cells[i * 4 + j] as u8) << j;
            }
            s.push_str(&format!("{:x}", val));

            if i % 4 == 3 {
                s.push(' ');
            }
        }

        s
    }
}

impl fmt::Display for Slice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.n {
            for col in 0..self.n {
                let p = row * self.n + col;
                if self.cells[p] {
                    write!(f, "█")?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl PartialEq for Slice {
    fn eq(&self, other: &Self) -> bool {
        self.cells == other.cells
    }
}

