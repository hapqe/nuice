use std::{
    io::{stdout, BufReader},
    slice::Iter,
};

use crossterm::{event::Event, queue, Result};

use crate::{
    children::{Child, Children},
    helpers::{
        action::{Action, IsEvent},
        style::StateStyle,
        CombinedRect, Rect, SelectionState, VerticalRepeat,
    },
    info, strs,
    traits::{Draw, Effect, Input},
    volume::Volume,
};

pub struct Clip {
    name: String,
    effects: Children<dyn Effect>,
}

impl Clip {
    pub fn new(name: String) -> Self {
        Self {
            name,
            effects: Children::new(vec![Box::new(Volume::new()), Box::new(Volume::new())]),
        }
    }
}

impl Child for Clip {}

impl Draw for Clip {
    fn draw(&self, rect: Rect, state: SelectionState) -> Result<Rect> {
        let mut out = stdout();
        let title = format! {"┃ {} {}", strs::SPEAKER, self.name};
        queue!(out, rect.pos(), title.state(state))?;
        let rects = self.effects.draw(rect.right().down(), state)?;
        for rect in rects.iter() {
            queue!(
                out,
                rect.left().pos(),
                "┣".state(state),
                rect.left().down().pos()
            )?;
            "┃".state(state).v_repeat(rect.height - 1)?;
        }
        Ok(rects.combined().unwrap_or(rect).left().end())
    }
}

impl Input for Clip {
    fn handle_input(&mut self, event: Event) -> Option<Event> {
        if event.is(Action::Play) {
            info(format!("Play {}", self.name));
            let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

            let path = "/home/hapke/Desktop/snagg.wav";

            let file = std::fs::File::open(path).unwrap();
            let beep1 = stream_handle.play_once(BufReader::new(file)).unwrap();

            std::thread::sleep(std::time::Duration::from_millis(1000));
        }

        self.effects.active().handle_input(event)
    }
}
