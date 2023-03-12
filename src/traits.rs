use crossterm::event::Event;
use crossterm::Result;

use crate::helpers::{Rect, SelectionState};

pub trait Draw {
    fn draw(&self, rect: Rect, state: SelectionState) -> Result<Rect>;
}

pub trait Input {
    fn handle_input(&mut self, event: Event) -> Option<Event>;
}

pub trait Effect: Draw + Input {
    fn name(&self) -> String;
}
