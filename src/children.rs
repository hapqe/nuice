use std::slice::Iter;

use crate::{
    helpers::{Rect, SelectionState},
    traits::{Draw, Input},
};
use crossterm::Result;

pub struct ChildState {
    pub active: usize,
    pub selected: Vec<usize>,
}

impl Default for ChildState {
    fn default() -> Self {
        Self {
            active: 0,
            selected: vec![],
        }
    }
}

pub trait Child: Draw + Input {}

pub trait Children {
    type Child: Child + ?Sized;
    fn draw_children(&self, rect: Rect, state: SelectionState) -> Result<Vec<Rect>> {
        let mut ret = Vec::new();
        let mut rect = rect;
        let info = self.child_state();
        for (i, child) in self.get_children().enumerate() {
            match state {
                SelectionState::Active => {
                    if i == info.active {
                        rect = child.draw(rect, SelectionState::Active)?;
                    } else if info.selected.contains(&i) {
                        rect = child.draw(rect, SelectionState::Selected)?;
                    } else {
                        rect = child.draw(rect, SelectionState::None)?;
                    }
                }
                _ => {
                    rect = child.draw(rect, SelectionState::None)?;
                }
            }
            ret.push(rect);
        }
        Ok(ret)
    }
    fn child_state(&self) -> &ChildState;
    fn get_children(&self) -> Iter<Box<Self::Child>>;
}
