pub mod action;
pub mod style;

use std::io::stdout;

use crossterm::cursor;
use crossterm::queue;
use crossterm::Result;

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

pub trait HorizontalRepeat {
    fn h_repeat(&self, n: u16) -> Result<()>;
}

impl HorizontalRepeat for &str {
    fn h_repeat(&self, n: u16) -> Result<()> {
        let mut out = stdout();
        let text = self.repeat(n as usize);
        queue!(out, crossterm::style::Print(text))?;
        Ok(())
    }
}

pub trait VerticalRepeat {
    fn v_repeat(&self, n: u16) -> Result<()>;
}

impl VerticalRepeat for &str {
    fn v_repeat(&self, n: u16) -> Result<()> {
        let mut out = stdout();
        for _ in 0..n {
            queue!(
                out,
                crossterm::style::Print(self),
                crossterm::cursor::MoveDown(1),
                crossterm::cursor::MoveLeft(self.len() as u16)
            )?;
        }
        Ok(())
    }
}
