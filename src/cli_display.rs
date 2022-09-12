use crate::error::Error;
use crate::text::AnyString;

pub trait CLIScreen {
    fn display(&self) -> Result<Vec<AnyString>, Error>;
    // fn handle_input(&self, input) -> Result<bool, Error>;
}
