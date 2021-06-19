use wasm_bindgen::prelude::*;
use std::time::Instant;

use crate::{NUM_COLS, NUM_ROWS, player::Player, invaders::Invaders};

pub type Frame = Vec<Vec<& 'static str>>;

pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS {
            col.push(" ");
        }
        cols.push(col);
    }
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}

#[wasm_bindgen]
pub struct Render {
    player: Player,
    invaders: Invaders,
    instant: Instant,
    frame: Frame,
}

#[wasm_bindgen]
impl Render {
    pub fn new() -> Self {
        let player = Player::new();
        let invaders = Invaders::new();
        let instant = Instant::now();
        let frame = new_frame();

        Self {
            player,
            invaders,
            instant,
            frame
        }
    }

    pub fn detect_hits(&mut self) -> bool {
        self.player.detect_hits(&mut self.invaders)
    }

    pub fn tick(&mut self) {
        let delta = self.instant.elapsed();
        self.instant = Instant::now();

        let mut next = new_frame();

        self.player.update(delta);
        self.invaders.update(delta);

        self.player.draw(&mut next);
        self.invaders.draw(&mut next);

        self.frame = next;
    }
}