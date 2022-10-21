use crossterm::event::{KeyCode, KeyEventKind};

use crate::{error::Error, text::AnyString};

use super::{Screen, ScreenCommand};

pub struct OptionsList {
    options: Vec<AnyString>,
    enumerate: bool,
    currently_selected: usize,
}

impl OptionsList {
    pub fn from_any_string(options: Vec<AnyString>) -> Result<Self, Error> {
        if options.len() == 0 {
            return Err(Error::ItemlessOptionListError);
        }
        Ok(Self {
            options,
            enumerate: false,
            currently_selected: 0,
        })
    }
    pub fn from_string(options: Vec<String>) -> Result<Self, Error> {
        if options.len() == 0 {
            return Err(Error::ItemlessOptionListError);
        }
        let casted_options = options.iter().map(|e| e.into()).collect();
        Ok(Self {
            options: casted_options,
            enumerate: false,
            currently_selected: 0,
        })
    }
    pub fn from<T>(options: Vec<T>) -> Result<Self, Error> where T:ToString {
        if options.len() == 0 {
            return Err(Error::ItemlessOptionListError);
        }
        let casted_options = options.iter().map(|e| e.to_string().into()).collect();
        Ok(Self {
            options: casted_options,
            enumerate: false,
            currently_selected: 0,
        })
    }
    pub fn enumerate_items(mut self) -> Self {
        self.enumerate=true;
        self
    }
}

impl Screen for OptionsList {
    fn display(&self) -> Result<Vec<AnyString>, Error> {
        Ok(self.options.clone())
    }

    fn on_event(&mut self, input: crate::input::UserInput) -> Result<ScreenCommand, Error> {
        enum MoveCommand{
            UP,
            DOWN
        };
        Ok(ScreenCommand::Idle)
    }

    fn on_screen_exit(&self) -> Option<crate::input::UserInput> {
        todo!()
    }
}
