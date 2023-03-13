use std::{fs, io::stdout, slice::Iter};

use crossterm::{event::Event, queue, style, Result};

use crate::{
    children::Children,
    clip::Clip,
    helpers::{Rect, SelectionState},
    traits::{Draw, Input},
};

pub struct Search {
    query: String,
    clips: Children<Clip>,
}

impl Search {
    pub fn query(query: String) -> Self {
        let folder_path = "/home/hapke/Desktop/TestSounds";

        let paths = fs::read_dir(folder_path).unwrap();

        let mut clips = vec![];

        for path in paths {
            let path = path.unwrap().path();
            let path = path.to_str().unwrap().to_string();
            // if lowercase path contains lowercase query
            if path.to_lowercase().contains(&query.to_lowercase()) {
                clips.push(Box::new(Clip::new(path)));
            }
        }

        Self {
            query,
            clips: Children::new(clips),
        }
    }
}

impl Draw for Search {
    fn draw(&self, rect: Rect, state: SelectionState) -> Result<Rect> {
        let mut rect = rect;
        let mut out = stdout();
        queue!(out, rect.pos(), style::Print("Search"))?;
        self.clips.draw(rect.next(), state)?;
        Ok(rect)
    }
}

impl Input for Search {
    fn handle_input(&mut self, event: Event) -> Option<Event> {
        self.clips.active().handle_input(event)
    }
}
