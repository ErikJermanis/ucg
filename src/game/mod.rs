use std::{io::{self, Stdout, Write}, thread, time::{Duration, Instant}};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyEventKind, KeyModifiers},
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    QueueableCommand
};

mod utils;
mod ui;

pub struct Game {
  pub screen_width: u16,
  pub screen_height: u16,
  pub stdout: Stdout,
  pub should_quit: bool,
}

impl Game {
    pub fn new(stdout: Stdout) -> io::Result<Self> {
        let (screen_width, screen_height) = size()?;
        Ok(Game {
            screen_width,
            screen_height,
            stdout,
            should_quit: false,
        })
    }

    pub fn init(&mut self) -> io::Result<()> {
        self.stdout.queue(EnterAlternateScreen)?;
        self.stdout.queue(Clear(ClearType::Purge))?;
        self.stdout.queue(Hide)?;
        self.stdout.flush()?;
        enable_raw_mode()?;
        Ok(())
    }

    pub fn quit(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        self.stdout.queue(LeaveAlternateScreen)?;
        self.stdout.queue(Show)?;
        self.stdout.flush()?;
        Ok(())
    }

    pub fn show_main_menu(&mut self) -> io::Result<()> {
        self.stdout.queue(Clear(ClearType::All))?;
        self.stdout.queue(Clear(ClearType::Purge))?;
        self.draw_main_menu()?;
        self.stdout.flush()?;
        Ok(())
    }

    pub fn start_game_loop(&mut self) -> io::Result<()> {
        self.stdout.queue(Clear(ClearType::All))?;
        self.stdout.queue(Clear(ClearType::Purge))?;
        self.draw_game_controls()?;
        self.stdout.flush()?;

        let mut nr: usize = 0;
        loop {
            if poll(Duration::from_millis(500))? {
                match read()? {
                    Event::Key(event) => {
                        if event.kind == KeyEventKind::Press {
                            match event.code {
                                KeyCode::Char('q') => break,
                                KeyCode::Char(x) => {
                                    if event.modifiers.contains(KeyModifiers::CONTROL) {
                                        match x {
                                            'c' => {
                                                self.should_quit = true;
                                                break;
                                            },
                                            _ => {}
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        if event.code == KeyCode::Char('q') {
                            break;
                        }
                    },
                    _ => {},
                }
            }
            nr += 1;
            self.stdout.queue(MoveTo((self.screen_width / 3) * 2, self.screen_height / 2))?;
            self.stdout.queue(Print(format!("frame no: {}", nr)))?;
            self.stdout.flush()?;
        }
        self.show_main_menu()?;

        Ok(())
    }

    pub fn run_benchmark(&mut self) -> io::Result<()> {
        let start = Instant::now();
        let duration = Duration::from_secs(3);
        let mut n_of_frames: usize = 1;
        self.stdout.queue(Clear(ClearType::All))?;
        self.stdout.queue(Clear(ClearType::Purge))?;
        while Instant::now() - start < duration {
            self.stdout.queue(Clear(ClearType::All))?;
            self.stdout.queue(Clear(ClearType::Purge))?;
            self.draw_game_controls()?;
            self.stdout.flush()?;
            n_of_frames += 1;
            thread::sleep(Duration::from_millis(32));
        }
        self.stdout.queue(MoveTo(self.screen_width / 3, self.screen_height / 2))?;
        self.stdout.queue(Print(format!("Benchmark done: {} frames per second", n_of_frames / 3)))?;
        self.stdout.flush()?;

        Ok(())
    }
}
