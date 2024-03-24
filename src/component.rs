use crossterm::event::{KeyEvent, MouseEvent};
use eyre::Result;
use ratatui::prelude::{Frame, Rect};

use crate::app_action::AppAction;
use crate::app_event::AppEvent;

pub trait Component {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    #[allow(unused)]
    fn handle_action(&mut self, action: AppAction) {}

    #[allow(unused)]
    fn handle_event(&mut self, event: AppEvent) -> Option<AppAction> {
        match event {
            AppEvent::Key(key_event) => self.handle_key_event(key_event),
            AppEvent::Mouse(mouse_event) => {
                self.handle_mouse_event(mouse_event)
            }
            _ => None,
        }
    }

    #[allow(unused)]
    fn handle_key_event(&mut self, key: KeyEvent) -> Option<AppAction> {
        None
    }

    #[allow(unused)]
    fn handle_mouse_event(&mut self, mouse: MouseEvent) -> Option<AppAction> {
        None
    }

    #[allow(unused)]
    fn update(&mut self) -> Option<AppEvent> {
        None
    }

    fn render(&mut self, frame: &mut Frame, rect: Rect) -> Result<()>;
}
