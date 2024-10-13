use wasm_bindgen::prelude::*;

use crate::Puzzle;

#[wasm_bindgen]
pub fn from_81(text: &str) -> Result<Puzzle, JsError> {
    if text.len() != 81 {
        return Err(JsError::new("text must be 81 characters long"));
    }
    let mut truths: Vec<u8> = Vec::new();
    for c in text.chars() {
        match c {
            '0' | '.' | ' ' | 'X' | 'x' => truths.push(0),
            '1'..='9' => truths.push(str::parse::<u8>(&c.to_string())?),
            _ => {
                return Err(JsError::new(&format!("character {} can not be present", c)));
            }
        }
    }
    Puzzle::from_grid(&truths).map_err(|s| JsError::new(&s))
}
