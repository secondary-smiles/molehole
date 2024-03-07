use ratatui::prelude::{Frame, Rect};
use ratatui::widgets::{Paragraph, Wrap};

use crate::component::Component;

#[derive(Default, Clone)]
pub struct HelloWorld {
    pub text: String,
}

impl Component for HelloWorld {
    fn init(&mut self) -> eyre::Result<()> {
        self.text = "Hello, world!".to_string();
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, rect: Rect) -> eyre::Result<()> {
        frame.render_widget(
            Paragraph::new(self.text.clone()).wrap(Wrap { trim: true }),
            rect,
        );

        Ok(())
    }
}
