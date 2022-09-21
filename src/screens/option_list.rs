use crate::{cli_display::Displayable, error::Error, text::AnyString};

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

impl Displayable for OptionList {
    fn display(&self) -> Result<Vec<AnyString>, Error> {
        Ok(self.options.clone())
    }
}
