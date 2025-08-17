use std::io::{self, Read, Write, stdin, stdout};
use std::fs;
use crossterm::{QueueableCommand, cursor, execute, queue, style, event, terminal};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    file: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let mut stdout = stdout();


    let input = if let Some(file_path) = args.file {
        fs::read_to_string(file_path).unwrap()
    } else {
        let mut buffer = String::new();
        stdin().read_to_string(&mut buffer);
        buffer
    };

    terminal::enable_raw_mode()?;
    execute!(stdout, 
        terminal::EnterAlternateScreen,
        cursor::MoveTo(0, 0))?;

    for line in input.lines() {
        stdout.queue(style::Print(line));
        stdout.queue(style::Print("\r\n"));
    }

    loop {
        match event::read()? {
            event::Event::Key(ev) => { println!("{:?}", ev); break; },
            _ => break,
        }
    }

    execute!(stdout, terminal::LeaveAlternateScreen);
    terminal::disable_raw_mode()?;

    stdout.flush();
    Ok(())
}
