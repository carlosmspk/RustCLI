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

impl From<String> for UserInput {
    fn from(string: String) -> Self {
        UserInput::Text(string)
    }
}

impl From<&str> for UserInput {
    fn from(string: &str) -> Self {
        UserInput::Text(String::from(string))
    }
}

impl From<i64> for UserInput {
    fn from(number: i64) -> Self {
        UserInput::Number(number)
    }
}

impl From<crossterm::event::Event> for UserInput {
    fn from(event: crossterm::event::Event) -> Self {
        match event {
            crossterm::event::Event::Key(key_event) => UserInput::KeyEvent(key_event),
            crossterm::event::Event::Mouse(mouse_event) => UserInput::MouseEvent(mouse_event),
            _ => UserInput::Other(event),
        }
    }
}

pub trait UserInteractable {
    fn on_event(&mut self, input: UserInput) -> Result<bool, Error>;
    fn on_screen_exit(self) -> Option<UserInput>;
}
