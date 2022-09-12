use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};

use crate::cli_display::Error;

pub fn clear() -> Result<(), Error>{
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    Ok(())
}
