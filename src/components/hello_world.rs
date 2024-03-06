use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

use crate::component::Component;

#[derive(Default, Clone)]
pub struct HelloWorld {
    pub text: String,
}

impl Component for HelloWorld {
    fn render(&mut self, frame: &mut Frame, rect: Rect) -> eyre::Result<()> {
        frame.render_widget(Paragraph::new(self.text.clone()), rect);

        Ok(())
    }
}
