mod app;
mod app_action;
mod app_event;
mod component;
mod components;
mod keys;
mod tui;

use eyre::Result;

use app_action::AppAction;
use keys::key_commands::KeyCommand;

fn main() -> Result<()> {
    tui::install_hooks()?;
    let mut app = app::App::new(std::time::Duration::from_millis(10))?;
    let mut key_commands = vec![
        // Status bar
        KeyCommand {
            key_code: "o".to_string(),
            description: "Open new link".to_string(),
            action: AppAction::OpenUrl,
        },
        // Navigation
        KeyCommand {
            key_code: "g".to_string(),
            description: "Scroll to top".to_string(),
            action: AppAction::ScrollTop,
        },
        KeyCommand {
            key_code: "G".to_string(),
            description: "Scroll to bottom".to_string(),
            action: AppAction::ScrollBottom,
        },
        KeyCommand {
            key_code: "k".to_string(),
            description: "Scroll up one line".to_string(),
            action: AppAction::ScrollUp,
        },
        KeyCommand {
            key_code: "j".to_string(),
            description: "Scroll down one line".to_string(),
            action: AppAction::ScrollDown,
        },
        KeyCommand {
            key_code: "q".to_string(),
            description: "Quit molehole".to_string(),
            action: AppAction::Quit,
        },
        KeyCommand {
            key_code: "?".to_string(),
            description: "Toggle help menu".to_string(),
            action: AppAction::ShowHelpMenu,
        },
    ];
    app.key_commands.append(&mut key_commands);

    app.run()
}
