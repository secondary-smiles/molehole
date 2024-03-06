use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::app_action::AppAction;
use crate::component::Component;
use crate::keys::key_commands::*;

#[derive(Default)]
pub struct GlobalKeys {
    pub key_commands: Vec<KeyCommand>,

    pub should_show: bool,
    pub scroll: u16,
}

impl Component for GlobalKeys {
    fn init(&mut self) -> eyre::Result<()> {
        self.key_commands.push(KeyCommand {
            key_code: "?".to_string(),
            description: "Show help menu".to_string(),
            action: None,
        });

        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key: KeyEvent,
    ) -> eyre::Result<Option<AppAction>> {
        if key.kind == KeyEventKind::Press {
            for key_command in self.key_commands.iter_mut() {
                if key_command.key_code == serialize_key_event(key) {
                    if serialize_key_event(key) == "?" {
                        self.should_show = !self.should_show;
                    }

                    return Ok(key_command.action);
                }
            }
        }

        Ok(None)
    }

    fn render(&mut self, frame: &mut Frame, rect: Rect) -> eyre::Result<()> {
        let block = Block::default()
            .title("Keyboard shortcuts")
            .borders(Borders::ALL);

        let mut lines: Vec<Line> = vec![];
        for key_command in self.key_commands.iter_mut() {
            let command = Span::from(key_command.key_code.clone());
            let description =
                Span::from(key_command.description.clone()).italic();
            let spacer = Span::from("  ");

            let line = Line::from(vec![command, spacer, description]);
            lines.push(line);
        }

        let commands = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: true })
            .scroll((self.scroll, 0));

        if self.should_show {
            frame.render_widget(commands, rect);
        }

        Ok(())
    }
}
