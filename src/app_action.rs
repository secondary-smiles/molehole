#[derive(Default, Clone)]
pub enum AppAction {
    StatusBarMessage(String),
    StatusBarError(String),
    StatusBarInput(String),
    #[default]
    Quit,
}
