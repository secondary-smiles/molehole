use crossterm::event::{
    Event, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
    PushKeyboardEnhancementFlags,
};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use crossterm::{event, execute};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io;
use std::io::{stdout, Stdout};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    execute!(
        stdout(),
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES,
        )
    )?;
    enable_raw_mode()?;

    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    execute!(stdout(), PopKeyboardEnhancementFlags)?;
    disable_raw_mode()?;

    Ok(())
}

pub fn get_event(tick: std::time::Duration) -> io::Result<Option<Event>> {
    if event::poll(tick)? {
        return Ok(Some(event::read()?));
    }

    Ok(None)
}
