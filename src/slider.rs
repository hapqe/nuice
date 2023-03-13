use std::io::stdout;

use crossterm::{event::Event, queue, style, Result};

use crate::{
    helpers::{
        action::{Action, IsEvent},
        HorizontalRepeat, Rect, SelectionState,
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

        queue!(out, rect.pos())?;
        "―".h_repeat(rect.width)?;

        match self.state {
            SliderState::MinMax(min, max) => {
                let min = rect.right_n((min * 10.0) as u16);
                queue!(out, min.pos(), style::Print(strs::LEFT))?;

                let max = rect.right_n((max * 10.0) as u16);
                queue!(out, max.pos(), style::Print(strs::RIGHT))?;

                queue!(out, min.right().pos())?;
                strs::MID.h_repeat(max.x - min.x - 1)?;
            }
            SliderState::Value(value) => {
                let pos = rect.right_n((value * 10.0) as u16);
                queue!(out, pos.pos(), style::Print(strs::CIRCLE))?;
            }
        }

        Ok(rect)
    }
}

impl Input for Slider {
    fn handle_input(&mut self, event: Event) -> Option<Event> {
        if event.is(Action::Right) {
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
        if event.is(Action::Toggle) {
            self.state = match self.state {
                SliderState::MinMax(min, max) => SliderState::Value((min + max) / 2.0),
                SliderState::Value(value) => SliderState::MinMax(value - 0.1, value + 0.1),
            };
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
