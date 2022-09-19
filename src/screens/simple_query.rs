use crate::{cli_display::Displayable, error::Error, input::{UserInteractable, UserInput}, text::AnyString};
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

impl Displayable for SimpleQuery {
    fn display(&self) -> Result<Vec<AnyString>, Error> {
        Ok(vec![self.message_prompt.clone()])
    }
}

impl UserInteractable for SimpleQuery {
    fn on_event(&mut self, input: UserInput) -> Result<bool, Error> {
        let char_input_option = input.as_char();
        if let Some(input_char) = char_input_option {
            self.user_response_buffer.push(input_char);
            return Ok(false)
        } else if input.is_enter() {
            return Ok(true)
        }
        Ok(false)
    }

    fn on_screen_exit(self) -> Option<UserInput> {
        Some(UserInput::from(self.user_response_buffer))
    }
}
