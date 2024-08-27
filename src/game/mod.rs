use std::{io::{self, Stdout, Write}, thread, time::{Duration, Instant}};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyEventKind, KeyModifiers},
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    QueueableCommand
};
use utils::Tile;

use crate::player;

mod utils;
mod ui;

pub struct Game {
  pub screen_width: u16,
  pub screen_height: u16,
  pub stdout: Stdout,
  pub should_quit: bool,
  pub playfield_origin: (u16, u16),
  pub playfield_size: (u16, u16),
  pub player: player::Player,
  pub level: Vec<Vec<Tile>>,
}

impl Game {
    pub fn new(stdout: Stdout) -> io::Result<Self> {
        let (screen_width, screen_height) = size()?;
        let playfield_size: (u16, u16) = (60, 25);
        let playfield_origin_x: u16 = (screen_width - screen_width / 3 - playfield_size.0) / 2 + screen_width / 3;
        let playfield_origin_y: u16 = screen_height / 2 - playfield_size.1 / 2;
        let player = player::Player::new();
        Ok(Game {
            screen_width,
            screen_height,
            stdout,
            should_quit: false,
            playfield_origin: (playfield_origin_x, playfield_origin_y),
            playfield_size,
            player,
            level: Vec::new(),
        })
    }

    pub fn check_terminal_size(&mut self) -> io::Result<bool> {
        if self.screen_width < utils::MIN_TERMINAL_WIDTH || self.screen_height < utils::MIN_TERMINAL_HEIGHT {
            self.stdout.queue(Print(format!("Please resize terminal window to at least 138x32. Current size: {}x{}\n", self.screen_width, self.screen_height)))?;
            self.stdout.flush()?;
            return Ok(false);
        }
        
        Ok(true)
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
        self.level = self.generate_emtpy_level()?;
        self.draw_level()?;
        self.stdout.flush()?;

        loop {
            self.draw_player()?;
            self.stdout.flush()?;
            if poll(Duration::from_millis(500))? {
                match read()? {
                    Event::Key(event) => {
                        if event.kind == KeyEventKind::Press {
                            match event.code {
                                KeyCode::Char('q') => break,
                                KeyCode::Down => {
                                    if !self.check_collision(self.player.position.0, self.player.position.1 + 1) {
                                        self.player.move_player(player::Direction::Down);
                                    }
                                },
                                KeyCode::Up => {
                                    if !self.check_collision(self.player.position.0, self.player.position.1 - 1) {
                                        self.player.move_player(player::Direction::Up);
                                    }
                                },
                                KeyCode::Left => {
                                    if !self.check_collision(self.player.position.0 - 1, self.player.position.1) {
                                        self.player.move_player(player::Direction::Left);
                                    }
                                },
                                KeyCode::Right => {
                                    if !self.check_collision(self.player.position.0 + 1, self.player.position.1) {
                                        self.player.move_player(player::Direction::Right);
                                    }
                                },
                                KeyCode::Char('j') => {
                                    if !self.check_collision(self.player.position.0, self.player.position.1 + 1) {
                                        self.player.move_player(player::Direction::Down);
                                    }
                                },
                                KeyCode::Char('k') => {
                                    if !self.check_collision(self.player.position.0, self.player.position.1 - 1) {
                                        self.player.move_player(player::Direction::Up);
                                    }
                                },
                                KeyCode::Char('h') => {
                                    if !self.check_collision(self.player.position.0 - 1, self.player.position.1) {
                                        self.player.move_player(player::Direction::Left);
                                    }
                                },
                                KeyCode::Char('l') => {
                                    if !self.check_collision(self.player.position.0 + 1, self.player.position.1) {
                                        self.player.move_player(player::Direction::Right);
                                    }
                                },
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
