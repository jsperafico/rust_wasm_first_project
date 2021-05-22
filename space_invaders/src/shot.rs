use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    pub timer: usize,
}

#[wasm_bindgen]
impl Shot {
    pub fn new() -> Shot {
        Shot {
            x: 5,
            y: 50,
            exploding: false,
            timer: 1
        }
    }
}