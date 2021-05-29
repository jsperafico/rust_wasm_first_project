use wasm_bindgen::prelude::*;

use crate::frame::{Drawable, Frame};

#[wasm_bindgen]
pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    pub timer: usize,
}

#[wasm_bindgen]
impl Shot {
    pub fn new(x: usize, y:usize) -> Shot {
        Shot {
            x: x,
            y: y,
            exploding: false,
            timer: 1
        }
    }

    pub fn update(&mut self, delta: usize) {
        self.y -= if !self.exploding && self.y > 0 { 1 } else { 0 };
    }

    pub fn explode(&mut self) {
        self.exploding = true;
    }

    pub fn dead(&self) -> bool {
        self.exploding || self.y == 0
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding { "*" } else { "|" };
    }
}