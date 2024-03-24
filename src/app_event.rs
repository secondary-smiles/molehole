use crossterm::event::{KeyEvent, MouseEvent};
use url::Url;

#[derive(Clone)]
pub enum AppEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),

    OpenUrl(Url),
}
