use crate::Puzzle;
use crate::solver::get_counts;
use crate::strategies::{Solution, Strategy};


pub fn hidden_single(board: &Puzzle) -> Option<Solution> {
    let counts = get_counts(&board, false);
    for i in 0..board.values.len() {
        let n_guesses = board.values[i].count_ones();
        if n_guesses <= 1 {
            // skip empty cell too even though we should probably error
            continue;
        }
        for cons in 0..board.boxes.len() {
            if board.boxes[cons][i] == 0 {
                continue;
            }
            for val in 0..board.size {
                if board.values[i] & (1 << val) == 0 {
                    continue;
                }
                if counts[cons][board.boxes[cons][i] - 1][val] == 1 {
                    return Some(Solution {
                        values: vec![(i, 1 << val)],
                        strategy: Strategy::HiddenSingle,
                        guide_cells: vec![],
                    });
                }
            }
        }
    }
    None
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::redo_guesses;

    #[test]
    fn test_hidden_single() {
        let mut board = Puzzle::raw_from_grid(&vec![0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4]);
        board.values = redo_guesses(&board);
        let solution = hidden_single(&board);
        assert_eq!(solution, None);

        let mut board = Puzzle::raw_from_grid(&vec![0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0]);
        board.values = redo_guesses(&board);
        let solution = hidden_single(&board);
        assert_eq!(solution, Some(Solution {
            values: vec![(12, 1)],
            strategy: Strategy::HiddenSingle,
            guide_cells: vec![],
        }));
    }
}
