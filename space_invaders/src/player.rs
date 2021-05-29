use wasm_bindgen::prelude::*;

use crate::{NUM_COLS, NUM_ROWS, frame::{Drawable, Frame}, shot::Shot, invader::Invaders};

#[wasm_bindgen]
pub struct Player {
    pub x: usize,
    pub y: usize,
    shots: Vec<Shot>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: Vec::new(),
        }
    }

    pub fn left(&mut self) {
        self.x -= if self.x > 0 { 1 } else { 0 }
    }

    pub fn right(&mut self) {
        self.x += if self.x < NUM_COLS { 1 } else { 0 }
    }

    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 2 {
            self.shots.push(Shot::new(self.x, self.y - 2));
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta: usize) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
    }

    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool {
        let mut hit_something = false;
        for shot in self.shots.iter_mut() {
            if !shot.exploding && invaders.kill_invader_at(shot.x, shot.y) {
                hit_something = true;
                shot.explode()
            }
        }
        hit_something
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}