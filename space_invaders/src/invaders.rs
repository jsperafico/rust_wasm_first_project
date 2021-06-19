use wasm_bindgen::prelude::*;
use std::{cmp::max, time::Duration};
use rusty_time::prelude::Timer;

use crate::{NUM_COLS, NUM_ROWS, frame::{Frame, Drawable}};

#[wasm_bindgen]
struct Invader {
    pub x: usize,
    pub y: usize,
}

#[wasm_bindgen]
pub struct Invaders {
    army: Vec<Invader>,
    timer: Timer,
    direction: i32,
}

#[wasm_bindgen]
impl Invaders {
    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }

    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.y).max().unwrap_or(0) >= NUM_ROWS - 2
    }
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if x > 1 && x < NUM_COLS
                && y > 0 && y < 9
                && x % 2 == 0 && y % 2 == 0 {
                    army.push(Invader {x, y});
                }
            }
        }
        Self {
            army,
            timer: Timer::from_millis(2000),
            direction: 1,
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.timer.update(delta);
        
        if self.timer.ready {
            self.timer.reset();

            let mut downwards = false;
            if self.direction == -1 {
                let min = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min == 0 {
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let max = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max == NUM_COLS - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }

            
            for invader in self.army.iter_mut() {
                if downwards {
                    let new_duration = max(self.timer.duration.as_millis() - 250, 250);
                    self.timer = Timer::from_millis(new_duration as u64);
                    invader.y += 1;
                } else {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }

            true
        } else {
            false
        }
    }

    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> bool {
        if let Some(idx) = self.army.iter().position(|invader| invader.x == x && invader.y == y) {
            self.army.remove(idx);
            true
        } else {
            false
        }
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            frame[invader.x][invader.y] = if self.timer.time_left.as_secs_f32() / self.timer.duration.as_secs_f32() > 0.5 {
                "x"
            } else {
                "+"
            };
        }
    }
}