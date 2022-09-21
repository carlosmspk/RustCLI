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

#[cfg(test)]
mod tests {
    use super::UserInput;
    use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

    struct TestUtils;
    impl TestUtils {
        fn new_key_event_a() -> crossterm::event::Event {
            Event::Key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE))
        }
        fn new_key_event_a_capitalized_shift() -> crossterm::event::Event {
            Event::Key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::SHIFT))
        }
        fn new_key_event_a_capitalized() -> crossterm::event::Event {
            Event::Key(KeyEvent::new(KeyCode::Char('A'), KeyModifiers::NONE))
        }
        fn new_key_event_enter() -> crossterm::event::Event {
            Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE))
        }
        fn new_mouse_event() -> crossterm::event::Event {
            Event::Mouse(MouseEvent {
                column: 0,
                row: 0,
                kind: MouseEventKind::Moved,
                modifiers: KeyModifiers::NONE,
            })
        }
        fn new_misc_event() -> crossterm::event::Event {
            Event::FocusGained
        }
    }

    #[test]
    fn user_input_conversion() {
        let number_type = UserInput::from(2);
        let text_type = UserInput::from("I'm a Text type!");
        let key_type = UserInput::from(TestUtils::new_key_event_a());
        let mouse_type = UserInput::from(TestUtils::new_mouse_event());
        let misc_type = UserInput::from(TestUtils::new_misc_event());

        assert!(matches!(number_type, UserInput::Number(_)));
        assert!(matches!(text_type, UserInput::Text(_)));
        assert!(matches!(key_type, UserInput::KeyEvent(_)));
        assert!(matches!(mouse_type, UserInput::MouseEvent(_)));
        assert!(matches!(misc_type, UserInput::Other(_)));
    }

    #[test]
    fn user_input_is_enter() {
        let enter_input = UserInput::from(TestUtils::new_key_event_enter());
        assert!(enter_input.is_enter());
        let enter_input = UserInput::from(TestUtils::new_key_event_a());
        assert!(!enter_input.is_enter());
        let enter_input = UserInput::from(TestUtils::new_mouse_event());
        assert!(!enter_input.is_enter());
        let enter_input = UserInput::from("Enter");
        assert!(!enter_input.is_enter());
        let enter_input = UserInput::from(0);
        assert!(!enter_input.is_enter());
    }

    #[test]
    fn user_input_to_char() {
        let capitalized_a = UserInput::from(TestUtils::new_key_event_a_capitalized());
        let capitalized_a_as_char = capitalized_a.as_char();
        assert!(matches!(capitalized_a_as_char, Some(_)));
        let capitalized_a_as_char = capitalized_a_as_char.unwrap();
        assert_eq!(capitalized_a_as_char, 'A');

        let capitalized_a_shift =
            UserInput::from(TestUtils::new_key_event_a_capitalized_shift());
        let capitalized_a_shift_as_char = capitalized_a_shift.as_char();
        assert!(matches!(capitalized_a_shift_as_char, Some(_)));
        let capitalized_a_shift_as_char = capitalized_a_shift_as_char.unwrap();
        assert_eq!(capitalized_a_shift_as_char, 'A');

        let a =
            UserInput::from(TestUtils::new_key_event_a());
        let a_as_char = a.as_char();
        assert!(matches!(a_as_char, Some(_)));
        let a_as_char = a_as_char.unwrap();
        assert_eq!(a_as_char, 'a');

        let bad_input = UserInput::from("A");
        let bad_input = bad_input.as_char();
        assert!(matches!(bad_input, None));

        let bad_input = UserInput::from(2);
        let bad_input = bad_input.as_char();
        assert!(matches!(bad_input, None));

        let bad_input = UserInput::from(TestUtils::new_mouse_event());
        let bad_input = bad_input.as_char();
        assert!(matches!(bad_input, None));
    }
}
