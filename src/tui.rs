use crossterm::{event, event::Event, execute, terminal::*};
use ratatui::prelude::*;
use std::io;
use std::io::{stdout, Stdout};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;

    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

pub fn get_event(tick: std::time::Duration) -> io::Result<Option<Event>> {
    if event::poll(tick)? {
        return Ok(Some(event::read()?));
    }

    Ok(None)
}
