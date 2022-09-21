pub mod option_list;
pub mod simple_query;
pub use option_list::OptionList;
pub use simple_query::SimpleQuery;

use crate::error::Error;
use crate::input::UserInput;
use crate::text::AnyString;

pub trait Screen {
    fn display(&self) -> Result<Vec<AnyString>, Error>;
    fn on_event(&mut self, input: UserInput) -> Result<bool, Error>;
    fn on_screen_exit(self) -> Option<UserInput>;
}
