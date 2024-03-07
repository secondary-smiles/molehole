use crossterm::event::{KeyEvent, KeyEventKind};
use ratatui::prelude::{
    Alignment, Color, Constraint, Direction, Frame, Layout, Line, Margin, Rect,
    Span, Style, Stylize,
};
use ratatui::widgets::block::{Block, BorderType, Title};
use ratatui::widgets::{
    Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
    Wrap,
};

use crate::app_action::AppAction;
use crate::component::Component;
use crate::keys::key_commands::{serialize_key_event, KeyCommand};

#[derive(Default)]
pub struct GlobalKeys {
    pub key_commands: Vec<KeyCommand>,

    pub should_show: bool,
    pub scroll: usize,
    pub scroll_state: ScrollbarState,
}

impl Component for GlobalKeys {
    fn init(&mut self) -> eyre::Result<()> {
        self.key_commands.append(&mut vec![KeyCommand {
            key_code: "?".to_string(),
            description: "Toggle help menu".to_string(),
            action: None,
        }]);

        self.scroll_state =
            ScrollbarState::new(self.key_commands.len()).position(self.scroll);

        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key: KeyEvent,
    ) -> eyre::Result<Option<AppAction>> {
        if key.kind == KeyEventKind::Press {
            let key_event = serialize_key_event(key);
            let eat_input = match key_event.as_str() {
                "?" => {
                    self.should_show = !self.should_show;
                    self.scroll = 0;
                    true
                }
                "g" => {
                    self.scroll = 0;
                    true
                }
                "G" => {
                    self.scroll = self.key_commands.len() - 1;
                    true
                }
                "down" | "j" => {
                    if self.scroll < self.key_commands.len() - 1 {
                        self.scroll += 1;
                    }
                    true
                }
                "up" | "k" => {
                    if self.scroll > 0 {
                        self.scroll -= 1;
                    }
                    true
                }
                _ => false,
            };
            self.scroll_state = self.scroll_state.position(self.scroll);
            if eat_input && self.should_show {
                return Ok(None);
            }

            for key_command in &mut self.key_commands {
                if key_command.key_code == key_event {
                    return Ok(key_command.action);
                }
            }
        }

        Ok(None)
    }

    fn render(&mut self, frame: &mut Frame, rect: Rect) -> eyre::Result<()> {
        let vertical_center = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50 / 2),
                Constraint::Percentage(50),
                Constraint::Percentage(50 / 2),
            ])
            .split(rect);
        let center = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50 / 2),
                Constraint::Percentage(50),
                Constraint::Percentage(50 / 2),
            ])
            .split(vertical_center[1])[1];

        let block = Block::default()
            .title(
                Title::from("Keyboard shortcuts").alignment(Alignment::Center),
            )
            .borders(Borders::ALL)
            .border_type(BorderType::Thick);

        let mut lines: Vec<Line> = vec![];
        for key_command in &mut self.key_commands {
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
            .scroll((u16::try_from(self.scroll)?, 0))
            .style(Style::default().bg(Color::DarkGray).fg(Color::White));

        if self.should_show {
            frame.render_widget(Clear, center);
            frame.render_widget(commands, center);
            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::VerticalRight),
                center.inner(&Margin {
                    vertical: 1,
                    horizontal: 0,
                }),
                &mut self.scroll_state,
            );
        }

        Ok(())
    }
}
