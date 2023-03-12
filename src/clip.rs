use std::{io::stdout, slice::Iter};

use crossterm::{event::Event, queue, Result};

use crate::{
    children::{Child, ChildState, Children},
    helpers::{style::StateStyle, Rect, SelectionState},
    strs,
    traits::{Draw, Effect, Input},
    volume::Volume,
};

pub struct Clip {
    name: String,
    effects: Vec<Box<dyn Effect>>,
    child_state: ChildState,
}

impl Clip {
    pub fn new(name: String) -> Self {
        let effects: Vec<Box<dyn Effect>> = vec![Box::new(Volume::new()), Box::new(Volume::new())];
        Self {
            name,
            effects,
            child_state: ChildState::default(),
        }
    }
}

impl Children for Clip {
    type Child = dyn Effect;
    fn child_state(&self) -> &ChildState {
        &self.child_state
    }
    fn get_children(&self) -> Iter<Box<Self::Child>> {
        self.effects.iter()
    }
}

impl Draw for Clip {
    fn draw(&self, rect: Rect, state: SelectionState) -> Result<Rect> {
        let mut out = stdout();
        let mut rect = rect;
        let title = format! {"┃ {} {}", strs::SPEAKER, self.name};
        queue!(out, rect.pos(), title.state(state))?;
        let rects = self.draw_children(rect.right(), state)?;
        for rect in rects {
            queue!(out, rect.pos(), "┃".state(state))?;
        }
        // for (i, effect) in self.effects.iter().enumerate() {
        //     rect = if i == self.selected_effect {
        //         effect.draw(rect.down(), SelectionState::Active)?
        //     } else {
        //         effect.draw(rect.down(), SelectionState::None)?
        //     };
        //     queue!(out, start.pos(), "┣".state(state))?;
        //     let height = rect.y - start.y;
        //     for _ in 0..height {
        //         start = start.down();
        //         queue!(out, start.pos(), "┃".state(state))?;
        //     }
        //     start = start.down();
        // }
        Ok(rect.down().left())
    }
}

impl Input for Clip {
    fn handle_input(&mut self, event: Event) -> Option<Event> {
        // let active_effect = self.;

        // let event = active_effect.handle_input(event);

        Some(event)
    }
}
