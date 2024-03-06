mod action;
mod app;
mod app_event;
mod component;
mod tui;

use eyre::Result;

fn main() -> Result<()> {
    let mut app = app::App::start(std::time::Duration::from_millis(10))?;

    app.quit()
}
