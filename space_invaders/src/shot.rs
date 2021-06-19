use wasm_bindgen::prelude::*;
use std::time::Duration;
use rusty_time::prelude::Timer;

use crate::frame::{Drawable, Frame};

#[wasm_bindgen]
pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    timer: Timer,
}

#[wasm_bindgen]
impl Shot {
    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::from_millis(250);
    }

    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.ready) || self.y == 0
    }
}

impl Shot {
    pub fn new(x: usize, y:usize) -> Shot {
        Shot {
            x: x,
            y: y,
            exploding: false,
            timer: Timer::from_millis(50),
        }
    }
    
    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready && !self.exploding {
            self.y -= if self.y > 0 { 1 } else { 0 };
            self.timer.reset();
        }
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding { "*" } else { "|" };
    }
}