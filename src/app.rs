use eyre::Result;
use ratatui::prelude::*;
use std::io::Stdout;
use std::time::Duration;

use crate::tui;

pub struct App {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    pub tick_rate: Duration,
}

impl App {
    pub fn start(tick_rate: Duration) -> Result<Self> {
        let terminal = tui::init()?;

        Ok(Self {
            terminal,
            tick_rate,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn quit(&mut self) -> Result<()> {
        tui::restore()?;

        Ok(())
    }
}
