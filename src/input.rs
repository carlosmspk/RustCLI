use crate::error::Error;

pub enum UserInput {
    Number(i64),
    Text(String),
    KeyPress(crossterm::event::KeyCode),
    Mouse(crossterm::event::MouseEventKind),
    Other(crossterm::event::Event),
}

impl UserInput {
    pub fn as_char(&self) -> Option<char> {
        if let Self::KeyPress(code) = self {
            if let crossterm::event::KeyCode::Char(c) = code {
                return Some(*c)
            }
        }
        None
    }
    pub fn is_enter(&self) -> bool {
        if let UserInput::KeyPress(code) = self {
            if let crossterm::event::KeyCode::Enter = code {
                return true
            }
        }
        return false
    }
}


pub trait UserInteractable {
    fn on_event(&mut self, input: UserInput) -> Result<bool, Error>;
    fn on_screen_exit(self) -> Option<UserInput>;
}
