use crate::{cli_display::Displayable, error::Error};
use std::io::Write;

use crossterm::{
    self,
    terminal::{Clear, ClearType},
    QueueableCommand,
};

/// Struct that holds relevant info regarding the process's current
/// stdout and terminal. Can display many template CLI screens such as
/// multiple choice menus and listings, progress screens, simples text
/// inputs, etc. Also provides means to extract single key inputs from
/// the user.
pub struct Terminal {
    stdout: std::io::Stdout,
    screen_stack: Vec<Box<dyn Displayable>>,
}

impl Terminal {
    /// Create a new Terminal instance. This does not open a new terminal but rather creates a struct that holds the current terminal's data, such as `Stdout`, required for future method calls.
    pub fn new() -> Terminal {
        Terminal {
            stdout: std::io::stdout(),
            screen_stack: Vec::new(),
        }
    }

    pub fn add_screen(&mut self, screen: Box<dyn Displayable>) -> Result<(), Error>{
        self.screen_stack.push(screen);
        let content_to_display = self.screen_stack.get(self.screen_stack.len()-1);
        if let None = content_to_display {
            return Err(Error::TerminalScreenStackEmpty)
        }
        let content_to_display = content_to_display.unwrap().display()?;
        self.clear()?;
        for string in content_to_display {
            println!("{}", string)
        }
        self.flush()?;
        return Ok(())
    }

    /// Clears the terminal.
    /// ## Returns
    /// `Ok(())` if the clear was successful. Returns `Err(Error)` if something went wrong, such as stdout being absent.
    pub fn clear(&mut self) -> Result<(), Error> {
        self.stdout.queue(Clear(ClearType::All))?;
        self.flush()?;
        return Ok(())
    }

    /// Flushes stdout with all buffered data. Most of the time, there is no need to call this method, as the `Terminal` struct will call it when required.
    /// ## Returns
    /// `Ok(())` if flush was successful. Returns `Err(Error)` if something went wrong, such as stdout being absent.
    pub fn flush(&mut self) -> Result<(), Error> {
        self.stdout.flush()?;
        return Ok(())
    }

    /// Reads input from the keyboard, blocking indefinitely until input is available. Returns the read event wrapped in Result.
    /// Most times you'll want to use one of the input screens to gather user info, displaying some sort of prompt message, or
    /// multiple choices from a list to pick from. This function will directly read a single event, without updating the current screen.
    /// ## Returns
    /// `Ok(Event)` if the read was successful. Returns `Err(Error)` if something went wrong, such as stdout being absent.
    pub fn read_sync(&self) -> Result<crossterm::event::Event, Error> {
        let event = crossterm::event::read()?;
        return Ok(event)
    }

    /// Reads input from the keyboard, blocking for a given amount of time provided by `timeout`. If `timeout` is `None`, then the function immediately returns if no input is available. Regardless, a result with the optional value is returned (as there may be no input available within the timeout frame).
    /// Most times you'll want to use one of the input screens to gather user info, displaying some sort of prompt message, or
    /// multiple choices from a list to pick from. This function will directly read a single event, without updating the current screen.
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
