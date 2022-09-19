use crate::error::Error;
use crate::text::AnyString;

pub trait Displayable {
    fn display(&self) -> Result<Vec<AnyString>, Error>;
}
