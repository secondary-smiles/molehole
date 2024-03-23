use crossterm::event::Event;
use eyre::Result;
use ratatui::prelude::*;
use std::time::Duration;

use crate::app_action::AppAction;
use crate::app_event::AppEvent;
use crate::component::Component;
use crate::components;
use crate::keys::key_commands::KeyCommand;
use crate::tui;
pub struct App {
    pub tui: tui::Tui,
    pub tick_rate: Duration,
    pub components: Vec<Box<dyn Component>>,
    pub key_commands: Vec<KeyCommand>,

    should_quit: bool,
}

impl App {
    pub fn new(tick_rate: Duration) -> Result<Self> {
        let tui = tui::init()?;

        Ok(Self {
            tui,
            tick_rate,

            should_quit: false,
            components: vec![],
            key_commands: vec![],
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let global_keys = components::global_keys::GlobalKeys {
            key_commands: self.key_commands.clone(),
            ..Default::default()
        };
        let status_bar = components::status::StatusBar::default();
        self.components = vec![Box::new(global_keys), Box::new(status_bar)];

        for component in &mut self.components {
            component.init()?;
        }

        loop {
            if self.should_quit {
                break Ok(());
            }

            self.draw()?;
        }
    }

    pub fn draw(&mut self) -> Result<()> {
        let event: Option<AppEvent> = match tui::get_event(self.tick_rate)? {
            Some(event) => match event {
                Event::Key(key) => Some(AppEvent::Key(key)),
                Event::Mouse(mouse) => Some(AppEvent::Mouse(mouse)),
                Event::FocusGained => todo!(),
                Event::FocusLost => todo!(),
                Event::Paste(_) => todo!(),
                Event::Resize(_, _) => todo!(),
            },
            None => None,
        };

        if let Some(event) = event {
            let mut actions: Vec<AppAction> = vec![];
            for component in &mut self.components {
                if let Some(action) = component.handle_event(event.clone())? {
                    actions.push(action);
                }
            }

            for action in actions {
                self.handle_action(action)?;
            }
        }

        if self.should_quit {
            return Ok(());
        }

        self.tui.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(100),
                    Constraint::Min(1),
                ])
                .split(frame.size());

            // status bar
            let _ = self.components[1].render(frame, layout[1]);
            // global_keys
            let _ = self.components[0].render(frame, frame.size());
        })?;

        Ok(())
    }

    pub fn quit(&mut self) -> Result<()> {
        tui::restore()?;
        self.should_quit = true;

        Ok(())
    }

    fn handle_action(&mut self, action: AppAction) -> Result<()> {
        match action {
            AppAction::Quit => Ok(self.quit()?),
            _ => {
                for component in &mut self.components {
                    component.handle_action(action.clone());
                }
                Ok(())
            }
        }
    }
}
