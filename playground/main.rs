use rust_cli::debug::string_from_event;
use rust_cli::error::Error;
use rust_cli::terminal;

fn read_input_and_print_loop() -> Result<(), Error> {
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

fn simple_terminal() -> Result<(), Error> {
    let mut term = rust_cli::terminal::Terminal::new();
    let screen = rust_cli::screens::SimpleQuery::new(String::from("Input: ").into());
    let screen = Box::new(screen);
    term.add_screen(screen)?;
    let input = term.read_sync()?;
    println!("{:?}", input);
    return Ok(());
}

fn main() -> Result<(), Error> {
    simple_terminal()
}
