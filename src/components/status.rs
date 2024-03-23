use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::app_action::AppAction;
use crate::component::Component;
use crate::keys::key_commands::serialize_key_event;

#[derive(Default, Clone)]
pub struct StatusBar {
    message: String,
    current_key: String,
    error: bool,
}

impl Component for StatusBar {
    fn handle_key_event(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) -> eyre::Result<Option<AppAction>> {
        let key_str = serialize_key_event(key);
        self.current_key = key_str;

        Ok(None)
    }

    fn handle_action(&mut self, action: crate::app_action::AppAction) {
        match action {
            AppAction::StatusBarSetMessage(message) => {
                self.error = false;
                self.message = message;
            }
            AppAction::StatusBarSetError(message) => {
                self.error = true;
                self.message = message;
            }
            AppAction::StatusBarGetInput(_prompt) => todo!(),
            _ => {
                self.current_key += " ";
                self.current_key += &action.to_string();
            }
        }
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
