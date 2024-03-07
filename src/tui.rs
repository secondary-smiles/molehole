use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use crossterm::{event, event::Event, execute};
use ratatui::prelude::{CrosstermBackend, Terminal};
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
