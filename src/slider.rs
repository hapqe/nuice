use std::io::stdout;

use crossterm::{event::Event, queue, style, Result};

use crate::{
    helpers::{
        action::{Action, IsEvent},
        Rect, SelectionState,
    },
    strs,
    traits::{Draw, Input},
};

pub struct Slider {
    state: SliderState,
    selection: SliderSelection,
}

impl Slider {
    pub fn new(state: SliderState) -> Self {
        Self {
            state,
            selection: SliderSelection::Center,
        }
    }
}

impl Draw for Slider {
    fn draw(&self, rect: Rect, state: SelectionState) -> Result<Rect> {
        let mut out = stdout();

        let h = "─".repeat(rect.width as usize - 2);
        queue!(out, rect.pos(), style::Print(&h), rect.down().pos())?;

        match self.state {
            SliderState::MinMax(min, max) => {
                let pos = rect.right_n((min * 10.0) as u16);
                queue!(out, pos.pos(), style::Print(strs::LEFT))?;
            }
            SliderState::Value(value) => {}
        }

        Ok(rect)
    }
}

impl Input for Slider {
    fn handle_input(&mut self, event: Event) -> Option<Event> {
        if event.is(Action::Right).is_some() {
            match self.state {
                SliderState::MinMax(min, max) => {
                    self.state = SliderState::MinMax(min + 0.1, max + 0.1);
                }
                SliderState::Value(value) => {
                    self.state = SliderState::Value(value + 0.1);
                }
            }
            return None;
        }
        Some(event)
    }
}

pub enum SliderState {
    MinMax(f32, f32),
    Value(f32),
}

enum SliderSelection {
    Min,
    Max,
    Center,
}
