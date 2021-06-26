use std::time::Duration;
use wasm_timer::Instant;

use crate::frame::{Drawable, Frame};

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    duration: Duration,
    time: Instant,
}

impl Shot {
    pub fn new(x: usize, y:usize) -> Shot {
        Shot {
            x: x,
            y: y,
            exploding: false,
            duration: Duration::from_millis(50),
            time: Instant::now(),
        }
    }

    pub fn explode(&mut self) {
        self.exploding = true;
        self.duration = Duration::from_millis(250);
    }

    pub fn dead(&self) -> bool {
        (self.exploding && self.time.elapsed() >= self.duration) || self.x == 0
    }
    
    pub fn update(&mut self) {
        if self.time.elapsed() >= self.duration && !self.exploding {
            self.x -= if self.x > 0 { 1 } else { 0 };
            self.time = Instant::now();
        }
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding { "*" } else { "|" };
    }
}