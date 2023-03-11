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
        LeaveAlternateScreen,
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

        update(event, &mut search)?;
    }

    Ok(())
}

fn update(event: Event, search: &mut Search) -> Result<()> {
    let mut out = stdout();
    // clear screen

    out.queue(Clear(ClearType::All))?;

    header()?;
    // out.queue(cursor::MoveTo(10, 10))?;
    // boxed("Hello World", 20, Some("🔍Search"))?;

    out.queue(cursor::MoveTo(10, 5))?;
    search.handle_input(event);
    search.draw(20, SelectionState::Active)?;

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

fn boxed(text: &str, width: i32, label: Option<&str>) -> Result<()> {
    let mut out = stdout();
    let cursor = cursor::position()?;
    let h = strs::H.repeat(width as usize - 2);
    let text = format!("{:width$}", text, width = width as usize - 2);

    queue!(
        out,
        cursor::MoveTo(cursor.0, cursor.1),
        style::Print(strs::TL),
        style::Print(&h),
        style::Print(strs::TR),
        cursor::MoveTo(cursor.0, cursor.1 + 1),
        style::Print(strs::V),
        style::Print(text),
        style::Print(strs::V),
        cursor::MoveTo(cursor.0, cursor.1 + 2),
        style::Print(strs::BL),
        style::Print(&h),
        style::Print(strs::BR),
    )?;

    if let Some(label) = label {
        queue!(
            out,
            cursor::MoveTo(cursor.0 + 2, cursor.1),
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
    fn draw(&self, width: i32, state: SelectionState) -> Result<()>;
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
    fn draw(&self, width: i32, state: SelectionState) -> Result<()> {
        let mut out = stdout();
        let mut cursor = cursor::position()?;
        let title = format! {"┃ {} {}", strs::SPEAKER, self.name};
        let style = match state {
            SelectionState::None => |t: String| t.stylize(),
            SelectionState::Active => |t: String| t.green().bold(),
            SelectionState::Selected => |t: String| t.dark_green(),
        };
        queue!(out, style::Print(style(title)), cursor.down_right())?;
        for (i, effect) in self.effects.iter().enumerate() {
            // queue!(
            //     out,
            //     style::Print(style("┣".to_string())),
            //     cursor::MoveTo(cursor.0, cursor.1 + 1),
            //     style::Print(style(strs::VB.to_string())),
            //     cursor::MoveTo(cursor.0 + 1, cursor.1)
            // )?;
            if i == self.selected_effect {
                effect.draw(width, SelectionState::Active)?;
            } else {
                effect.draw(width, SelectionState::None)?;
            }
            let new = cursor::position()?;
            queue!(out, cursor.down(), style::Print(style("┣".to_string())))?;
            let height = new.1 - cursor.1;
            for i in 2..height {
                queue!(
                    out,
                    cursor::MoveTo(cursor.0, cursor.1 + i),
                    style::Print(style(strs::VB.to_string()))
                )?;
            }
            cursor = (cursor.0, cursor.1 + height - 1);
            out.queue(new.position())?;
        }
        let new = cursor::position()?;
        out.queue(new.left())?;
        Ok(())
        // if selected draw blue background
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
    fn draw(&self, width: i32, state: SelectionState) -> Result<()> {
        let mut out = stdout();
        let cursor = cursor::position()?;
        queue!(out, style::Print("Search"), cursor.down())?;
        for (i, clip) in self.clips.iter().enumerate() {
            // move cursor down one
            if i == self.active_clip {
                clip.draw(width, SelectionState::Active)?;
            } else {
                clip.draw(width, SelectionState::None)?;
            }
        }
        Ok(())
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
    fn draw(&self, width: i32, state: SelectionState) -> Result<()> {
        let mut out = stdout();
        let cursor = cursor::position()?;
        let h = "─".repeat(width as usize - 2);
        queue!(out, style::Print(&h), cursor.down())
    }
}

impl Input for Slider {
    fn handle_input(&mut self, event: Event) -> Option<Event> {
        if event.is(Action::Right).is_some() {
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
    fn draw(&self, width: i32, state: SelectionState) -> Result<()> {
        let mut out = stdout();
        let cursor = cursor::position()?;
        queue!(out, style::Print(" Volume"), cursor.down(),)?;
        self.min_max.draw(width, state)
    }
}

impl Volume {
    fn new() -> Self {
        Self {
            min_max: Slider::new(SliderState::MinMax(0.3, 0.0)),
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

trait Down {
    fn down(&self) -> cursor::MoveTo;
}

impl Down for (u16, u16) {
    fn down(&self) -> cursor::MoveTo {
        cursor::MoveTo(self.0, self.1 + 1)
    }
}

trait DownRight {
    fn down_right(&self) -> cursor::MoveTo;
}

impl DownRight for (u16, u16) {
    fn down_right(&self) -> cursor::MoveTo {
        cursor::MoveTo(self.0 + 1, self.1 + 1)
    }
}

trait Left {
    fn left(&self) -> cursor::MoveTo;
}

impl Left for (u16, u16) {
    fn left(&self) -> cursor::MoveTo {
        cursor::MoveTo(self.0 - 1, self.1)
    }
}

trait Position {
    fn position(&self) -> cursor::MoveTo;
}

impl Position for (u16, u16) {
    fn position(&self) -> cursor::MoveTo {
        cursor::MoveTo(self.0, self.1)
    }
}
