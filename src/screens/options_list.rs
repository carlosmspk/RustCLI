use crate::{error::Error, text::AnyString};

use super::{Screen, ScreenCommand};

pub struct OptionsList {
    options: Vec<AnyString>,
}

impl OptionsList {
    pub fn new(options: Vec<AnyString>) -> Result<Self, Error> {
        if options.len() == 0 {
            return Err(Error::ItemlessOptionListError);
        }
        Ok(Self { options })
    }
}

impl Screen for OptionsList {
    fn display(&self) -> Result<Vec<AnyString>, Error> {
        Ok(self.options.clone())
    }

    fn on_event(&mut self, input: crate::input::UserInput) -> Result<ScreenCommand, Error> {
        todo!()
    }

    fn on_screen_exit(&self) -> Option<crate::input::UserInput> {
        todo!()
    }
}
