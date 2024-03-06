use crossterm::event::Event;
use eyre::Result;
use std::time::Duration;

use crate::app_action::AppAction;
use crate::app_event::AppEvent;
use crate::component::Component;
use crate::components;
use crate::tui;

pub struct App {
    pub tui: tui::Tui,
    pub tick_rate: Duration,
    pub components: Vec<Box<dyn Component>>,

    should_quit: bool,
}

impl App {
    pub fn new(tick_rate: Duration) -> Result<Self> {
        let tui = tui::init()?;

        let global_keys = components::global_keys::GlobalKeys::default();
        let hello_world = components::hello_world::HelloWorld::default();

        Ok(Self {
            tui,
            tick_rate,
            components: vec![Box::new(hello_world), Box::new(global_keys)],
            should_quit: false,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        for component in self.components.iter_mut() {
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
            for component in self.components.iter_mut() {
                match component.handle_event(event)? {
                    Some(action) => actions.push(action),
                    None => (),
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
            for (_i, component) in self.components.iter_mut().enumerate() {
                match component.render(frame, frame.size()) {
                    Ok(_) => (),
                    Err(_) => (),
                }
            }
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
        }
    }
}
