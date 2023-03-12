use std::io::stdout;

use crate::{
    helpers::{Rect, SelectionState},
    slider::{Slider, SliderState},
    traits::{Draw, Effect, Input},
};

use crossterm::{event::Event, queue, style, Result};

pub struct Volume {
    min_max: Slider,
}

impl Effect for Volume {
    fn name(&self) -> String {
        "Volume".to_string()
    }
}

impl Input for Volume {
    fn handle_input(&mut self, event: Event) -> Option<Event> {
        let event = self.min_max.handle_input(event);
        event
    }
}

impl Draw for Volume {
    fn draw(&self, rect: Rect, state: SelectionState) -> Result<Rect> {
        let mut out = stdout();
        queue!(out, rect.pos(), style::Print(" Volume"))?;
        self.min_max.draw(rect.down(), state)
    }
}

impl Volume {
    pub fn new() -> Self {
        Self {
            min_max: Slider::new(SliderState::MinMax(0.3, 0.5)),
        }
    }
}
