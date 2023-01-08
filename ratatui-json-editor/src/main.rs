use crossterm::event::EnableMouseCapture;
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    // --snip--