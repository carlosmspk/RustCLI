use crossterm::{
    execute,
    terminal::{Clear, ClearType},
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
    }
}
