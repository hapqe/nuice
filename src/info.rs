use std::io::stdout;

use crossterm::{
    cursor,
    style::{self, Stylize},
    QueueableCommand, Result,
};

pub fn info(info: String) -> Result<()> {
    let mut out = stdout();
    let size = crossterm::terminal::size()?;
    out.queue(cursor::MoveTo(0, size.1 - 1))?;
    out.queue(style::Print(info.italic().grey()))?;
    Ok(())
}
