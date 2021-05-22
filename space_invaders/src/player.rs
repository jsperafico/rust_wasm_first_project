use wasm_bindgen::prelude::*;

use crate::shot::Shot;

#[wasm_bindgen]
pub struct Player {
    pub x: usize,
    pub y: usize,
    shot: Shot,
}

#[wasm_bindgen]
impl Player {
    pub fn new() -> Player {
        Player {
            x: 10,
            y: 20,
            shot: Shot::new()
        }
    }
}
