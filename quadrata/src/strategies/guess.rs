use crate::Puzzle;
use crate::strategies::{Solution, Strategy};


pub fn guess(board: &Puzzle) -> Option<Solution> {
    let mut easiest_cell = 0;
    let mut n_values_min = 10;
    for i in 0..board.values.len() {
        let n_values = board.values[i].count_ones();
        if n_values == 1 {
            continue;
        }
        if n_values == 2 {
            // can't go lower than 2
            return Some(Solution {
                values: vec![(i, 1 << board.values[i].trailing_zeros())],
                strategy: Strategy::Guess,
                guide_cells: vec![],
            })
        }
        if n_values < n_values_min {
            easiest_cell = i;
            n_values_min = n_values;
        }
    }
    if n_values_min == 10 {
        return None;
    }
    Some(Solution {
        values: vec![(easiest_cell, 1 << board.values[easiest_cell].trailing_zeros())],
        strategy: Strategy::Guess,
        guide_cells: vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::redo_guesses;

    #[test]
    fn test_guess() {
        let mut board = Puzzle::raw_from_grid(&vec![0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4]);
        board.values = redo_guesses(&board);

        let solution = guess(&board);
        assert_eq!(solution, Some(Solution {
            values: vec![(2, 4)],
            strategy: Strategy::Guess,
            guide_cells: vec![],
        }));
    }
}
