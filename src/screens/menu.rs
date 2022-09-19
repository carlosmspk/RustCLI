use crate::{cli_display::Displayable, error::Error, text::AnyString};

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

impl Displayable for Menu {
    fn display(&self) -> Result<Vec<AnyString>, Error> {
        Ok(self.options.clone())
    }
}
