mod io;
mod solver;
mod strategies;

pub use crate::io::from_81;

use wasm_bindgen::prelude::*;

use solver::{get_counts, redo_guesses, solve};

// TODO: delete this helper logging code at some point
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
macro_rules! console_log {
    ($($t:tt)*) => (unsafe { log(&format_args!($($t)*).to_string()) })
}

const FIXED_MASK: u8 = 0b10000000;
const COLOR_MASK: u8 = 0b00111111;

fn generate_boxes(size: usize) -> Vec<Vec<usize>> {
    // TODO: this should be able to generate colors too?
    let box_size = (size as f64).sqrt().floor() as usize;
    let mut rows = Vec::with_capacity(size);
    let mut cols = Vec::with_capacity(size);
    let mut boxes = Vec::with_capacity(size);
    for ix in 0..size * size {
        let row: usize = ix / size;
        rows.push(row + 1);
        let col: usize = ix % size;
        cols.push(col + 1);
        if size == 6  || size == 8{
            // TODO: test this
            boxes.push(2 * (row / 2) + (col / (size / 2)));
        } else {
            let box_idx: usize = box_size * (row / box_size) + (col / box_size);
            boxes.push(box_idx + 1);
        }
    }
    vec![rows, cols, boxes]
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum AutoPencil {
   Never = "never",
   OnlyRemove = "onlyremove",
   Always = "always",
   Snyder = "synder",
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Puzzle {
    /// The candidates and solutions for the grid
    values: Vec<u16>,
    /// How many rows or cols the grid has.
    pub size: usize,
    /// Cached values for the rows, cols, and boxes.
    boxes: Vec<Vec<usize>>,
    /// Stores in a bit array if values are fixed (preset) and their color.
    types: Vec<u8>,
    /// The true values of the cells.
    truths: Vec<u8>,
    /// If each cell is "solved".
    solved: Vec<bool>,
    /// All the previous board states (values and solved)
    history: Vec<(usize, u16, bool)>,
}

#[wasm_bindgen]
impl Puzzle {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Puzzle {
        let size = 9;
        Puzzle {
            size,
            boxes: generate_boxes(size),
            types: vec![0; size * size],
            truths: vec![0; size * size],
            values: vec![0; size * size],
            solved: vec![false; size * size],
            history: Vec::new(),
        }
    }

    pub fn raw_from_grid(grid: &[u8]) -> Puzzle {
        let size = (grid.len() as f64).sqrt().floor() as usize;
        let mut types = Vec::with_capacity(grid.len());
        let mut values = Vec::with_capacity(grid.len());
        let mut solved = Vec::with_capacity(grid.len());
        for v in grid.iter() {
            if *v == 0 {
                types.push(0);
                values.push(0);
                solved.push(false);
            } else {
                types.push(FIXED_MASK);
                values.push(1 << (*v - 1));
                solved.push(true);
            }
        }
        Puzzle {
            size,
            boxes: generate_boxes(size),
            types,
            truths: grid.to_vec(),
            values,
            solved,
            history: Vec::new(),
        }
    }

    pub fn from_grid(grid: &[u8]) -> Result<Puzzle, String> {
        let mut puzzle = Self::raw_from_grid(grid);
        puzzle.truths = solve(&puzzle)?;
        Ok(puzzle)
    }

    pub fn to_grid(&self) -> Vec<u8> {
        let mut grid = Vec::new();
        for (v, s) in self.values.iter().zip(&self.solved) {
            if *s {
                grid.push(v.trailing_zeros() as u8 + 1);
            } else {
                grid.push(0);
            }
        }
        grid
    }

    pub fn set_value(&mut self, idx: usize, value: u8) {
        if self.types[idx] & FIXED_MASK != 0 {
            // don't allow if this cell is fixed
            return;
        }
        if value == 0 {
            if self.values[idx] != 0 && self.history.last().map(|v| v.0) == Some(idx) {
                self.undo();
            } else {
                self.history.push((idx, self.values[idx], self.solved[idx]));
                self.values[idx] = 0;
                self.solved[idx] = false;
            }
        } else {
            self.history.push((idx, self.values[idx], self.solved[idx]));
            self.values[idx] = 1 << (value - 1);
            self.solved[idx] = true;
        }
    }

    pub fn undo(&mut self) {
        if let Some((idx, val, solved)) = self.history.pop() {
            self.values[idx] = val;
            self.solved[idx] = solved;
        }
    }

    #[wasm_bindgen(getter)]
    pub fn fixed_values(&self) -> Vec<JsValue> {
        self.types
            .iter()
            .map(|x| JsValue::from_bool(x & FIXED_MASK == FIXED_MASK))
            .collect()
    }

    #[wasm_bindgen(getter)]
    pub fn values(&self) -> Vec<u8> {
        self.values
            .iter()
            .zip(&self.solved)
            .map(|(v, s)| {
                if !s {
                    return 0;
                }
                v.trailing_zeros() as u8 + 1
            })
            .collect()
    }

    pub fn erase_guess(&mut self, idx: usize) {
        if self.types[idx] & FIXED_MASK != 0 {
            // don't allow if this cell is fixed
            return;
        }
        if !self.solved[idx] {
            self.history.push((idx, self.values[idx], self.solved[idx]));
            self.values[idx] = 0;
        }
    }

    pub fn set_guess(&mut self, idx: usize, guess: u8) {
        if self.types[idx] & FIXED_MASK != 0 {
            // don't allow if this cell is fixed
            return;
        }
        if !self.solved[idx] {
            self.history.push((idx, self.values[idx], self.solved[idx]));
            self.values[idx] ^= 1 << (guess - 1);
        }
    }

    #[wasm_bindgen(getter)]
    pub fn guesses(&self) -> Vec<String> {
        let mut str_guesses: Vec<String> = Vec::new();
        for v in &self.values {
            let mut str_guess = String::new();
            for i in 0..self.size {
                if 1 & (v >> i) == 1 {
                    str_guess.push_str(&format!("{:X}", i + 1));
                }
            }
            str_guesses.push(str_guess);
        }
        str_guesses
    }

    pub fn verify(&self, strict: bool) -> Vec<usize> {
        let mut bad_cells = Vec::new();
        if strict {
            for i in (0..self.values.len()).filter(|i| self.solved[*i]) {
                if self.truths[i] != 0 && self.truths[i] != self.values[i].trailing_zeros() as u8 + 1 {
                    bad_cells.push(i);
                }
            }
            return bad_cells;
        }

        let counts = get_counts(&self, true);
        for (i, v) in (0..self.values.len()).filter(|i| self.solved[*i]).map(|i| (i, self.values[i].trailing_zeros())) {
            let mut loop_cyc = 0;
            for (cons, count) in self.boxes.iter().zip(&counts) {
                loop_cyc += 1;
                if count[cons[i] - 1][v as usize] > 1 {
                    bad_cells.push(i);
                    break;
                }
            }
        }
        bad_cells
    }

    pub fn update_guesses(&mut self, style: AutoPencil) {
        let updated_guesses = redo_guesses(&self);
        match style {
            AutoPencil::Always => {
                self.values = updated_guesses;
                return;
            },
            AutoPencil::Snyder => {
                // TODO: it would be nice if this could take any user edited guesses
                // into account when updating
                let mut box_guess_counts = vec![vec![0u8; self.size]; self.size];
                for i in 0..self.values.len() {
                    if self.solved[i] {
                        continue;
                    }
                    for val in 0..self.size {
                        if (updated_guesses[i] >> val) & 1 == 1 {
                            let box_idx = self.boxes.get(2).map(|b| b[i] - 1).unwrap_or(0);
                            box_guess_counts[box_idx][val] += 1;
                        }
                    }
                }
                for i in 0..self.values.len() {
                    if self.solved[i] {
                        continue;
                    }
                    let box_idx = self.boxes.get(2).map(|b| b[i] - 1).unwrap_or(0);
                    let mut new_value = 0;
                    for val in 0..self.size {
                        let val_box_counts = box_guess_counts[box_idx][val];
                        if (updated_guesses[i] >> val) & 1 == 1 && val_box_counts > 0 && val_box_counts <= 2 {
                            new_value |= 1 << val;
                        }
                    }
                    self.values[i] = new_value;
                }
            },
            AutoPencil::Always => {
                for i in 0..self.values.len() {
                    self.values[i] &= updated_guesses[i];
                }
            }
            _ => {},
        }
    }

    // pub fn update_guesses(&mut self, overwrite: bool, snyder: bool) {
    //     let counts = get_counts(&self, true);

    //     let mut box_guess_counts = vec![vec![0u8; self.size]; self.size];

    //     for i in 0..self.values.len() {
    //         if self.solved[i] {
    //             continue;
    //         }
    //         if overwrite {
    //             self.values[i] = 0;
    //         }
    //         for val in 0..self.size {
    //             let mut total = 0;
    //             for (cons, count) in self.boxes.iter().zip(&counts) {
    //                 total += count[cons[i] - 1][val];
    //             }
    //             if overwrite && total == 0 {
    //                 self.values[i] |= 1 << val;
    //                 let box_idx = self.boxes.get(3).map(|b| b[i] - 1).unwrap_or(0);
    //                 box_guess_counts[box_idx][val] += 1;
    //             }
    //             if total > 0 {
    //                 self.values[i] &= !(1 << val);
    //             }
    //         }
    //     }

    //     if snyder {
    //         for i in 0..self.values.len() {
    //             if self.solved[i] {
    //                 continue;
    //             }
    //             let box_idx = self.boxes.get(3).map(|b| b[i] - 1).unwrap_or(0);
    //             for val in 0..self.size {
    //                 if box_guess_counts[box_idx][val] > 2 {
    //                     self.values[i] &= !(1 << val);
    //                 }
    //             }
    //         }
    //     }
    // }

    pub fn is_complete(&self) -> bool {
        for i in 0..self.values.len() {
            if !self.solved[i] || self.truths[i] != self.values[i].trailing_zeros() as u8 + 1 {
                return false;
            }
        }
        true
    }
}

#[wasm_bindgen]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_and_from_grid() {
        let grid = vec![0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4];
        let puzzle = Puzzle::raw_from_grid(&grid);
        let new_grid = puzzle.to_grid();
        assert_eq!(grid, new_grid);
    }
}
