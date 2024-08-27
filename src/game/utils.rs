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
pub const WALL_TEXTURE: char = '█';
pub const FLOOR_TEXTURE: char = ' ';
pub const PLAYER_TEXTURE: char = 'O';

pub const GAME_TITLE: &str = "Unnamed Cli Game";

#[derive(Clone)]
pub enum Tile {
    Wall,
    Floor,
}

impl Tile {
    pub fn to_char(&self) -> char {
        match self {
            Tile::Wall => WALL_TEXTURE,
            Tile::Floor => FLOOR_TEXTURE,
        }
    }
}

impl Game {
    pub fn generate_emtpy_level(&mut self) -> io::Result<Vec<Vec<Tile>>> {
        let mut level: Vec<Vec<Tile>> = Vec::new();
        let mut row: Vec<Tile> = Vec::new();
        for _ in 0..self.playfield_size.0 {
            row.push(Tile::Wall);
        }
        level.push(row.clone());
        row.clear();
        for _ in 0..self.playfield_size.1 - 2 {
            row.push(Tile::Wall);
            for _ in 0..self.playfield_size.0 - 2 {
                row.push(Tile::Floor);
            }
            row.push(Tile::Wall);
            level.push(row.clone());
            row.clear();
        }
        for _ in 0..self.playfield_size.0 {
            row.push(Tile::Wall);
        }
        level.push(row.clone());

        Ok(level)
    }

    pub fn get_tile(&self, x: u16, y: u16) -> Option<&Tile> {
        if let Some(row) = self.level.get(y as usize) {
            row.get(x as usize)
        } else {
            None
        }
    }

    pub fn check_collision(&self, x: u16, y: u16) -> bool {
        if let Some(tile) = self.get_tile(x, y) {
            match tile {
                Tile::Wall => true,
                _ => false,
            }
        } else {
            true
        }
    }
}
