use clap::Parser;
use crossterm::{cursor, event, execute, style, terminal, QueueableCommand};
use std::fs;
use std::io::{self, stdin, stdout, Stdout, Read, Write};

fn draw_frame(out: &mut Stdout, lines: &[&str], scroll: usize, rows: usize) -> io::Result<()> {
    out.queue(cursor::MoveTo(0, 0))?;
    out.queue(terminal::Clear(terminal::ClearType::All))?;

    for (i, line) in lines.iter().skip(scroll).take(rows).enumerate() {
        out.queue(cursor::MoveTo(0, i as u16))?;
        out.queue(style::Print(line))?;
    }

    out.flush()?;
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
        fs::read_to_string(file_path)?
    } else {
        let mut buffer = String::new();
        stdin().read_to_string(&mut buffer)?;
        buffer
    };

    let lines: Vec<&str> = input.lines().collect();

    terminal::enable_raw_mode()?;
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide,
        cursor::MoveTo(0, 0)
    )?;

    let (_, h0) = terminal::size()?;
    let rows = h0 as usize;
    let max_scroll = lines.len().saturating_sub(rows);
    let mut scroll = 0;

    draw_frame(&mut stdout, &lines, scroll, rows)?;

    let get_jump = |rows: usize| rows.saturating_sub(1);

    'tl: loop {
        match event::read()? {
            event::Event::Key(ev) => {
                use crossterm::event::KeyCode;

                match ev.code {
                    KeyCode::Char('q') => break 'tl,
                    KeyCode::Up | KeyCode::Char('k') => {
                        if scroll > 0 {
                            scroll -= 1;
                            draw_frame(&mut stdout, &lines, scroll, rows)?;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if scroll < max_scroll {
                            scroll += 1;
                            draw_frame(&mut stdout, &lines, scroll, rows)?;
                        }
                    }
                    KeyCode::Home => {
                        scroll = 0;
                        draw_frame(&mut stdout, &lines, scroll, rows)?;
                    }
                    KeyCode::End => {
                        scroll = max_scroll;
                        draw_frame(&mut stdout, &lines, scroll, rows)?;
                    }
                    KeyCode::PageUp => {
                        let jump = get_jump(rows);
                        if scroll <= jump {
                            scroll = 0;
                        } else {
                            scroll -= jump;
                        }
                        draw_frame(&mut stdout, &lines, scroll, rows)?;
                    }
                    KeyCode::PageDown => {
                        let jump = get_jump(rows);
                        scroll = (scroll + jump).min(max_scroll);
                        draw_frame(&mut stdout, &lines, scroll, rows)?;
                    }
                    _ => {}
                }
            }
            _ => break,
        }
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    stdout.flush()?;
    Ok(())
}
