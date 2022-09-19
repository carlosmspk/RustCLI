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

