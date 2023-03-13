use crate::{
    helpers::{Rect, SelectionState},
    traits::{Draw, Input},
};
use crossterm::Result;

pub struct Children<C: Child + ?Sized> {
    active: usize,
    selected: Vec<usize>,
    children: Vec<Box<C>>,
}

impl<C: Child + ?Sized> Children<C> {
    pub fn new(children: Vec<Box<C>>) -> Self {
        Children {
            active: 0,
            selected: Vec::new(),
            children,
        }
    }

    pub fn draw(&self, rect: Rect, state: SelectionState) -> Result<Vec<Rect>> {
        let mut ret = Vec::new();
        let mut rect = rect;
        for (i, child) in self.children.iter().enumerate() {
            match state {
                SelectionState::Active => {
                    if i == self.active {
                        rect = child.draw(rect, SelectionState::Active)?;
                    } else if self.selected.contains(&i) {
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
            rect = rect.next();
        }
        Ok(ret)
    }

    pub fn active(&mut self) -> &mut C {
        self.children[self.active].as_mut()
    }
}

impl Into<Children<dyn Child>> for Vec<Box<dyn Child>> {
    fn into(self) -> Children<dyn Child> {
        Children::new(self)
    }
}

pub trait Child: Draw + Input {}
