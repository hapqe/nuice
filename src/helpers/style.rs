use crossterm::style::{self, StyledContent, Stylize};

use super::SelectionState;

pub trait StateStyle {
    fn state(&self, state: SelectionState) -> style::Print<StyledContent<String>>;
}

impl StateStyle for String {
    fn state(&self, state: SelectionState) -> style::Print<StyledContent<String>> {
        match state {
            SelectionState::Active => style::Print(self.clone().green()),
            SelectionState::Selected => style::Print(self.clone().yellow()),
            SelectionState::None => style::Print(self.clone().stylize()),
        }
    }
}

impl StateStyle for &str {
    fn state(&self, state: SelectionState) -> style::Print<StyledContent<String>> {
        self.to_string().state(state)
    }
}
