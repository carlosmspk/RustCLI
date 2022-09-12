use crate::error::Error;
use std::io::Write;

use crossterm::{
    self,
    terminal::{Clear, ClearType},
    QueueableCommand,
};

pub struct Terminal {
    stdout: std::io::Stdout,
}

impl Terminal {
    /// Create a new Terminal instance. This does not open a new terminal but rather creates a struct that holds the current terminal's data, such as `Stdout`, required for future method calls.
    pub fn new() -> Terminal {
        Terminal {
            stdout: std::io::stdout(),
        }
    }
    /// Clears the terminal.
    /// ## Returns
    /// `Ok(())` if the read was successful. Returns `Err(Error)` if something went wrong, such as stdout being absent.
    pub fn clear(&mut self) -> Result<(), Error> {
        self.stdout.queue(Clear(ClearType::All))?;
        self.flush()
    }
    /// Flushes stdout with all buffered data. Most of the time, there is no need to call this method, as the `Terminal` struct will call it when required.
    /// ## Returns
    /// `Ok(())` if flush was successful. Returns `Err(Error)` if something went wrong, such as stdout being absent.
    pub fn flush(&mut self) -> Result<(), Error> {
        self.stdout.flush()?;
        Ok(())
    }
    /// Reads input from the keyboard, blocking indefinitely until input is available. Returns the read event wrapped in Result.
    /// ## Returns
    /// `Ok(Event)` if the read was successful. Returns `Err(Error)` if something went wrong, such as stdout being absent.
    pub fn read_sync(&self) -> Result<crossterm::event::Event, Error> {
        let event = crossterm::event::read()?;
        Ok(event)
    }

    /// Reads input from the keyboard, blocking for a given amount of time provided by `timeout`. If `timeout` is `None`, then the function immediately returns if no input is available. Regardless, a result with the optional value is returned (as there may be no input available within the timeout frame).
    /// ## Returns
    /// `Ok()` wrapping `Some(Event)` if some input was available, otherwise returns `Ok(None)`. Returns `Err(Error)` if something went wrong, such as stdout being absent.
    /// ## Arguments
    /// `timeout`: The amount of time to wait for input. If some input is available before the time provided, the function will exit early returning the input.
    pub fn read_async(
        &self,
        timeout: Option<std::time::Duration>,
    ) -> Result<Option<crossterm::event::Event>, Error> {
        let timeout = timeout.unwrap_or(std::time::Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            let event = crossterm::event::read()?;
            return Ok(Some(event));
        }
        return Ok(None);
    }
}
