use wasm_bindgen::prelude::*;

use crate::frame::{Render, Drawable, new_frame};
use crate::{player::Player, invaders::Invaders};

#[wasm_bindgen]
pub struct Engine {
    player: Player,
    invaders: Invaders,
    render: Render,
}

#[wasm_bindgen]
impl Engine {
    pub fn new() -> Self {
        let player = Player::new();
        let invaders = Invaders::new();
        let render = Render::new();

        Self {
            player,
            invaders,
            render
        }
    }

    pub fn input(&mut self, action: Action) {
        match action {
            Action::LEFT => self.player.left(),
            Action::RIGHT => self.player.right(),
            Action::SHOOT => self.player.shoot(),
        };
    }

    pub fn is_endgame(&mut self) -> Result {
        if self.invaders.all_killed() {
            Result::WIN
        } else if self.invaders.reached_bottom() {
            Result::LOSE
        } else {
            Result::NONE
        }
    }

    pub fn tick(&mut self) {
        let mut next = new_frame();

        self.player.update();
        self.invaders.update();

        self.player.detect_hits(&mut self.invaders);
        
        self.player.draw(&mut next);
        self.invaders.draw(&mut next);

        self.render.update(next);
    }

    pub fn render(&self) -> String {
        self.render.render()
    }
}

#[wasm_bindgen]
#[repr(u8)] //make each cell represented as a byte
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    LEFT = 0,
    RIGHT = 1,
    SHOOT = 2,
}

#[wasm_bindgen]
#[repr(u8)] //make each cell represented as a byte
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Result {
    WIN = 0,
    LOSE = 1,
    NONE = 2,
}