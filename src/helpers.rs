pub mod action;
pub mod style;

use crossterm::cursor;

#[derive(Debug, Copy, Clone)]
pub enum SelectionState {
    None,
    Active,
    Selected,
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
            width: self.width,
            height: self.height,
        }
    }

    pub fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }

    pub fn right_n(&self, n: u16) -> Self {
        Self {
            x: self.x + n,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }

    pub fn pos(&self) -> cursor::MoveTo {
        cursor::MoveTo(self.x, self.y)
    }

    pub fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}
