use std::{io::{self, stdout, Write}, time::Duration};

use crossterm::{
    cursor::MoveToColumn,
    event::{poll, read, Event, KeyCode},
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
    QueueableCommand
};

fn main() -> io::Result<()> {
    let mut stdout = stdout();
    let mut nr: usize = 0;

    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Clear(ClearType::Purge))?;
    enable_raw_mode()?;

    loop {
        nr += 1;
        stdout.queue(MoveToColumn(0))?;
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(event) => {
                    if event.code == KeyCode::Char('q') {
                        break;
                    }
                    
                    stdout.queue(Print(format!("{:?\n}", event)))?;
                },
                _ => {},
            }
        }
        stdout.queue(Print(format!("loop {}\n", nr)))?;
        stdout.flush()?;
    }

    disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;

    Ok(())
}
