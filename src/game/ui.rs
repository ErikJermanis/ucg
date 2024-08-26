use std::io;

use crossterm::{cursor::MoveTo, style::{Color, Print, SetForegroundColor}, QueueableCommand};

use super::{utils::{DL_CORNER, DR_CORNER, GAME_TITLE, HORIZONTAL_LINE, L_T, R_T, UL_CORNER, UR_CORNER, VERTICAL_LINE}, Game};

impl Game {
    pub fn draw_main_menu(&mut self) -> io::Result<()> {
        let box_width = 24;
        let box_height = 6;
        let pos_x = self.screen_width / 2 - (box_width / 2);
        let pos_y = self.screen_height / 2 - (box_height / 2);
        
        self.stdout.queue(MoveTo(pos_x, pos_y))?;
        self.stdout.queue(Print(format!("{}{}{}", UL_CORNER, HORIZONTAL_LINE.repeat(box_width as usize - 2), UR_CORNER)))?;
        for i in 1..(box_height - 1) {
            self.stdout.queue(MoveTo(pos_x, pos_y + i))?;
            self.stdout.queue(Print(format!("{}{}{}", VERTICAL_LINE, " ".repeat(box_width as usize - 2), VERTICAL_LINE)))?;
        }
        self.stdout.queue(MoveTo(pos_x, pos_y + (box_height - 1)))?;
        self.stdout.queue(Print(format!("{}{}{}", DL_CORNER, HORIZONTAL_LINE.repeat(box_width as usize - 2), DR_CORNER)))?;

        self.stdout.queue(SetForegroundColor(Color::DarkYellow))?;
        self.stdout.queue(MoveTo(pos_x + 1 + ((box_width - 2) - GAME_TITLE.len() as u16) / 2, pos_y + 1))?;
        self.stdout.queue(Print(GAME_TITLE))?;
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

        Ok(())
    }

    pub fn draw_game_controls(&mut self) -> io::Result<()> {
        for i in 1..self.screen_height - 1 {
            self.stdout.queue(MoveTo(self.screen_width / 3, i))?;
            self.stdout.queue(Print(VERTICAL_LINE))?;
            self.stdout.queue(MoveTo(0, i))?;
            self.stdout.queue(Print(VERTICAL_LINE))?;
        }
        for i in 1..self.screen_width / 3 {
            self.stdout.queue(MoveTo(i, 0))?;
            self.stdout.queue(Print(HORIZONTAL_LINE))?;
            self.stdout.queue(MoveTo(i, self.screen_height - 3))?;
            self.stdout.queue(Print(HORIZONTAL_LINE))?;
            self.stdout.queue(MoveTo(i, self.screen_height))?;
            self.stdout.queue(Print(HORIZONTAL_LINE))?;
        }
        self.stdout.queue(MoveTo(0 , self.screen_height - 3))?;
        self.stdout.queue(Print(L_T))?;
        self.stdout.queue(MoveTo(self.screen_width / 3 , self.screen_height - 3))?;
        self.stdout.queue(Print(R_T))?;
        self.stdout.queue(MoveTo(0, 0))?;
        self.stdout.queue(Print(UL_CORNER))?;
        self.stdout.queue(MoveTo(self.screen_width / 3, 0))?;
        self.stdout.queue(Print(UR_CORNER))?;
        self.stdout.queue(MoveTo(self.screen_width / 3, self.screen_height))?;
        self.stdout.queue(Print(DR_CORNER))?;
        self.stdout.queue(MoveTo(0, self.screen_height))?;
        self.stdout.queue(Print(DL_CORNER))?;
        self.stdout.queue(MoveTo(self.screen_width / 6 - 3, self.screen_height - 2))?;
        self.stdout.queue(Print("Quit"))?;
        self.stdout.queue(MoveTo(self.screen_width / 6 + 2, self.screen_height - 2))?;
        self.stdout.queue(SetForegroundColor(Color::Cyan))?;
        self.stdout.queue(Print("q"))?;
        self.stdout.queue(SetForegroundColor(Color::Reset))?;

        Ok(())
    }

    pub fn draw_level(&mut self) -> io::Result<()> {
        let mut buffer = String::new();
        for (y, row) in self.level.iter().enumerate() {
            buffer.clear();
            buffer = row.iter().collect();
            self.stdout.queue(MoveTo(self.playfield_origin.0, self.playfield_origin.1 + y as u16))?;
            self.stdout.queue(Print(&buffer))?;
        }

        Ok(())
    }

    pub fn draw_player(&mut self) -> io::Result<()> {
        self.stdout.queue(MoveTo(self.player.position.0, self.player.position.1))?;
        self.stdout.queue(Print("P"))?;
        self.stdout.queue(MoveTo(self.player.old_position.0, self.player.old_position.1))?;
        self.stdout.queue(Print(format!("{}", self.level[self.player.old_position.0 as usize][self.player.old_position.1 as usize])))?;

        Ok(())
    }
}