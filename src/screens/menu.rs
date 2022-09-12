use crate::{
    cli_display::{CLIScreen, Error},
    text::AnyString,
};

pub struct Menu {
    options: Vec<AnyString>,
}

impl Menu {
    pub fn new(options: Vec<AnyString>) -> Result<Self, Error> {
        if options.len() == 0 {
            return Err(Error::ItemlessMenuError);
        }
        Ok(Self { options })
    }
}

impl CLIScreen for Menu {
    fn display(&self) -> Result<Vec<AnyString>, Error> {
        Ok(self.options.clone())
    }
}
