use crate::{error::Error, input::UserInput, text::AnyString};

use super::{Screen, ScreenCommand};
pub struct SimpleQuery {
    message_prompt: AnyString,
    user_response_buffer: String,
}

impl SimpleQuery {
    pub fn new(message_prompt: AnyString) -> SimpleQuery {
        SimpleQuery {
            message_prompt,
            user_response_buffer: String::new(),
        }
    }
}

impl Screen for SimpleQuery {
    fn display(&self) -> Result<Vec<AnyString>, Error> {
        Ok(vec![self.message_prompt.clone(), self.user_response_buffer.clone().into()])
    }
    fn on_event(&mut self, input: UserInput) -> Result<ScreenCommand, Error> {
        let char_input_option = input.as_char();
        if let Some(input_char) = char_input_option {
            self.user_response_buffer.push(input_char);
            return Ok(ScreenCommand::Refresh)
        }
        if let UserInput::KeyEvent(key) = input {
            if let crossterm::event::KeyCode::Backspace = key.code {
                self.user_response_buffer.pop();
                return Ok(ScreenCommand::Refresh)
            }
        }
        if input.is_enter() {
            return Ok(ScreenCommand::Exit)
        }
        Ok(ScreenCommand::Idle)
    }

    fn on_screen_exit(&self) -> Option<UserInput> {
        Some(self.user_response_buffer.clone().into())
    }
}

