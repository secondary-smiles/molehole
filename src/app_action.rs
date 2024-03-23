use std::fmt;

#[derive(Default, Clone, Debug)]
pub enum AppAction {
    StatusBarGetInput(String),
    StatusBarSetMessage(String),
    StatusBarSetError(String),
    OpenUrl,

    ScrollUp,
    ScrollDown,
    ScrollTop,
    ScrollBottom,

    ShowHelpMenu,

    #[default]
    Quit,
}

impl fmt::Display for AppAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
