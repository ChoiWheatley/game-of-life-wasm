mod utils;

use getrandom::*;
use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
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

                // 불필요한 if 문을 없애기 위한 조치
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        use crate::Cell::*;
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewr than two live neighbors
                    // dies, as if coused by underpopulation.
                    (Alive, x) if x < 2 => Dead,

                    // Rule 2: Any live cell with two or three live neighbors
                    // lives on to the next generation.
                    (Alive, 2) | (Alive, 3) => Alive,

                    // Rule 3: Any live cell with more than three live
                    // neighbors dies, as if by overpopulation.
                    (Alive, x) if x > 3 => Dead,

                    // Rule 4: Any dead cell with exactly three live neighbors
                    // becomes a live cell, as if by reproduction.
                    (Dead, 3) => Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                self.cells[idx] = next_cell;
            }
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn new(width: u32, height: u32) -> Self {
        const SIZE: usize = 1 << 16;
        let mut randbuf = [0; SIZE];
        getrandom(&mut randbuf).expect("getrandom failed");

        let cells = (0..width * height)
            .map(|i| {
                if (i + 1) as usize % SIZE == 0 {
                    getrandom(&mut randbuf).expect("getrandom failed");
                }
                if dbg!(randbuf[i as usize % SIZE]) & 15 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        Universe {
            width,
            height,
            cells,
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = match cell {
                    Cell::Alive => '◼',
                    Cell::Dead => '◻',
                };
                write!(f, "{symbol}")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
