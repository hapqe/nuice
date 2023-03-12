use std::{io::stdout, slice::Iter};

use crossterm::{event::Event, queue, style, Result};

use crate::{
    children::{ChildState, Children},
    clip::Clip,
    helpers::{Rect, SelectionState},
    traits::{Draw, Input},
};

pub struct Search {
    query: String,
    clips: Vec<Box<Clip>>,
    child_state: ChildState,
}

impl Search {
    pub fn query(query: String) -> Self {
        let mut clips = vec![
            Box::new(Clip::new("Clip 1".to_string())),
            Box::new(Clip::new("Clip 2".to_string())),
            Box::new(Clip::new("Clip 3".to_string())),
        ];
        Self {
            query,
            clips,
            child_state: ChildState::default(),
        }
    }
}

impl Children for Search {
    type Child = Clip;
    fn child_state(&self) -> &ChildState {
        &self.child_state
    }
    fn get_children(&self) -> Iter<Box<Self::Child>> {
        self.clips.iter()
    }
}

impl Draw for Search {
    fn draw(&self, rect: Rect, state: SelectionState) -> Result<Rect> {
        let mut rect = rect;
        let mut out = stdout();
        queue!(out, rect.pos(), style::Print("Search"))?;
        self.draw_children(rect.down(), state)?;
        Ok(rect)
    }
}

impl Input for Search {
    fn handle_input(&mut self, event: Event) -> Option<Event> {
        let active_clip = &mut self.clips[self.child_state.active];
        let event = active_clip.handle_input(event);
        event
    }
}
