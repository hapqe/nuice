use std::io::stdout;

use crossterm::{event::Event, queue, style, Result};

use crate::{
    clip::Clip,
    helpers::{Rect, SelectionState},
    traits::{Draw, Input},
};

pub struct Search {
    query: String,
    clips: Vec<Clip>,
    active_clip: usize,
}

impl Search {
    pub fn query(query: String) -> Self {
        let mut clips = vec![
            Clip::new("Clip 1".to_string()),
            Clip::new("Clip 2".to_string()),
            Clip::new("Clip 3".to_string()),
        ];
        Self {
            query,
            clips,
            active_clip: 0,
        }
    }
}

impl Draw for Search {
    fn draw(&self, rect: Rect, state: SelectionState) -> Result<Rect> {
        let mut rect = rect;
        let mut out = stdout();
        queue!(out, rect.pos(), style::Print("Search"))?;
        for (i, clip) in self.clips.iter().enumerate() {
            rect = if i == self.active_clip {
                clip.draw(rect, SelectionState::Active)?
            } else {
                clip.draw(rect, SelectionState::None)?
            }
        }
        Ok(rect)
    }
}

impl Input for Search {
    fn handle_input(&mut self, event: Event) -> Option<Event> {
        let active_clip = &mut self.clips[self.active_clip];
        let event = active_clip.handle_input(event);
        event
    }
}
