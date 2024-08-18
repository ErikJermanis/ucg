use std::{io::{self, Stdout, Write}, time::Duration};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyEventKind, KeyModifiers},
    style::{Color, Print, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    QueueableCommand
};
use utils::{DL_CORNER, DR_CORNER, HORIZONTAL_LINE, UL_CORNER, UR_CORNER, VERTICAL_LINE};

mod utils;

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
        let box_width = 24;
        let box_height = 6;
        let pos_x = self.screen_width / 2 - (box_width / 2);
        let pos_y = self.screen_height / 2 - (box_height / 2);
        
        self.stdout.queue(Clear(ClearType::Purge))?;
        self.stdout.queue(Clear(ClearType::All))?;
        self.stdout.queue(MoveTo(pos_x, pos_y))?;
        self.stdout.queue(Print(format!("{}{}{}", UL_CORNER, HORIZONTAL_LINE.repeat(box_width as usize - 2), UR_CORNER)))?;
        for i in 1..(box_height - 1) {
            self.stdout.queue(MoveTo(pos_x, pos_y + i))?;
            self.stdout.queue(Print(format!("{}{}{}", VERTICAL_LINE, " ".repeat(box_width as usize - 2), VERTICAL_LINE)))?;
        }
        self.stdout.queue(MoveTo(pos_x, pos_y + (box_height - 1)))?;
        self.stdout.queue(Print(format!("{}{}{}", DL_CORNER, HORIZONTAL_LINE.repeat(box_width as usize - 2), DR_CORNER)))?;

        let title = "Unnamed Cli Game";
        self.stdout.queue(SetForegroundColor(Color::DarkYellow))?;
        self.stdout.queue(MoveTo(pos_x + 1 + ((box_width - 2) - title.len() as u16) / 2, pos_y + 1))?;
        self.stdout.queue(Print(title))?;
        self.stdout.queue(SetForegroundColor(Color::Reset))?;
        self.stdout.queue(MoveTo(pos_x + 1, pos_y + 2))?;
        self.stdout.queue(Print(format!("{}", "-".repeat(box_width as usize - 2))))?;

        self.stdout.queue(MoveTo(pos_x + 1 + ((box_width - 2) - 8) / 2, pos_y + 3))?;
        self.stdout.queue(Print("Play"))?;
        self.stdout.queue(MoveTo(pos_x + 1 + ((box_width - 2) - 8) / 2, pos_y + 4))?;
        self.stdout.queue(Print("Quit"))?;
        self.stdout.queue(SetForegroundColor(Color::Cyan))?;
        self.stdout.queue(MoveTo(pos_x + 1 + ((box_width - 2) - 8) / 2 + 7, pos_y + 3))?;
        self.stdout.queue(Print("p"))?;
        self.stdout.queue(MoveTo(pos_x + 1 + ((box_width - 2) - 8) / 2 + 7, pos_y + 4))?;
        self.stdout.queue(Print("q"))?;
        self.stdout.queue(SetForegroundColor(Color::Reset))?;
        self.stdout.flush()?;

        Ok(())
    }

    pub fn start_game_loop(&mut self) -> io::Result<()> {
        self.stdout.queue(Clear(ClearType::Purge))?;
        self.stdout.queue(Clear(ClearType::All))?;
        for i in 0..self.screen_height {
            self.stdout.queue(MoveTo(self.screen_width / 3, i))?;
            self.stdout.queue(Print(format!("{}", VERTICAL_LINE)))?;
        }
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
}
