use wasm_bindgen::prelude::wasm_bindgen;
use js_sys::Math;
use std::fmt;
use std::fmt::Formatter;
use pythagore::{force, self as py};
use pythagore_wasm::force_2d::Force2D;
use crate::cell::Cell;

#[wasm_bindgen]
pub struct Universe {
    size: py::Force2D<u32>,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.size.dx() + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        for delta_row in [self.size.dy() - 1, 0, 1].iter().cloned() {
            for delta_col in [self.size.dx() - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.size.dy();
                let neighbor_col = (column + delta_col) % self.size.dy();
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }

        count
    }
}

// Public methods
#[wasm_bindgen]
impl Universe {
    pub fn dead(width: u32, height: u32) -> Universe {
        let cells = (0..width * height)
            .map(|_i| Cell::Dead)
            .collect();

        Universe {
            size: force![width, height],
            cells,
        }
    }

    pub fn random(width: u32, height: u32) -> Universe {
        let cells = (0..width * height)
            .map(|_i| {
                let rand = Math::random();

                if rand < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            size: force![width, height],
            cells
        }
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..*self.size.dy() {
            for col in 0..*self.size.dx() {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn size(&self) -> Force2D {
        force![*self.size.dx() as f64, *self.size.dy() as f64].into()
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(*self.size.dx() as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}
