use std::io::stdout;

use crossterm::{event::Event, queue, Result};

use crate::{
    helpers::{style::StateStyle, Rect, SelectionState},
    strs,
    traits::{Draw, Effect, Input},
    volume::Volume,
};

pub struct Clip {
    name: String,
    effects: Vec<Box<dyn Effect>>,
    selected_effect: usize,
}

impl Clip {
    pub fn new(name: String) -> Self {
        let effects: Vec<Box<dyn Effect>> = vec![Box::new(Volume::new()), Box::new(Volume::new())];
        Self {
            name,
            effects,
            selected_effect: 0,
        }
    }
}

impl Draw for Clip {
    fn draw(&self, rect: Rect, state: SelectionState) -> Result<Rect> {
        let mut out = stdout();
        let mut rect = rect;
        let title = format! {"┃ {} {}", strs::SPEAKER, self.name};
        queue!(out, rect.pos(), title.state(state))?;
        let mut start = rect.down();
        rect = rect.right();
        for (i, effect) in self.effects.iter().enumerate() {
            rect = if i == self.selected_effect {
                effect.draw(rect.down(), SelectionState::Active)?
            } else {
                effect.draw(rect.down(), SelectionState::None)?
            };
            queue!(out, start.pos(), "┣".state(state))?;
            let height = rect.y - start.y;
            for _ in 0..height {
                start = start.down();
                queue!(out, start.pos(), "┃".state(state))?;
            }
            start = start.down();
        }
        Ok(rect.down().left())
    }
}

impl Input for Clip {
    fn handle_input(&mut self, event: Event) -> Option<Event> {
        let active_effect = &mut self.effects[self.selected_effect];

        let event = active_effect.handle_input(event);

        event
    }
}
