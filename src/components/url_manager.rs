use url::Url;

use crate::app_action::AppAction;
use crate::app_event::AppEvent;
use crate::component::Component;

#[derive(Default)]
pub struct UrlManager {
    url: Option<Url>,
}

impl Component for UrlManager {
    fn handle_event(&mut self, event: AppEvent) -> Option<AppAction> {
        match event {
            AppEvent::OpenUrl(url) => {
                self.url = Some(url.clone());
                return Some(AppAction::StatusBarSetMessage(format!(
                    "Opening {}",
                    url.as_str()
                )));
            }
            _ => {}
        }
        None
    }

    fn render(
        &mut self,
        _frame: &mut ratatui::prelude::Frame,
        _rect: ratatui::prelude::Rect,
    ) -> eyre::Result<()> {
        Ok(())
    }
}
