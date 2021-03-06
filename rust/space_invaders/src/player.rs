use crate::{NUM_COLS, NUM_ROWS, frame::{Drawable, Frame}, shot::Shot, invaders::Invaders};

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

impl Player {

    pub fn new() -> Self {
        Self {
            x: NUM_ROWS - 2,
            y: NUM_COLS / 2,
            shots: Vec::new(),
        }
    }

    pub fn left(&mut self) {
        self.y -= if self.y > 0 { 1 } else { 0 }
    }

    pub fn right(&mut self) {
        self.y += if self.y < NUM_COLS - 1 { 1 } else { 0 }
    }

    pub fn shoot(&mut self) {
        if self.shots.len() < 2 {
            self.shots.push(Shot::new(self.x - 2, self.y));
        }
    }

    pub fn update(&mut self) {
        for shot in self.shots.iter_mut() {
            shot.update();
        }
        self.shots.retain(|shot| !shot.dead());
    }

    pub fn detect_hits(&mut self, invaders: &mut Invaders) {
        for shot in self.shots.iter_mut() {
            if !shot.exploding && invaders.kill_invader_at(shot.x, shot.y) {
                shot.explode()
            }
        }
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