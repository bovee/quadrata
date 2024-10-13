use dlx_rs::Sudoku;

use crate::Puzzle;
use crate::strategies::guess::guess;


/// This ignores if the field is actually marked as solved.
pub fn redo_guesses(board: &Puzzle) -> Vec<u16> {
    let counts = get_counts(&board, true);
    let mut new_values = vec![0; board.values.len()];
    for i in 0..board.values.len() {
        if board.solved[i] {
            new_values[i] = board.values[i];
            continue;
        }
        'value_iter: for val in 0..board.size {
            for (cons, count) in board.boxes.iter().zip(&counts) {
                if count[cons[i] - 1][val] > 0 {
                    continue 'value_iter;
                }
            }
            new_values[i] |= 1 << val;
        }
    }
    new_values
}

/// Get the number of occurances of each value in each set of constraints.
pub fn get_counts(board: &Puzzle, only_solved: bool) -> Vec<Vec<Vec<u8>>> {
    let mut counts = vec![vec![vec![0u8; board.size]; board.size]; board.boxes.len()];
    for (i, v) in board.values.iter().enumerate() {
        if only_solved {
            if !board.solved[i] {
                continue;
            }
            let val = v.trailing_zeros() as usize;
            // we loop over every set of constraints in `boxes`
            for cons in 0..board.boxes.len() {
                // if the constraint is set to 0; don't bother saving
                if board.boxes[cons][i] == 0 {
                    continue;
                }
                counts[cons][board.boxes[cons][i] - 1][val] += 1;
            }
            continue;
        }
        let n_values = v.count_ones();
        if n_values == 0 {
            continue;
        }
        for val in 0..board.size {
            if v & (1 << val) == 0 {
                continue;
            }
            for cons in 0..board.boxes.len() {
                if board.boxes[cons][i] == 0 {
                    continue;
                }
                counts[cons][board.boxes[cons][i] - 1][val] += 1;
            }
        }
    }
    counts
}

pub fn is_valid(board: &Puzzle) -> bool {
    // TODO: get rid of this function? We shouldn't be regenerating the get_counts here
    for value in &board.values {
        if *value == 0 {
            return false;
        }
    }
    let counts = get_counts(&board, true);
    for cons in counts {
        for zone in cons {
            for value in zone {
                if value > 1 {
                    return false;
                }
            }
        }
    }
    true
}


pub fn solve(board: &Puzzle) -> Result<Vec<u8>, String> {
    if board.boxes.len() != 3 {
        return Err("Only traditional sudoku can be solved".to_string());
    } else if board.size != 4 && board.size != 9 && board.size != 16 {
        return Err(format!("Board of size {} can not be solved", board.size));
    }

    let mut knowns = Vec::new();
    for i in 0..board.values.len() {
        if board.values[i].count_ones() != 1 {
            knowns.push(0);
            continue;
        }
        knowns.push(board.values[i].trailing_zeros() as usize + 1);
    }
    let sudoku = Sudoku::new_from_input(&knowns);
    let solutions: Vec<Vec<usize>> = sudoku.collect();
    if solutions.len() == 0 {
        return Err("Board has no valid solution".to_string());
    } else if solutions.len() > 1 {
        return Err("Board has multiple valid solutions".to_string());
    }
    Ok(solutions[0].iter().map(|s| *s as u8).collect())
}

// TODO: fix this up?
// pub fn solve(board: &Puzzle) -> Result<Vec<u16>, String> {
//     if !is_valid(board) {
//         return Err("Starting grid is invalid".to_string());
//     }
// 
//     let board = board.clone();
//     reset_guesses(&mut board);
//     // TODO: check if board is complete already; return if so
//     if let Some(solution) = guess(&board) {
//         for (cell, value) in solution.values {
//             board.values[cell] = value;
//         }
//     } else {
//         return Err("Error forcing puzzle solution");
//     }
//     // TODO: check if the board is valid; return error if so
//     // and retry cell above with next guess 
// 
//     
//     Ok(vec![])
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_box_counts() {
        let mut board = Puzzle::raw_from_grid(&vec![0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4]);
        board.values[4] = 5;

        let counts = get_counts(&board, true);
        assert_eq!(counts, vec![
                vec![vec![1, 0, 0, 0], vec![0, 1, 0, 0], vec![0, 0, 1, 0], vec![0, 0, 0, 1]],
                vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![1, 1, 1, 1]],
                vec![vec![0, 0, 0, 0], vec![1, 1, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 1, 1]]
        ]);
        let counts = get_counts(&board, false);
        assert_eq!(counts, vec![
                vec![vec![1, 0, 0, 0], vec![1, 1, 1, 0], vec![0, 0, 1, 0], vec![0, 0, 0, 1]],
                vec![vec![1, 0, 1, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![1, 1, 1, 1]],
                vec![vec![1, 0, 1, 0], vec![1, 1, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 1, 1]]
        ]);
    }

    #[test]
    fn test_redo_guesses() {
        let mut board = Puzzle::raw_from_grid(&vec![0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4]);
        board.values[4] = 5;
        board.values[0] = 10;

        let new_values = redo_guesses(&board);
        assert_eq!(new_values, vec![14, 14, 12, 1, 13, 13, 12, 2, 11, 11, 3, 4, 7, 7, 3, 8]);
    }

    #[test]
    fn test_solve() {
        let board = Puzzle::raw_from_grid(&vec![2, 0, 0, 0, 0, 1, 0, 2, 0, 0, 3, 0, 0, 0, 0, 4]);
        let solution = solve(&board);
        assert_eq!(solution, Ok(vec![2, 4, 1, 3, 3, 1, 4, 2, 4, 2, 3, 1, 1, 3, 2, 4]));

        let board = Puzzle::raw_from_grid(&vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let solution = solve(&board);
        assert_eq!(solution, Err("Board has multiple valid solutions".to_string()));

        // TODO: dlx_rs panics on the following
        // let board = Puzzle::from_grid(&vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0]);
        // let solution = solve(&board);
        // assert_eq!(solution, Err("Board has no valid solution".to_string()));
    }
}
