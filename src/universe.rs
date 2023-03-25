use crate::bitset::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(PartialEq, Debug)]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Bitset,
}

impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        ((row * self.width) + col) as usize
    }

    /// count how many of its neighbors are alive
    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += if self.cells.get(idx) { 1 } else { 0 };
            }
        }
        count
    }
}

/// getter, setter
#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_ptr()
    }

    pub fn set_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells.set(idx);
    }
}

// internal setter
impl Universe {
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx);
        }
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut tmp_cell = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells.get(idx);
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewr than two live neighbors
                    // dies, as if coused by underpopulation.
                    (true, x) if x < 2 => false,

                    // Rule 2: Any live cell with two or three live neighbors
                    // lives on to the next generation.
                    (true, 2) | (true, 3) => true,

                    // Rule 3: Any live cell with more than three live
                    // neighbors dies, as if by overpopulation.
                    (true, x) if x > 3 => false,

                    // Rule 4: Any dead cell with exactly three live neighbors
                    // becomes a live cell, as if by reproduction.
                    (false, 3) => true,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                // replace temporally
                tmp_cell.set_to(idx, next_cell);
            }
        }
        // finally apply result cells
        self.cells = tmp_cell;
    }

    pub fn with_random_start(width: u32, height: u32, density: f64) -> Self {
        let size = (width * height) as usize;
        let mut cells = Bitset::with_size(size);
        for i in 0..size {
            cells.set_to(i, js_sys::Math::random() < density);
        }
        Universe {
            width,
            height,
            cells,
        }
    }

    /// create empty universe
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        let cells = Bitset::with_size(size);
        Universe {
            width,
            height,
            cells,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_live_neighbor() {
        const W: u32 = 4;
        const H: u32 = 4;
        let mut u = Universe::new(W, H);
        u.set_cell(0, 0);
        let answer = [[0, 1, 0, 1], [1, 1, 0, 1], [0, 0, 0, 0], [1, 1, 0, 1]];
        for (i, line) in answer.iter().enumerate() {
            for (j, elem) in line.iter().enumerate() {
                assert_eq!(
                    *elem,
                    u.live_neighbor_count(i as u32, j as u32),
                    "in index ({}, {})",
                    i,
                    j
                );
            }
        }
    }
}
