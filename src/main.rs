mod children;
mod clip;
mod header;
mod helpers;
mod info;
mod search;
mod slider;
mod strs;
mod traits;
mod volume;

use std::io::{stdout, Write};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    queue,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    QueueableCommand, Result,
};

use header::*;
use helpers::{Rect, SelectionState};
use info::*;
use search::*;
use traits::{Draw, Input};

fn events() -> Result<()> {
    let mut search = Search::query("hit".to_string());
    update(Event::FocusGained, &mut search)?;
    loop {
        let event = read()?;

        if event == Event::Key(KeyCode::Char('q').into()) {
            break;
        }

        update(event, &mut search)?;
    }

    Ok(())
}

fn update(event: Event, search: &mut Search) -> Result<()> {
    let mut out = stdout();

    out.queue(Clear(ClearType::All))?;

    header()?;

    search.handle_input(event);
    search.draw(Rect::new(10, 5, 40, 1), SelectionState::Active)?;

    // info("Press q to quit".to_string())?;

    out.flush()
}

fn main() -> Result<()> {
    let mut stdout = stdout();

    enable_raw_mode()?;
    queue!(stdout, EnterAlternateScreen, cursor::Hide)?;
    stdout.flush()?;

    events()?;

    queue!(stdout, LeaveAlternateScreen, cursor::Show)?;
    stdout.flush()?;
    disable_raw_mode()?;

    Ok(())
}
