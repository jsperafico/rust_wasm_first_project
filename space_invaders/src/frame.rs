use wasm_bindgen::prelude::*;
use std::fmt;

use crate::{NUM_COLS, NUM_ROWS, player::Player, invaders::Invaders};

pub type Frame = Vec<Vec<& 'static str>>;

pub fn new_frame() -> Frame {
    let mut rows = Vec::with_capacity(NUM_ROWS);
    for _ in 0..NUM_ROWS {
        let mut cols = Vec::with_capacity(NUM_COLS);
        for _ in 0..NUM_COLS {
            cols.push(" ");
        }
        rows.push(cols);
    }
    rows
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}

#[wasm_bindgen]
pub struct Render {
    player: Player,
    invaders: Invaders,
    frame: Frame,
}

#[wasm_bindgen]
impl Render {
    pub fn new() -> Self {
        let player = Player::new();
        let invaders = Invaders::new();
        let frame = new_frame();

        Self {
            player,
            invaders,
            frame
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn detect_hits(&mut self) -> bool {
        self.player.detect_hits(&mut self.invaders)
    }

    pub fn tick(&mut self) {
        let mut next = new_frame();

        self.player.update();
        self.invaders.update();

        self.player.draw(&mut next);
        self.invaders.draw(&mut next);

        self.frame = next;
    }
}

impl fmt::Display for Render {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.frame {
            for col in line {
                write!(f, "{}", col)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{thread, time::Duration};

    #[test]
    pub fn validate_render_creation_successfully() {
        let mut r = Render::new();
        r.tick();
        r.player.shoot();
        thread::sleep(Duration::from_secs(2));
        r.tick();
        r.player.left();
        r.player.left();
        for _ in 0..5 {
            r.tick(); 
            thread::sleep(Duration::from_secs(2));
        }
        println!("{}", r.render());
    }
}