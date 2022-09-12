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
    pub fn new() -> Terminal {
        Terminal {
            stdout: std::io::stdout(),
        }
    }
    pub fn clear(&mut self) -> Result<(), Error> {
        self.stdout.queue(Clear(ClearType::All))?;
        self.flush()
    }
    pub fn flush(&mut self) -> Result<(), Error> {
        self.stdout.flush()?;
    Ok(())
    }
    pub fn read_sync(&self) -> Result<crossterm::event::Event, Error> {
        let event = crossterm::event::read()?;
        Ok(event)
    }
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
