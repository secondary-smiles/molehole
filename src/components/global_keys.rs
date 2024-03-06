use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::app_action::AppAction;
use crate::component::Component;

#[derive(Default, Clone, Copy)]
pub struct GlobalKeys {
    should_show: bool,
}

impl Component for GlobalKeys {
    fn handle_key_event(
        &mut self,
        key: KeyEvent,
    ) -> eyre::Result<Option<AppAction>> {
        if key.kind == KeyEventKind::Press {
            return match key.code {
                KeyCode::Char('q') => Ok(Some(AppAction::Quit)),
                KeyCode::Char('?') => {
                    self.should_show = !self.should_show;
                    Ok(None)
                }
                _ => Ok(None),
            };
        }

        Ok(None)
    }

    fn render(&mut self, frame: &mut Frame, rect: Rect) -> eyre::Result<()> {
        let horizontal_center = Layout::default()
            .direction(Direction::Horizontal);

        let block = Block::default()
            .title("Keyboard shortcuts")
            .borders(Borders::ALL);

        if self.should_show {
            frame.render_widget(block, rect);
        }

        Ok(())
    }
}
