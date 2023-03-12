use crossterm::{
    cursor, queue,
    style::{self, Stylize},
    QueueableCommand, Result,
};

use std::io::stdout;

pub fn header() -> Result<()> {
    let mut out = stdout();
    let size = crossterm::terminal::size()?;
    out.queue(cursor::MoveTo(0, 0))?;
    let bar = "—".repeat(size.0 as usize);
    out.queue(style::Print(bar))?;

    out.queue(cursor::MoveTo(2, 0))?;

    let s = |s| style::PrintStyledContent(s);

    queue!(
        out,
        s("  ᑎ".blue()),
        s(" ᑌ".green()),
        s(" ∕".yellow()),
        s(" ᑕ".red()),
        s(" ᑢ  ".magenta())
    )?;

    let version = env!("CARGO_PKG_VERSION");
    let version = format!(" v: {} ", version);

    out.queue(cursor::MoveTo(size.0 - version.len() as u16 - 2, 0))?;
    out.queue(style::Print(version))?;
    Ok(())
}
