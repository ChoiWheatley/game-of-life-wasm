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
        for delta_row in [self.height - 1, 0, 1].iter() {
            for delta_col in [self.width - 1, 0, 1].iter() {
                if delta_row == &0 && delta_col == &0 {
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

                self.cells.set_to(idx, next_cell);
            }
        }
    }

    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        let mut cells = Bitset::with_size(size);
        for i in 0..size {
            cells.set_to(i, js_sys::Math::random() < 0.04);
        }
        Universe {
            width,
            height,
            cells,
        }
    }
}
