use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app_action::AppAction;

#[derive(Default, Clone)]
pub struct KeyCommand {
    pub key_code: String,
    pub description: String,
    pub action: AppAction,
}

impl std::fmt::Display for KeyCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\t{}", self.key_code, self.description)
    }
}

pub fn serialize_key_event(event: KeyEvent) -> String {
    let mut modifiers = Vec::with_capacity(3);
    if event.modifiers.intersects(KeyModifiers::CONTROL) {
        modifiers.push("ctrl");
    }
    if event.modifiers.intersects(KeyModifiers::SUPER)
        || event.modifiers.intersects(KeyModifiers::HYPER)
        || event.modifiers.intersects(KeyModifiers::META)
    {
        modifiers.push("super");
    }
    if event.modifiers.intersects(KeyModifiers::ALT) {
        modifiers.push("alt");
    }

    let char;
    let key = match event.code {
        KeyCode::Backspace | KeyCode::Delete => "del",
        KeyCode::Enter => "enter",
        KeyCode::Left => "left",
        KeyCode::Right => "right",
        KeyCode::Up => "up",
        KeyCode::Down => "down",
        KeyCode::Tab => "tab",
        KeyCode::Char(' ') => "space",
        KeyCode::Char(c) => {
            char = c.to_string();
            &char
        }
        KeyCode::Esc => "esc",
        _ => "",
    };
    let separator = if modifiers.is_empty() { "" } else { "-" };
    let serialized_event =
        format!("{}{}{}", modifiers.join("-"), separator, key);

    serialized_event
}
