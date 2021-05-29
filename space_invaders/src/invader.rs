use crate::{NUM_COLS, NUM_ROWS, frame::{Frame, Drawable}};

pub struct Invader {
    pub x: usize,
    pub y: usize,
}

pub struct Invaders {
    pub army: Vec<Invader>,
    timer: usize,
    direction: i32,
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
            timer: 0,
            direction: 1,
        }
    }

    pub fn update(&mut self, delta: usize) -> bool {
        self.timer = delta;
        
        if self.timer > 1000 {
            self.timer = 0;

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

    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }

    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.y).max().unwrap_or(0) >= NUM_ROWS - 2
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
            frame[invader.x][invader.y] = if self.timer > 0 && self.timer <= 100 {
                "x"
            } else {
                "+"
            };
        }
    }
}