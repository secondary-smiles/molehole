use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

use crate::app_action::AppAction;
use crate::component::Component;
use crate::keys::key_commands::*;

#[derive(Default, Clone)]
pub struct HelloWorld {
    pub text: String,
}

impl Component for HelloWorld {
    fn init(&mut self) -> eyre::Result<()> {
        self.text = "Hello, world!".to_string();
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) -> eyre::Result<Option<AppAction>> {
        self.text = serialize_key_event(key);
        Ok(None)
    }

    fn render(&mut self, frame: &mut Frame, rect: Rect) -> eyre::Result<()> {

        frame.render_widget(Paragraph::new(self.text.clone()), rect);

        Ok(())
    }
}
