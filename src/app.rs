use crossterm::event::Event;
use eyre::Result;
use ratatui::prelude::*;
use std::time::Duration;

use crate::app_event::AppEvent;
use crate::component::Component;
use crate::components;
use crate::tui;

pub struct App {
    pub tui: tui::Tui,
    pub tick_rate: Duration,
    pub components: Vec<Box<dyn Component>>,
}

impl App {
    pub fn new(tick_rate: Duration) -> Result<Self> {
        let tui = tui::init()?;

        let hello_world = components::hello_world::HelloWorld::default();
        let hello_world1 = components::hello_world::HelloWorld::default();
        let hello_world2 = components::hello_world::HelloWorld::default();
        let hello_world3 = components::hello_world::HelloWorld::default();

        Ok(Self {
            tui,
            tick_rate,
            components: vec![
                Box::new(hello_world),
                Box::new(hello_world1),
                Box::new(hello_world2),
                Box::new(hello_world3),
            ],
        })
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            let event: Option<AppEvent> = match tui::get_event(self.tick_rate)?
            {
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

            if event.is_some() {
                for component in self.components.iter_mut() {
                    let _ = component.handle_event(event.expect(""))?;
                }
            }

            self.tui.draw(|frame| {
                let layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                    ])
                    .split(frame.size());

                for (i, component) in self.components.iter_mut().enumerate() {
                    let _ = component.render(frame, layout[i]);
                }
            })?;
        }
    }

    pub fn quit(&mut self) -> Result<()> {
        tui::restore()?;

        Ok(())
    }
}
