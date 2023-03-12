use crossterm::event::Event;
use crossterm::Result;

use crate::{
    children::Child,
    helpers::{Rect, SelectionState},
};

pub trait Draw {
    fn draw(&self, rect: Rect, state: SelectionState) -> Result<Rect>;
}

pub trait Input {
    fn handle_input(&mut self, event: Event) -> Option<Event>;
}

pub trait Effect: Child {
    fn name(&self) -> String;
}
