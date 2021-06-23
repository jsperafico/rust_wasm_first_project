use wasm_bindgen::prelude::*;
use std::{cmp::max, time::Duration};
use wasm_timer::Instant;

use crate::{NUM_COLS, NUM_ROWS, frame::{Frame, Drawable}};

#[wasm_bindgen]
struct Invader {
    pub x: usize,
    pub y: usize,
}

#[wasm_bindgen]
pub struct Invaders {
    army: Vec<Invader>,
    duration: Duration,
    time: Instant,
    direction: i32,
}

#[wasm_bindgen]
impl Invaders {
    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }

    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.x).max().unwrap_or(0) >= NUM_ROWS - 3
    }
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..NUM_ROWS {
            for y in 0..NUM_COLS {
                if x > 0 && x < 6
                && y > 2 && y < NUM_COLS - 2
                && x % 2 == 1 && y % 2 == 0 {
                    army.push(Invader {x, y});
                }
            }
        }
        Self {
            army,
            duration: Duration::from_secs(2),
            time: Instant::now(),
            direction: 1,
        }
    }

    pub fn update(&mut self) -> bool {

        if self.time.elapsed() >= self.duration {
            let mut downwards = false;
            if self.direction == -1 {
                let min = self.army.iter().map(|invader| invader.y).min().unwrap_or(0);
                if min == 0 {
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let max = self.army.iter().map(|invader| invader.y).max().unwrap_or(0);
                if max == NUM_COLS - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }

            for invader in self.army.iter_mut() {
                if downwards {
                    let new_duration = max(self.time.elapsed().as_millis() - 250, 250);
                    self.duration = Duration::from_millis(new_duration as u64);
                    invader.x += 1;
                } else {
                    if self.duration.as_secs() != 2 {
                        self.duration = Duration::from_secs(2);
                    }
                    invader.y = ((invader.y as i32) + self.direction) as usize;
                }
            }

            self.time = Instant::now();

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
        let remaining = self.duration - self.time.elapsed();
        for invader in self.army.iter() {
            frame[invader.x][invader.y] = if remaining.as_secs_f32() > 0.5 {
                "x"
            } else {
                "+"
            };
        }
    }
}