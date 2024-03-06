mod app;
mod app_action;
mod app_event;
mod component;
mod components;
mod tui;

use eyre::Result;

fn main() -> Result<()> {
    let mut app = app::App::new(std::time::Duration::from_millis(10))?;
    app.run()?;

    app.quit()
}
