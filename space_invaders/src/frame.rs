use std::fmt;

use crate::{NUM_COLS, NUM_ROWS};

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

pub struct Render {
    frame: Frame,
}

impl Render {
    pub fn new() -> Self {
        let frame = new_frame();

        Self {
            frame
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn update(&mut self, next: Frame) {
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