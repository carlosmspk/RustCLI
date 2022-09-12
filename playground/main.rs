use rust_cli::error::Error;
use rust_cli::terminal;
use rust_cli::debug::string_from_event;

fn main() -> Result<(), Error> {
    let mut term = terminal::Terminal::new();
    println!("Press any keys (q to exit)...");
    loop {
        let event = term.read_sync()?;
        let event_as_string = string_from_event(&event);
        print!("{} ", &event_as_string);
        term.flush()?;
        if event_as_string == "q" {
            return Ok(());
        }
    }
}
