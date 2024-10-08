use std::io::{self, stdout};

use crossterm::event::{read, Event, KeyCode, KeyEventKind, KeyModifiers};

mod game;
mod player;
use game::Game;

fn main() -> io::Result<()> {
    let stdout = stdout();

    let mut game = Game::new(stdout)?;

    if !game.check_terminal_size()? {
        return Ok(());
    }

    game.init()?;
    game.show_main_menu()?;

    while game.should_quit == false {
        match read()? {
            Event::Key(event) => {
                if event.kind == KeyEventKind::Press {
                    match event.code {
                        KeyCode::Char('q') => game.should_quit = true,
                        KeyCode::Char('p') => game.start_game_loop()?,
                        KeyCode::Char('b') => game.run_benchmark()?,
                        KeyCode::Char(x) => {
                            if event.modifiers.contains(KeyModifiers::CONTROL) {
                                match x {
                                    'c' => game.should_quit = true,
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

    game.quit()?;

    Ok(())
}
