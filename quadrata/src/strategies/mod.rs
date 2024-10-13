pub mod guess;
pub mod hidden_single;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Strategy {
    HiddenSingle,
    Guess,
}

#[derive(Debug, PartialEq)]
pub struct Solution {
    /// A list of cells and their updated guesses.
    values: Vec<(usize, u16)>,
    /// Which strategy was used to derive the solution.
    strategy: Strategy,
    /// Which cells guided the solution.
    guide_cells: Vec<usize>,
}
