pub mod option_list;
pub mod simple_query;
pub use option_list::OptionList;
pub use simple_query::SimpleQuery;

use crate::error::Error;
use crate::input::UserInput;
use crate::text::AnyString;

pub trait Screen {
    /// Provides with the contents to display in the terminal. Empty arrays, for no content to print, are valid. Strings will be printed
    /// back t back (newlines need to be added manually, i.e., by the screen implementation).
    /// ## Returns
    /// `Ok()` containing an array with AnyString whose contents will be printed as a buffer (newlines need to be added manually, i.e.,
    /// by the screen implementation). If anything goes wrong, return Err(rust_cli::error::Error)
    fn display(&self) -> Result<Vec<AnyString>, Error>;
    
    /// Sends any input the user may have provided, leaving the Screen to decide on wheather that input is valid, or when all required input was received and the screen is ready to be popped.
    /// ## Returns
    /// `Ok(bool)` if this particular event was successfully interpreted. The screen should return `Ok(true)` when it is no longer expecting input (for instance, when an ENTER key was pressed, indicating that all input up until this point is ready to be sent). An `Ok(false)` means valid input, but screen expects further input. If anything goes wrong, return Err(rust_cli::error::Error)
    /// ## Arguments
    /// `input`: input to be interpreted by the screen struct. Mostly UserInput will be of any Event-like form.
    fn on_event(&mut self, input: UserInput) -> Result<bool, Error>;
    
    /// Notified the screen that it is about to be popped. Most screens will provide with `Some(UserInput)` as a response, but some screens, such as loading screens, can just return `None`
    fn on_screen_exit(self) -> Option<UserInput>;
}
