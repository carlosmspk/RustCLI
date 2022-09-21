use crate::{error::Error, text::AnyString};

use super::Screen;

pub struct OptionList {
    options: Vec<AnyString>,
}

impl OptionList {
    pub fn new(options: Vec<AnyString>) -> Result<Self, Error> {
        if options.len() == 0 {
            return Err(Error::ItemlessOptionListError);
        }
        Ok(Self { options })
    }
}

impl Screen for OptionList {
    fn display(&self) -> Result<Vec<AnyString>, Error> {
        Ok(self.options.clone())
    }

    fn on_event(&mut self, input: crate::input::UserInput) -> Result<bool, Error> {
        todo!()
    }

    fn on_screen_exit(self) -> Option<crate::input::UserInput> {
        todo!()
    }
}
