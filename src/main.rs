mod strs;

use std::{
    fmt::Display,
    io::{stdout, Stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode},
    queue,
    style::{self, SetForegroundColor, Stylize},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen, ScrollDown, SetSize,
    },
    ExecutableCommand, QueueableCommand, Result,
};

fn events() -> Result<()> {
    let mut search = Search::query("".to_string());
    update(Event::FocusGained, &mut search)?;
    loop {
        let event = read()?;

        if event == Event::Key(KeyCode::Char('q').into()) {
            break;
        }

        // if j
        // if event == Event::Key(KeyCode::Char('j').into()) {
        //     queue!(stdout(), SetSize(10, 10), ScrollDown(1))?;
        // }

        update(event, &mut search)?;
    }

    Ok(())
}

fn update(event: Event, search: &mut Search) -> Result<()> {
    let mut out = stdout();

    out.queue(Clear(ClearType::All))?;

    header()?;

    search.handle_input(event);
    search.draw(Rect::new(10, 5, 20, 10), SelectionState::Active)?;

    info("Press q to quit".to_string())?;

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

fn header() -> Result<()> {
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

fn info(info: String) -> Result<()> {
    let mut out = stdout();
    let size = crossterm::terminal::size()?;
    out.queue(cursor::MoveTo(0, size.1 - 1))?;
    out.queue(style::Print(info.italic().grey()))?;
    Ok(())
}

fn boxed(text: &str, width: i32, label: Option<&str>) -> Result<()> {
    let mut out = stdout();
    let h = strs::H.repeat(width as usize - 2);
    let text = format!("{:width$}", text, width = width as usize - 2);

    // queue!(
    //     out,
    //     cursor::MoveTo(cursor.0, cursor.1),
    //     style::Print(strs::TL),
    //     style::Print(&h),
    //     style::Print(strs::TR),
    //     cursor::MoveTo(cursor.0, cursor.1 + 1),
    //     style::Print(strs::V),
    //     style::Print(text),
    //     style::Print(strs::V),
    //     cursor::MoveTo(cursor.0, cursor.1 + 2),
    //     style::Print(strs::BL),
    //     style::Print(&h),
    //     style::Print(strs::BR),
    // )?;

    if let Some(label) = label {
        queue!(
            out,
            // cursor::MoveTo(cursor.0 + 2, cursor.1),
            style::Print(label),
        )?;
    }
    Ok(())
}

enum SelectionState {
    None,
    Active,
    Selected,
}

trait Draw {
    fn draw(&self, rect: Rect, state: SelectionState) -> Result<Rect>;
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

impl Rect {
    fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
            width: self.width,
            height: self.height,
        }
    }

    fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }

    fn right_n(&self, n: u16) -> Self {
        Self {
            x: self.x + n,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }

    fn pos(&self) -> cursor::MoveTo {
        cursor::MoveTo(self.x, self.y)
    }

    fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}

struct Clip {
    name: String,
    effects: Vec<Box<dyn Effect>>,
    selected_effect: usize,
}

impl Clip {
    fn new(name: String) -> Self {
        let effects: Vec<Box<dyn Effect>> = vec![Box::new(Volume::new()), Box::new(Volume::new())];
        Self {
            name,
            effects,
            selected_effect: 0,
        }
    }
}

impl Draw for Clip {
    fn draw(&self, rect: Rect, state: SelectionState) -> Result<Rect> {
        let mut out = stdout();
        let mut rect = rect;
        let title = format! {"┃ {} {}", strs::SPEAKER, self.name};
        let style = match state {
            SelectionState::None => |t: String| t.stylize(),
            SelectionState::Active => |t: String| t.green().bold(),
            SelectionState::Selected => |t: String| t.dark_green(),
        };
        queue!(out, rect.pos(), style::Print(style(title)),)?;
        let mut start = rect.down();
        rect = rect.right();
        for (i, effect) in self.effects.iter().enumerate() {
            rect = if i == self.selected_effect {
                effect.draw(rect.down(), SelectionState::Active)?
            } else {
                effect.draw(rect.down(), SelectionState::None)?
            };
            queue!(out, start.pos(), style::Print(style("┣".to_string())))?;
            let height = rect.y - start.y;
            for i in 0..height {
                start = start.down();
                queue!(out, start.pos(), style::Print(style("┃".to_string())))?;
            }
            start = start.down();
        }
        Ok(rect.down().left())
    }
}

struct Search {
    query: String,
    clips: Vec<Clip>,
    active_clip: usize,
}

impl Search {
    fn query(query: String) -> Self {
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

trait Effect: Draw + Input {
    fn name(&self) -> String;
}

trait Input {
    fn handle_input(&mut self, event: Event) -> Option<Event>;
}

impl Input for Search {
    fn handle_input(&mut self, event: Event) -> Option<Event> {
        let active_clip = &mut self.clips[self.active_clip];
        let event = active_clip.handle_input(event);
        event
    }
}

impl Input for Clip {
    fn handle_input(&mut self, event: Event) -> Option<Event> {
        let active_effect = &mut self.effects[self.selected_effect];

        let event = active_effect.handle_input(event);

        event
    }
}

enum SliderState {
    MinMax(f32, f32),
    Value(f32),
}

enum SliderSelection {
    Min,
    Max,
    Center,
}

struct Slider {
    state: SliderState,
    selection: SliderSelection,
}

impl Slider {
    fn new(state: SliderState) -> Self {
        Self {
            state,
            selection: SliderSelection::Center,
        }
    }
}

impl Draw for Slider {
    fn draw(&self, rect: Rect, state: SelectionState) -> Result<Rect> {
        let mut out = stdout();

        let h = "─".repeat(rect.width as usize - 2);
        queue!(out, rect.pos(), style::Print(&h), rect.down().pos())?;

        match self.state {
            SliderState::MinMax(min, max) => {
                let pos = rect.right_n((min * 10.0) as u16);
                queue!(out, pos.pos(), style::Print(strs::LEFT))?;
            }
            SliderState::Value(value) => {}
        }

        Ok(rect)
    }
}

impl Input for Slider {
    fn handle_input(&mut self, event: Event) -> Option<Event> {
        if event.is(Action::Right).is_some() {
            match self.state {
                SliderState::MinMax(min, max) => {
                    self.state = SliderState::MinMax(min + 0.1, max + 0.1);
                }
                SliderState::Value(value) => {
                    self.state = SliderState::Value(value + 0.1);
                }
            }
            return None;
        }
        Some(event)
    }
}

struct Volume {
    min_max: Slider,
}

impl Effect for Volume {
    fn name(&self) -> String {
        "Volume".to_string()
    }
}

impl Input for Volume {
    fn handle_input(&mut self, event: Event) -> Option<Event> {
        let event = self.min_max.handle_input(event);
        event
    }
}

impl Draw for Volume {
    fn draw(&self, rect: Rect, state: SelectionState) -> Result<Rect> {
        let mut out = stdout();
        queue!(out, rect.pos(), style::Print(" Volume"))?;
        self.min_max.draw(rect.down(), state)
    }
}

impl Volume {
    fn new() -> Self {
        Self {
            min_max: Slider::new(SliderState::MinMax(0.3, 0.5)),
        }
    }
}

enum Action {
    Up,
    Down,
    Left,
    Right,
}

struct EventThatIs<'a> {
    event: &'a Event,
}

trait IsEvent {
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

trait EventThatAlsoIs {
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
