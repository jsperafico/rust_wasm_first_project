use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)] //make each cell represented as a byte
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}