use crate::error::Error;

pub enum UserInput {
    Number(i64),
    Text(String),
    KeyEvent(crossterm::event::KeyEvent),
    MouseEvent(crossterm::event::MouseEvent),
    Other(crossterm::event::Event),
}

impl UserInput {
    pub fn as_char(&self) -> Option<char> {
        if let Self::KeyEvent(key_event) = self {
            if let crossterm::event::KeyCode::Char(char) = key_event.code {
                if key_event.modifiers.contains(crossterm::event::KeyModifiers::SHIFT) {
                    return Some(char.to_ascii_uppercase());
                }
                return Some(char);
            }
        }
        return None;
    }
    pub fn is_enter(&self) -> bool {
        if let UserInput::KeyEvent(code) = self {
            if let crossterm::event::KeyCode::Enter = code.code {
                return true;
            }
        }
        return false;
    }
}


pub trait UserInteractable {
    fn on_event(&mut self, input: UserInput) -> Result<bool, Error>;
    fn on_screen_exit(self) -> Option<UserInput>;
}
