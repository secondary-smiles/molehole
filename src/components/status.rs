use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::app_action::AppAction;
use crate::app_event::AppEvent;
use crate::component::Component;
use crate::keys::key_commands::serialize_key_event;

#[derive(Default, Clone)]
pub struct StatusBar {
    pub message: String,
    pub current_key: String,
    pub error: bool,
    pub url_to_open: Option<url::Url>,
}

impl Component for StatusBar {
    fn handle_key_event(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) -> Option<AppAction> {
        let key_str = serialize_key_event(key);
        self.current_key = key_str;

        None
    }

    fn handle_action(&mut self, action: crate::app_action::AppAction) {
        match action.clone() {
            AppAction::StatusBarSetMessage(message) => {
                self.error = false;
                self.message = message;
            }
            AppAction::StatusBarSetError(message) => {
                self.error = true;
                self.message = message;
            }
            AppAction::OpenUrl => {
                self.url_to_open =
                    Some(url::Url::parse("molerat://example.com").unwrap());
            }
            _ => {}
        }
    }

    fn update(&mut self) -> Option<AppEvent> {
        if let Some(url) = &self.url_to_open {
            let event = AppEvent::OpenUrl(url.clone());
            self.url_to_open = None;
            return Some(event);
        }

        None
    }

    fn render(
        &mut self,
        frame: &mut ratatui::prelude::Frame,
        rect: ratatui::prelude::Rect,
    ) -> eyre::Result<()> {
        let block =
            Block::default().style(Style::default().bg(if self.error {
                Color::Red
            } else {
                Color::DarkGray
            }));

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(rect);

        let message = Paragraph::new(self.message.clone()).block(block.clone());
        let current_key = Paragraph::new(self.current_key.clone())
            .block(block)
            .alignment(Alignment::Right);

        frame.render_widget(message, layout[0]);
        frame.render_widget(current_key, layout[1]);

        Ok(())
    }
}
