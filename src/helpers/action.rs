use crossterm::event::{Event, KeyCode};

pub enum Action {
    Up,
    Down,
    Left,
    Right,
}

pub struct EventThatIs<'a> {
    event: &'a Event,
}

pub trait IsEvent {
    fn is(&self, action: Action) -> Option<EventThatIs>;
}

impl IsEvent for Event {
    fn is(&self, action: Action) -> Option<EventThatIs> {
        if let Event::Key(key) = self {
            match action {
                Action::Up => {
                    if key.code == KeyCode::Up {
                        return Some(EventThatIs { event: self });
                    }
                }
                Action::Down => {
                    if key.code == KeyCode::Down {
                        return Some(EventThatIs { event: self });
                    }
                }
                Action::Left => {
                    if key.code == KeyCode::Left {
                        return Some(EventThatIs { event: self });
                    }
                }
                Action::Right => {
                    if key.code == KeyCode::Right {
                        return Some(EventThatIs { event: self });
                    }
                }
            }
        }
        None
    }
}

pub trait EventThatAlsoIs {
    fn and(&self, action: Action) -> Option<EventThatIs>;
}

impl<'a> EventThatAlsoIs for Option<EventThatIs<'a>> {
    fn and(&self, action: Action) -> Option<EventThatIs> {
        if let Some(that_is) = self {
            return that_is.event.is(action);
        }
        None
    }
}
