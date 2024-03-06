use crossterm::event::{KeyEvent, MouseEvent};

#[derive(Clone, Copy)]
pub enum AppEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
}
