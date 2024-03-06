use crossterm::event::{KeyEvent, MouseEvent};
use eyre::Result;
use ratatui::prelude::{Frame, Rect};

use crate::action::Action;
use crate::app_event::AppEvent;

pub trait Component {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    #[allow(unused)]
    fn handle_event(&mut self, event: AppEvent) -> Result<Option<Action>> {
        match event {
            AppEvent::Key(key_event) => Ok(self.handle_key_event(key_event)?),
            AppEvent::Mouse(mouse_event) => {
                Ok(self.handle_mouse_event(mouse_event)?)
            }
            _ => Ok(None),
        }
    }

    #[allow(unused)]
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        Ok(None)
    }

    #[allow(unused)]
    fn handle_mouse_event(
        &mut self,
        mouse: MouseEvent,
    ) -> Result<Option<Action>> {
        Ok(None)
    }

    #[allow(unused)]
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        Ok(None)
    }

    fn render(&mut self, frame: &mut Frame, rect: Rect) -> Result<()>;
}
