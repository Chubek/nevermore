use std::io::{self, Read, Write, stdin, stdout};
use std::fs;
use crossterm::{QueueableCommand, Command, cursor, execute, queue, style, event, terminal};
use clap::Parser;

fn draw_frame<W: Write>(out: &mut W, lines: &[&str], scroll: usize, rows: usize) -> io::Result<()> {
    out.queue(cursor::MoveTo(0, 0))?;
    out.queue(terminal::Clear(terminal::ClearType::All))?;

    for (i, line) in lines.iter()
            .skip(scroll)
            .take(rows)
            .enumerate() {
                out.queue(cursor::MoveTo(0, i as u16))?;
                out.queue(style::Print(line))?;
            }

    out.flush();
    Ok(())
        
}

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

    let lines: Vec<&str> = input.lines().collect();

    terminal::enable_raw_mode()?;
    execute!(stdout, 
        terminal::EnterAlternateScreen,
        cursor::Hide,
        cursor::MoveTo(0, 0))?;

    let (cols, rows) = terminal::size()?;
    let max_scroll = lines.len().saturating_sub(rows as usize);
    let mut scroll = 0;

    draw_frame(&mut stdout, &lines, scroll, rows as usize)?;

'tl: loop {
        match event::read()? {
            event::Event::Key(ev) => { 
                use crossterm::event::{KeyEventKind, KeyCode, KeyModifiers};

                match ev.code {
                    KeyCode::Char('q') => break 'tl,
                    KeyCode::Up | KeyCode::Char('k') => {
                        if scroll > 0 {
                            scroll -= 1;
                            draw_frame(&mut stdout, &lines, scroll, rows as usize)?;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if scroll < max_scroll {
                            scroll += 1;
                            draw_frame(&mut stdout, &lines, scroll, rows as usize);
                        }
                    }
                    _ => {}
                }
            },
            _ => break,
        }
    }

    execute!(stdout,
        cursor::Show,
        terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    stdout.flush();
    Ok(())
}
