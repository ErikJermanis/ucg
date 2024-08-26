use std::io;

use super::Game;

pub const HORIZONTAL_LINE: &str = "━";
pub const VERTICAL_LINE: &str = "┃";
pub const UL_CORNER: &str = "┏";
pub const UR_CORNER: &str = "┓";
pub const DL_CORNER: &str = "┗";
pub const DR_CORNER: &str = "┛";
pub const R_T: &str = "┫";
pub const L_T: &str = "┣";
pub const MIN_TERMINAL_WIDTH: u16 = 138;
pub const MIN_TERMINAL_HEIGHT: u16 = 32;

pub const GAME_TITLE: &str = "Unnamed Cli Game";

impl Game {
    pub fn generate_emtpy_level(&mut self) -> io::Result<Vec<Vec<char>>> {
        let mut level: Vec<Vec<char>> = Vec::new();
        let mut row: Vec<char> = Vec::new();
        for _ in 0..self.playfield_size.0 {
            row.push('#');
        }
        level.push(row.clone());
        row.clear();
        for _ in 0..self.playfield_size.1 - 2 {
            row.push('#');
            for _ in 0..self.playfield_size.0 - 2 {
                row.push(' ');
            }
            row.push('#');
            level.push(row.clone());
            row.clear();
        }
        for _ in 0..self.playfield_size.0 {
            row.push('#');
        }
        level.push(row.clone());

        Ok(level)
    }
}
