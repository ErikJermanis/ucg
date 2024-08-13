use std::io::{self, stdout, Stdout, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event, KeyCode, KeyEventKind, KeyModifiers},
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    QueueableCommand
};

const HORIZONTAL_LINE: &str = "━";
const VERTICAL_LINE: &str = "┃";
const UL_CORNER: &str = "┏";
const UR_CORNER: &str = "┓";
const DL_CORNER: &str = "┗";
const DR_CORNER: &str = "┛";

fn draw_menu_box(stdout: &mut Stdout, screen_width: u16, screen_height: u16) -> io::Result<()> {
    let box_width = 50;
    let box_height = 16;
    let pos_x = screen_width / 2 - (box_width / 2);
    let pos_y = screen_height / 2 - (box_height / 2);

    stdout.queue(MoveTo(pos_x, pos_y))?;
    stdout.queue(Print(format!("{}{}{}", UL_CORNER, HORIZONTAL_LINE.repeat(box_width as usize - 2), UR_CORNER)))?;
    for i in 1..(box_height - 1) {
        stdout.queue(MoveTo(pos_x, pos_y + i))?;
        stdout.queue(Print(format!("{}{}{}", VERTICAL_LINE, " ".repeat(box_width as usize - 2), VERTICAL_LINE)))?;
    }
    stdout.queue(MoveTo(pos_x, pos_y + (box_height - 1)))?;
    stdout.queue(Print(format!("{}{}{}", DL_CORNER, HORIZONTAL_LINE.repeat(box_width as usize - 2), DR_CORNER)))?;

    let title = "Unnamed Cli Game";
    stdout.queue(MoveTo(pos_x + ((box_width - 2) - title.len() as u16) / 2, pos_y + 1))?;
    stdout.queue(Print(title))?;

    Ok(())
}

fn main() -> io::Result<()> {
    let mut stdout = stdout();

    stdout.queue(EnterAlternateScreen)?;
    stdout.queue(Clear(ClearType::Purge))?;
    stdout.queue(Hide)?;
    stdout.flush()?;
    enable_raw_mode()?;

    let (screen_width, screen_height) = size()?;

    draw_menu_box(&mut stdout, screen_width, screen_height)?;

    stdout.flush()?;

    loop {
        match read()? {
            Event::Key(event) => {
                if event.kind == KeyEventKind::Press {
                    match event.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char(x) => {
                            if event.modifiers.contains(KeyModifiers::CONTROL) {
                                match x {
                                    'c' => break,
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            },
            _ => {}
        }
    }

    disable_raw_mode()?;
    stdout.queue(LeaveAlternateScreen)?;
    stdout.queue(Show)?;
    stdout.flush()?;

    Ok(())
}
