use std::fmt;



pub struct Slice {
    n: usize,
    cells: Vec<bool>,
    // orphan: bool, // this state could not be derived from a previous state (i.e. it must have been randomly generated or manually set)
}

impl Slice {
    pub fn new(n: usize) -> Slice {
        let cells = vec![false; n * n];
        Slice {
            n,
            cells,
            // orphan: false,
        }
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

                if self.cells[p] {
                    // Any live cell with fewer than two live neighbors dies
                    // as if by underpopulation.
                    if live_neighbors < 2 {
                        next_cells[p] = false;
                    }
                    // Any live cell with two or three live neighbors lives
                    // on to the next generation.
                    else if live_neighbors == 2 || live_neighbors == 3 {
                        next_cells[p] = true;
                    }
                    // Any live cell with more than three live neighbors dies
                    // as if by overpopulation.
                    else {
                        next_cells[p] = false;
                    }
                } else {
                    // Any dead cell with exactly three live neighbors becomes
                    // a live cell, as if by reproduction.
                    if live_neighbors == 3 {
                        next_cells[p] = true;
                    }
                }
            }
        }

        self.cells = next_cells;
    }

    #[allow(dead_code)]
    pub fn next_generation_naive_optimized(&mut self) {
        let mut next_cells = vec![false; self.n * self.n];  
        let alive_indices = self.cells.iter().enumerate().filter(|(_, &alive)| alive).map(|(i, _)| i).collect::<Vec<_>>();

        for &index in &alive_indices {
            let live_neighbors = self.count_live_neighbors(index);

            // Any live cell with fewer than two live neighbors dies
            // as if by underpopulation.
            if live_neighbors < 2 {
                next_cells[index] = false;
            }
            // Any live cell with two or three live neighbors lives
            // on to the next generation.
            else if live_neighbors == 2 || live_neighbors == 3 {
                next_cells[index] = true;
            }
            // Any live cell with more than three live neighbors dies
            // as if by overpopulation.
            else {
                next_cells[index] = false;
            }

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
                if p >= self.cells.len() {
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


// struct MemoizedSlice {
//     slice: Slice,
//     memo: HashMap<(usize, usize), Slice>,
// }

// impl MemoizedSlice {
//     pub fn new(width: usize, height: usize) -> MemoizedSlice {
//         let slice = Slice::new(width, height);
//         let memo = HashMap::new();
//         MemoizedSlice { slice, memo }
//     }

//     pub fn next_generation(&mut self) {
//         let sub_slice_width = self.slice.width.saturating_sub(2);
//         let sub_slice_height = self.slice.height.saturating_sub(2);

//         for row in 0..sub_slice_height {
//             for col in 0..sub_slice_width {
//                 let sub_slice = self.slice.get_sub_slice(row, col);
//                 let transition = self.memo.entry((row, col)).or_insert_with(|| {
//                     let mut next_slice = sub_slice.clone();
//                     next_slice.next_generation();
//                     next_slice
//                 });

//                 self.slice.set_sub_slice(row, col, transition.clone());
//             }
//         }

//         self.slice.next_generation();
//     }
// }
