use crossterm::event::{Event, KeyCode, KeyModifiers};

use crate::info::info;

pub enum Action {
    Up,
    Down,
    Left,
    Right,
    C_Down,
    C_Up,
    C_Left,
    C_Right,
    Toggle,
    Play,
}

pub trait IsEvent {
    fn is(&self, action: Action) -> bool;
}

// a macro for checking a keycode
macro_rules! check_key {
    ($event:expr, $keycode:ident) => {
        *$event == Event::Key(KeyCode::$keycode.into())
    };
}

impl IsEvent for Event {
    fn is(&self, action: Action) -> bool {
        match action {
            Action::Up => check_key! {self, Up},
            Action::Down => check_key!(self, Down),
            Action::Left => check_key!(self, Left),
            Action::Right => check_key!(self, Right),
            Action::Toggle => check_key!(self, Tab),
            Action::Play => check_key!(self, Enter),
            _ => false,
        }
    }
}
